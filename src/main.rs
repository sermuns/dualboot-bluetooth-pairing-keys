use color_eyre::eyre::{OptionExt, bail};
use nt_hive::{Hive, KeyValueData};
use std::{env, fs::File, io::Read};
use uuid::Uuid;

fn main() -> color_eyre::Result<()> {
    let Some(filename) = env::args().nth(1) else {
        println!("Usage: readhive <FILENAME>");
        return Ok(());
    };
    println!();

    let mut buf = Vec::new();
    File::open(filename)?.read_to_end(&mut buf)?;

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
        println!("{}: {:X}", name, link_key.simple());
    }

    Ok(())
}
