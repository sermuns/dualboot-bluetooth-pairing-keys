use clap::Parser;
use color_eyre::eyre::{OptionExt, bail};
use nt_hive::{Hive, KeyValueData};
use std::{fs::File, io::Read, path::PathBuf};
use uuid::Uuid;

#[derive(Parser)]
struct Args {
    /// Where the Windows partition is mounted (e.g. /mnt/windows)
    mountpoint: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let Args { mountpoint } = Args::parse();

    let hive_file = mountpoint.join("Windows/System32/config/SYSTEM");
    let mut buf = Vec::new();
    File::open(hive_file)?.read_to_end(&mut buf)?;
    let hive = Hive::new(buf.as_ref())?;

    let root_key = hive.root_key_node()?;

    let adapters = root_key
        .subpath(r#"ControlSet001\Services\BTHPORT\Parameters\Keys"#)
        .ok_or_eyre("Failed to find BTHPORT keys")??;

    // // FIXME: support more
    let adapter = adapters
        .subkeys()
        .unwrap()?
        .next()
        .ok_or_eyre("No Bluetooth adapters found")??;

    for value in adapter.values().ok_or_eyre("no values")?? {
        let key = value?;
        let name = key.name()?;
        if name == "CentralIRK" {
            continue;
        }
        let KeyValueData::Small(link_key_bytes) = key.data()? else {
            bail!("Expected small data");
        };
        let link_key = Uuid::from_slice(link_key_bytes)?;
        println!(
            "{}: {:X}",
            name.to_string().to_uppercase(),
            link_key.simple()
        );
    }

    Ok(())
}
