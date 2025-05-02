// Calculate top-performing and bottom-performing places based on average air quality levels.

use crate::data::PollutionRecord;
use std::collections::HashMap;

/// Struct: AirQualitySummary
/// Holds the cleanest and most polluted locations based on average pollution across time.
pub struct AirQualitySummary {
    /// Abbreviation of place with lowest average pollution
    pub best_place: String,
    /// Abbreviation of place with highest average pollution
    pub worst_place: String,
}

/// Analyzes average air quality per neighborhood and returns the best and worst locations.
///
/// - records: list of cleaned pollution records
///
/// - Struct containing best and worst location abbreviations
///
/// - Sums scores and counts records by place
/// - Calculates averages and compares them
pub fn analyze_air_quality(records: &[PollutionRecord]) -> AirQualitySummary {
    let mut place_totals = HashMap::new();
    let mut place_counts = HashMap::new();

    for record in records {
        *place_totals.entry(&record.place_abbreviation).or_insert(0.0) += record.pollution_score;
        *place_counts.entry(&record.place_abbreviation).or_insert(0) += 1;
    }

    let mut best_place = "";
    let mut worst_place = "";
    let mut best_score = f64::MAX;
    let mut worst_score = f64::MIN;

    // Calculate average pollution and track min/max
    for (&place, &total) in &place_totals {
        if let Some(&count) = place_counts.get(place) {
            let avg = total / count as f64;
            if avg < best_score {
                best_score = avg;
                best_place = place;
            }
            if avg > worst_score {
                worst_score = avg;
                worst_place = place;
            }
        }
    }

    AirQualitySummary {
        best_place: best_place.to_string(),
        worst_place: worst_place.to_string(),
    }
}

/// Prints the top 3 cleanest and top 3 dirtiest locations by average pollution.
///
/// - records: all pollution records to be ranked
/// - abrev_names: map from abbreviation to full place name
///
/// - Aggregate pollution values by place
/// - Sort by average
/// - Print ranked output
pub fn print_top_clean_and_dirty(
    records: &[PollutionRecord],
    abbrev_names: &HashMap<String, String>
) {
    let mut totals: HashMap<String, f64> = HashMap::new();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for r in records {
        *totals.entry(r.place_abbreviation.clone()).or_insert(0.0) += r.pollution_score;
        *counts.entry(r.place_abbreviation.clone()).or_insert(0) += 1;
    }

    let mut averages: Vec<(String, f64)> = totals.iter()
        .filter_map(|(place, &total)| {
            counts.get(place).map(|&count| (place.clone(), total / count as f64))
        })
        .collect();

    averages.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nTop 3 Cleanest Areas:");
    for (i, (place, avg)) in averages.iter().take(3).enumerate() {
        let full_name = abbrev_names.get(place).unwrap_or(place);
        println!("{}. {} - {:.2} ppb", i + 1, full_name, avg);
    }

    println!("\nTop 3 Most Polluted Areas:");
    for (i, (place, avg)) in averages.iter().rev().take(3).enumerate() {
        let full_name = abbrev_names.get(place).unwrap_or(place);
        println!("{}. {} - {:.2} ppb", i + 1, full_name, avg);
    }
}
