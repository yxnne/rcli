use anyhow::Result;
use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    // ? 相当于做了match
    // match reader {
    //     Ok(v) => ...
    //     Err(e) => return Err(e.into()),
    // }

    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let json = serde_json::to_string_pretty(&ret)?;

    fs::write(output, json)?;

    Ok(())
}
