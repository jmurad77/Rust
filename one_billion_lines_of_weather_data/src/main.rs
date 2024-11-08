use std::error::Error;
use csv::Reader;
use std::collections::HashMap;

fn read_weather_data() -> Result<(), Box<dyn Error>> {
    let mut weather_data = HashMap::new();
    let mut rdr = Reader::from_path("weather_stations_10Mb.csv")?;
    for result in rdr.records() {
        if ('#' != (result?).get(0)) {
            let mut values: [u64; 4]
            weather_data
        }
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_weather_data() {
        println!("error reading csv file: {}", err);
    }
}
