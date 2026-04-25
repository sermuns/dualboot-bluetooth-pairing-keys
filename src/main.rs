use clap::Parser;
use color_eyre::{
    Section,
    eyre::{Context, OptionExt, bail, ensure, eyre},
};
use nt_hive::{Hive, KeyValueData};
use std::{fs::File, io::Read, path::PathBuf};
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Args {
    /// Where the Windows partition is mounted (e.g. /mnt/windows)
    mountpoint: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(false)
        .install()?;

    let Args { mountpoint } = Args::parse();

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

        let adapter_name_with_colons = hex_string_with_colons(&adapter.name()?.to_string());

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
            adapter_name_with_colons,
            num_devices,
            if num_devices == 1 { "" } else { "s" }
        );

        for device_key in adapter_values {
            let device_name_with_colons = hex_string_with_colons(&device_key.name()?.to_string());
            let KeyValueData::Small(link_key_bytes) = device_key.data()? else {
                bail!("Expected small data");
            };
            let link_key = Uuid::from_slice(link_key_bytes)?;
            println!(
                "- {} has link key {:X}",
                device_name_with_colons,
                link_key.simple()
            );
        }
    }

    Ok(())
}

fn hex_string_with_colons(s: &str) -> String {
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
