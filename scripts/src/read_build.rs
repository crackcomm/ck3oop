use anyhow::Result;
use ck3oop_scripts::build;

fn main() -> Result<()> {
    let json = build::Build::load()?.to_json()?;
    Ok(println!("{}", json))
}
