use clap::Parser;
use color_eyre::{
    Section,
    eyre::{Context, OptionExt, bail, ensure, eyre},
};
use nt_hive::{Hive, KeyValueData};
use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    process::Command,
};
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Args {
    /// Where the Windows partition is mounted (e.g. /mnt/windows)
    mountpoint: PathBuf,

    /// Attempt to write link keys to /var/lib/bluetooth/..
    #[arg(short, long)]
    write: bool,

    /// Attempt to restart systemd unit `bluetooth` after writing
    /// Only valid if `--write` is also set!
    #[arg(short, long, requires = "write")]
    restart_bluetooth: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(false)
        .install()?;

    let Args {
        mountpoint,
        write,
        restart_bluetooth,
    } = Args::parse();

    let windows_dir_path = mountpoint.join("Windows");
    ensure!(
        windows_dir_path.exists(),
        eyre!("Given mountpoint does not contain a 'Windows' directory").suggestion(format!(
            "Ensure you mounted the correct Windows partition at given path '{}'",
            mountpoint.display()
        ))
    );
    let hive_path = windows_dir_path.join("System32/config/SYSTEM");
    let mut buf = Vec::new();
    File::open(&hive_path)
        .with_context(|| format!("Failed to open hive file at '{}'", hive_path.display()))?
        .read_to_end(&mut buf)?;
    let hive = Hive::new(buf.as_ref())?;

    let root_key = hive.root_key_node()?;

    let adapters_key = root_key
        .subpath(r#"ControlSet001\Services\BTHPORT\Parameters\Keys"#)
        .ok_or_eyre("Failed to find BTHPORT keys")??;

    for adapter in adapters_key.subkeys().ok_or_eyre("No adapters")?? {
        let adapter = adapter?;

        let adapter_name_with_colons = create_hex_string_with_colons(&adapter.name()?.to_string());

        let adapter_values: Vec<_> = adapter
            .values()
            .into_iter()
            .filter_map(|v| v.ok())
            .flatten()
            .filter_map(|v| v.ok())
            .filter(|v| v.name().is_ok_and(|n| n != "CentralIrk"))
            .collect();

        let num_devices = adapter_values.len();
        println!(
            "Adapter with ID {} has {} device{}:",
            &adapter_name_with_colons,
            num_devices,
            if num_devices == 1 { "" } else { "s" }
        );

        for device_key in adapter_values {
            let device_name_with_colons =
                create_hex_string_with_colons(&device_key.name()?.to_string());
            let KeyValueData::Small(link_key_bytes) = device_key.data()? else {
                bail!("Expected small data");
            };
            let link_key = Uuid::from_slice(link_key_bytes)?;
            println!(
                "- {} has link key {:X}",
                device_name_with_colons,
                link_key.simple()
            );

            if !write {
                continue;
            }

            let adapter_dir = Path::new("/var/lib/bluetooth/").join(&adapter_name_with_colons);
            if let Err(e) = fs::create_dir(&adapter_dir)
                && e.kind() != io::ErrorKind::AlreadyExists
            {
                return Err(eyre!(e)
                    .wrap_err(format!("Failed to create '{}'", adapter_dir.display()))
                    .suggestion("Ensure you ran the program as superuser"));
            }

            let device_dir = adapter_dir.join(&device_name_with_colons);
            if let Err(e) = fs::create_dir(&device_dir)
                && e.kind() != io::ErrorKind::AlreadyExists
            {
                return Err(eyre!(e)
                    .wrap_err(format!("Failed to create '{}'", device_dir.display()))
                    .suggestion("Ensure you ran the program as superuser"));
            }

            let device_info_path = device_dir.join("info");

            let device_info_contents = match fs::read_to_string(&device_info_path) {
                Ok(contents) => {
                    let mut found = false;
                    // FIXME: shitty alloc
                    let mut lines: Vec<_> = contents
                        .lines()
                        .map(|line| {
                            if !found && line.starts_with("Key=") {
                                found = true;
                                format!("Key={:X}", link_key.simple())
                            } else {
                                line.to_string()
                            }
                        })
                        .collect();

                    if !found {
                        lines.push(format!("[LinkKey]\nKey={:X}", link_key.simple()));
                    }

                    lines.join("\n")
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    format!("[LinkKey]\nKey={:X}\n", link_key.simple())
                }
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                    return Err(eyre!(e)
                        .wrap_err(format!(
                            "Failed to read device info file for '{}'",
                            device_name_with_colons
                        ))
                        .suggestion("Ensure you ran the program as superuser"));
                }

                Err(e) => {
                    return Err(eyre!(e).wrap_err(format!(
                        "Failed to read device info file for '{}'",
                        device_name_with_colons
                    )));
                }
            };

            fs::write(&device_info_path, device_info_contents).with_context(|| {
                format!(
                    "Failed to write device info file for '{}'",
                    device_name_with_colons
                )
            })?;
        }
    }

    if restart_bluetooth {
        println!("\nSuccessfully wrote link keys to /var/lib/bluetooth/");
        Command::new("systemctl")
            .args(["restart", "bluetooth"])
            .status()?;
        println!("\nRestarted systemd unit 'bluetooth'");
    }

    Ok(())
}

fn create_hex_string_with_colons(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + s.len() / 2 - 1);

    for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
        if i > 0 {
            out.push(':');
        }
        for &b in chunk {
            out.push((b as char).to_ascii_uppercase());
        }
    }

    out
}
