// src/visualize.rs

use crate::data::PollutionRecord;
use plotters::prelude::*;
use std::collections::HashMap;
use std::error::Error;

/// Draws a matrix heatmap of pollution data where:
/// X-axis = Place abbreviation, Y-axis = Year, Color = Average pollution level
pub fn draw_matrix_heatmap(records: &[PollutionRecord], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut grouped: HashMap<(String, u32), Vec<f64>> = HashMap::new();

    for r in records {
        grouped.entry((r.place_abbreviation.clone(), r.year))
            .or_default()
            .push(r.pollution_score);
    }

    let mut summary: Vec<((String, u32), f64)> = grouped
        .into_iter()
        .map(|((place, year), values)| {
            let avg = values.iter().sum::<f64>() / values.len() as f64;
            ((place, year), avg)
        })
        .collect();

    summary.sort_by_key(|((place, year), _)| (place.clone(), *year));

    let mut places: Vec<String> = summary.iter().map(|((p, _), _)| p.clone()).collect();
    let mut years: Vec<u32> = summary.iter().map(|((_, y), _)| *y).collect();
    places.sort(); places.dedup();
    years.sort(); years.dedup();

    let max_score = summary.iter().map(|(_, val)| *val).fold(f64::MIN, f64::max);

    let label_count = std::cmp::min(places.len(), 100);
    let canvas_width: u32 = std::cmp::max((places.len() as u32) * 40, 1600);
    let canvas_height: u32 = 1000;

    let root = BitMapBackend::new(filename, (canvas_width, canvas_height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Air Pollution Heatmap (Place × Year)", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(200)
        .y_label_area_size(80)
        .build_cartesian_2d(0..places.len(), *years.first().unwrap()..(*years.last().unwrap() + 1))?;

    chart.configure_mesh()
        .x_labels(label_count)
        .x_label_formatter(&|x| {
            places.get(*x).cloned().unwrap_or_default()
        })
        .x_label_style(("sans-serif", 12).into_font().transform(FontTransform::Rotate90))
        .x_label_offset(40)
        .y_desc("Year")
        .x_desc("Place")
        .label_style(("sans-serif", 12))
        .axis_desc_style(("sans-serif", 18))
        .draw()?;

    chart.draw_series(
        summary.iter().filter_map(|((place, year), score)| {
            let x = places.iter().position(|p| p == place)?;
            let hue = (*score / max_score).min(1.0) * 0.3;
            let color = HSLColor(hue, 1.0, 0.5);
            Some(Rectangle::new([(x, *year), (x + 1, *year + 1)], color.filled()))
        })
    )?;

    Ok(())
}

/// Draws a bar chart comparing top 3 cleanest and dirtiest places by average pollution.
pub fn draw_top_clean_and_dirty_bar_chart(
    records: &[PollutionRecord],
    filename: &str,
    abbrev_names: &HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
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

    let top_clean: Vec<_> = averages.iter().take(3).cloned().collect();
    let top_dirty: Vec<_> = averages.iter().rev().take(3).cloned().collect();
    let all = [top_clean, top_dirty].concat();

    let root = BitMapBackend::new(filename, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_score = all.iter().map(|(_, score)| *score).fold(f64::MIN, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Top 3 Cleanest and Dirtiest Areas", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..(max_score * 1.2), 0..all.len())?;

    chart.configure_mesh()
        .y_labels(all.len())
        .y_label_formatter(&|i| {
            all.get(*i)
                .map(|(place, _)| abbrev_names.get(place).cloned().unwrap_or_else(|| place.clone()))
                .unwrap_or_default()
        })
        .x_desc("Average Pollution (ppb)")
        .y_desc("Neighborhood")
        .draw()?;

    chart.draw_series(
        all.iter().enumerate().map(|(i, (_, score))| {
            let color = if i < 3 { GREEN.filled() } else { RED.filled() };
            Rectangle::new([(0.0, i), (*score, i + 1)], color)
        })
    )?;

    Ok(())
}
