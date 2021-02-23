extern crate csv;
use crate::types::{
  SpotifyChartCsvRecord,
  SOD,
  get_default_sod
};

fn map_csv_str_to_records(data: String) -> Vec<SpotifyChartCsvRecord> {
  let mut reader = csv::Reader::from_reader(data.as_bytes());
  reader
    .records()
    .map(|record| match record {
    Ok(record) => {
      let deserialized = SpotifyChartCsvRecord::from(record);
      Some(deserialized)
    },
    Err(err) => {
      println!("Error: Error deserializing record {:?}", err);
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
      Some(map_csv_str_to_records(data))
    },
    Err(err) => {
      eprintln!("Error: Error converting http text/csv to string {}", err);
      None
    }
  }  
}

pub fn record_to_sod(record: Option<&SpotifyChartCsvRecord>) -> SOD {
  match record {
    Some(rec) => {
      let url = rec.clone().url;
      fn transform(data: String) -> Option<String> {
        match data.is_empty() {
          true => None,
          false => Some(data)
        }
      }
      SOD {
        artist_name: transform(rec.artist.to_string()),
        track_name: transform(rec.track_name.to_string()),
        uri: transform(rec.url.to_string()),
        source: get_source_from_uri(transform(url))
      }
    },
    None => {
      get_default_sod()
    }
  }
}

pub fn get_source_from_uri(uri: Option<String>) -> Option<String> {
  match uri {
    Some(u) => {
      let spotify_uri = "open.spotify.com".to_owned();
      let youtube_uri = "youtube.com".to_owned();
      if u.contains(&spotify_uri) {
        Some("spotify".to_owned())
      } else if u.contains(&youtube_uri) {
        Some("youtube".to_owned())
      } else {
        None
      }
    },
    None => None
  }
}
