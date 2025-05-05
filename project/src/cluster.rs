use crate::read_csv::DataFrame;
use crate::read_csv::ColumnVal;
use plotters::prelude::*;
use std::error::Error;
use std::path::Path;

pub fn plot_scatter<P: AsRef<Path>>(df: &DataFrame, x_label: &str,y_label: &str,output_path: P,) -> Result<(), Box<dyn Error>> {
    let x_idx = df.col_labels.iter().position(|l| l == x_label)
        .ok_or_else(|| format!("Column '{}' not found", x_label))?;

    let y_idx = df.col_labels.iter().position(|l| l == y_label)
        .ok_or_else(|| format!("Column '{}' not found", y_label))?;

    let x_col = &df.columns[x_idx];
    let y_col = &df.columns[y_idx];

    if x_col.len() != y_col.len() {
        return Err("X and Y columns have different lengths".into());
    }

    let mut points = Vec::new();
    for (x_val, y_val) in x_col.iter().zip(y_col.iter()) {
        if let (ColumnVal::Three(xf), ColumnVal::Three(yf)) = (x_val, y_val) {
            points.push((*xf, *yf));
        }
    }

    if points.is_empty() {
        return Err("No numeric data to plot".into());
    }

    let root = BitMapBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min)
        ..points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_range = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min)
        ..points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    chart.draw_series(points.iter().map(|(x, y)| {
        Circle::new((*x, *y), 5, BLUE.filled())
    }))?;

    root.present()?;
    Ok(())
}