//Entry point that loads data, runs analysis, and generates visual output for air quality insights.

mod data;
mod analyze;
mod visualize;

use data::load_pollution_data;
use analyze::{analyze_air_quality, print_top_clean_and_dirty};
use visualize::{draw_matrix_heatmap, draw_top_clean_and_dirty_bar_chart};

/// Main entry point of the application.
///
///  Workflow
/// 1. Load cleaned pollution data from CSV
/// 2. Analyze best and worst locations
/// 3. Print top 3 cleanest and most polluted neighborhoods (with full names)
/// 4. Generate:
///     - Matrix-style heatmap PNG
///     - Horizontal bar chart PNG
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load air quality data and abbreviation-to-name map
    let (records, abbrev_names) = load_pollution_data("Air_Quality.csv")?;

    // Analyze for cleanest and dirtiest area
    let summary = analyze_air_quality(&records);

    // Print full place names for best/worst areas
    if let Some(name) = abbrev_names.get(&summary.best_place) {
        println!("Best Air Quality: {} ({})", summary.best_place, name);
    }
    if let Some(name) = abbrev_names.get(&summary.worst_place) {
        println!("Worst Air Quality: {} ({})", summary.worst_place, name);
    }

    // Print full name summaries in ranked order
    print_top_clean_and_dirty(&records, &abbrev_names);

    // Generate grid-style heatmap chart
    draw_matrix_heatmap(&records, "air_quality_matrix.png")?;

    // Generate clean/dirty bar chart
    draw_top_clean_and_dirty_bar_chart(&records, "top_pollution_chart.png", &abbrev_names)?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::data::PollutionRecord;
    use crate::analyze::{analyze_air_quality, print_top_clean_and_dirty};
    use std::collections::HashMap;

    /// Unit test for the analyze_air_quality function.
    ///
    /// Purpose
    /// Verifies that the function correctly identifies the cleanest and dirtiest locations
    /// by computing average pollution scores.
    ///
    /// What It Does
    /// - Creates mock data with two locations
    /// - A1 has lower average pollution
    /// - A2 has higher average pollution
    /// - Confirms that the summary result matches expectations
    #[test]
    fn test_analyze_air_quality_identifies_extremes() {
        let records = vec![
            PollutionRecord { place_abbreviation: "A1".to_string(), year: 2020, pollution_score: 10.0 },
            PollutionRecord { place_abbreviation: "A2".to_string(), year: 2020, pollution_score: 50.0 },
            PollutionRecord { place_abbreviation: "A1".to_string(), year: 2021, pollution_score: 15.0 },
            PollutionRecord { place_abbreviation: "A2".to_string(), year: 2021, pollution_score: 55.0 },
        ];

        let result = analyze_air_quality(&records);
        assert_eq!(result.best_place, "A1");
        assert_eq!(result.worst_place, "A2");
    }

    /// Smoke test for the print_top_clean_and_dirty function.
    ///
    ///
    /// Ensures that the function runs without panicking or crashing when printing top results.
    /// (Does not capture printed output; just checks the function executes successfully.)
    ///
    ///
    /// - Creates mock data for 5 locations with varying pollution
    /// - Passes in a dummy map of abbreviations to full names
    /// - Confirms that the function runs without error
    #[test]
    fn test_print_top_clean_and_dirty_runs_without_panic() {
        let records = vec![
            PollutionRecord { place_abbreviation: "A1".to_string(), year: 2020, pollution_score: 5.0 },
            PollutionRecord { place_abbreviation: "A2".to_string(), year: 2020, pollution_score: 15.0 },
            PollutionRecord { place_abbreviation: "A3".to_string(), year: 2020, pollution_score: 25.0 },
            PollutionRecord { place_abbreviation: "A4".to_string(), year: 2020, pollution_score: 35.0 },
            PollutionRecord { place_abbreviation: "A5".to_string(), year: 2020, pollution_score: 45.0 },
        ];

        let mut abbrev_names = HashMap::new();
        abbrev_names.insert("A1".to_string(), "Place 1".to_string());
        abbrev_names.insert("A2".to_string(), "Place 2".to_string());
        abbrev_names.insert("A3".to_string(), "Place 3".to_string());
        abbrev_names.insert("A4".to_string(), "Place 4".to_string());
        abbrev_names.insert("A5".to_string(), "Place 5".to_string());

        print_top_clean_and_dirty(&records, &abbrev_names);
    }
}
