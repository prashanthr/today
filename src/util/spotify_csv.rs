extern crate csv;
use crate::types::{
  SpotifyChartCsvRecord
};

fn map_csv_str_to_records(data: String) -> Vec<SpotifyChartCsvRecord> {
  let mut reader = csv::Reader::from_reader(data.as_bytes());
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
}

pub async fn response_to_records(data: reqwest::Response) -> Option<Vec<SpotifyChartCsvRecord>> {
  match data.text().await {
    Ok(data) =>  {
      println!("Success converting http text/csv data to str");
      println!("Data: {:?}", data);
      Some(map_csv_str_to_records(data))
    },
    Err(err) => {
      eprintln!("Error: Error converting http text/csv to string {}", err);
      None
    }
  }  
}
