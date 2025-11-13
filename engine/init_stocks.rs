use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File};
use bincode::serialize_into;

#[derive(Serialize, Deserialize, Debug)]
struct PricePoint 
{
    timestamp: u64,
    price: f32,
}

fn read_csv(path:&str) -> Result<Vec<PricePoint>, Box<dyn Error>>
{
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: PricePoint = result?;
        records.push(record);
    }
    Ok(records)
}

fn write_bincode(path: &str, data: &[PricePoint]) -> Result<(), Box <dyn Error>> 
{
    let mut file = File::create(path)?;
    serialize_into(&mut file, data)?;
    Ok(())
}

fn write_csv_to_bin(ipath: &str, opath: &str) -> Result<(), Box <dyn Error>>
{
    let data = read_csv(ipath)?;
    println!("Loaded {} records from {}...", data.len(), ipath);
    write_bincode(opath, &data)?;
    println!("Saved records to {}...", opath);
    Ok(())
}