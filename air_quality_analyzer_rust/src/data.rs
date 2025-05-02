//Load, clean, and standardize raw air pollution data from a CSV file into structured records.

use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

/// Struct: PollutionRecord
/// Represents one cleaned data row containing only the fields needed for analysis and visualization.
#[derive(Debug, Clone)]
pub struct PollutionRecord {
    /// Abbreviated code for place name (e.g., A7 to  Upper West Side)
    pub place_abbreviation: String,
    /// Year extracted from Time Period (used for timeline-based plotting)
    pub year: u32,
    /// Pollution score in parts per billion (ppb)
    pub pollution_score: f64,
}

/// Loads and transforms the CSV file data.
///
/// - filepath: path to the CSV data file
///
/// - A tuple:
///     - Vec<PollutionRecord>: cleaned records used for plotting
///     - HashMap<String, String>: abbreviation to full place name mapping for labels

/// - Reads raw CSV rows with many columns
/// - Selects and renames only relevant fields
/// - Assigns abbreviations to each unique place
/// - Extracts the start year from the time period string
pub fn load_pollution_data(filepath: &str) -> Result<(Vec<PollutionRecord>, HashMap<String, String>), Box<dyn Error>> {
    #[derive(Debug, Deserialize)]
    struct RawRecord {
        #[serde(rename = "Geo Place Name")] place: String,
        #[serde(rename = "Time Period")] time_period: String,
        #[serde(rename = "Data Value")] pollution_score: f64,
    }

    let mut rdr = Reader::from_reader(BufReader::new(File::open(filepath)?));
    let mut records = Vec::new();
    let mut abbrev_map: HashMap<String, String> = HashMap::new();
    let mut abbrev_id = 0;

    for result in rdr.deserialize() {
        let raw: RawRecord = result?;

        // Assign abbreviation if first time seeing the full place name
        let abbr = abbrev_map.entry(raw.place.clone()).or_insert_with(|| {
            let id = abbrev_id;
            abbrev_id += 1;
            format!("A{}", id)
        }).clone();

        // Extract the starting year from time range string (e.g., "Winter 2014-15" to 2014)
        let year = raw.time_period
            .split_whitespace()
            .last()
            .unwrap_or("0")
            .split('-')
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);

        records.push(PollutionRecord {
            place_abbreviation: abbr,
            year,
            pollution_score: raw.pollution_score,
        });
    }

    let abbr_to_full: HashMap<String, String> =
        abbrev_map.into_iter().map(|(place, abbr)| (abbr, place)).collect();

    println!("\nAbbreviation Legend:");
    for (abbr, name) in &abbr_to_full {
        println!("{} → {}", abbr, name);
    }

    Ok((records, abbr_to_full))
}
