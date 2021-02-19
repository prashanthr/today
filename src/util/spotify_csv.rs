extern crate csv;
use crate::types::{
  SpotifyChartCsvRecord
};

pub fn map_spotify_data_csv(data: String) -> Vec<SpotifyChartCsvRecord> {
  let mut reader = csv::Reader::from_reader(data.as_bytes());
  // Some(
    reader
      .records()
      .map(|record| match record {
      Ok(record) => {
        println!("Original Record{:?}", record);
        let deserialized = SpotifyChartCsvRecord::from(record);
        println!("Deserialized Record: {:?}", deserialized);
        Some(deserialized)
      },
      Err(err) => {
        println!("Error deserializing record {:?}", err);
        None
      }
    })
    .filter_map(|i| i) // remove empty
    .collect()
  // )          
}
