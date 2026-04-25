use clap::Parser;
use color_eyre::eyre::{Context, OptionExt, bail};
use nt_hive::{Hive, KeyValueData};
use std::{fs::File, io::Read, path::PathBuf};
use uuid::Uuid;

#[derive(Parser)]
struct Args {
    /// Where the Windows partition is mounted (e.g. /mnt/windows)
    mountpoint: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()?;

    let Args { mountpoint } = Args::parse();

    let hive_path = mountpoint.join("Windows/System32/config/SYSTEM");
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
        let adapter_name_string = adapter.name()?.to_string();
        let adapter_values: Vec<_> = adapter
            .values()
            .into_iter()
            .filter_map(|v| v.ok())
            .flatten()
            .filter_map(|v| v.ok())
            .filter(|v| v.name().is_ok_and(|n| n != "CentralIrk"))
            .collect();

        let num_adapters = adapter_values.len();
        println!(
            "Adapter with ID {} has {} device{}:",
            adapter_name_string.to_uppercase(),
            num_adapters,
            if num_adapters == 1 { "" } else { "s" }
        );

        for device_key in adapter_values {
            let device_id = device_key.name()?;
            let KeyValueData::Small(link_key_bytes) = device_key.data()? else {
                bail!("Expected small data");
            };
            let link_key = Uuid::from_slice(link_key_bytes)?;
            println!(
                "- ID: {}, Link key: {:X}",
                device_id.to_string().to_uppercase(),
                link_key.simple()
            );
        }
    }

    Ok(())
}
