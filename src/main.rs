use chrono::NaiveTime;
use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct RowData {
    time: String,
    ram: f64,
    cpu: f64,
    gpu: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_csv_file("table.csv")?;
    create_chart(&data, "table.png")?;
    Ok(())
}

fn read_csv_file(file_path: &str) -> Result<Vec<RowData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    let mut data = Vec::new();
    for result in reader.records() {
        let record = result?;
        let row_data = RowData {
            time: record[0].to_string(),
            ram: record[1].parse()?,
            cpu: record[2].parse()?,
            gpu: record[3].parse()?,
        };
        data.push(row_data);
    }

    Ok(data)
}

fn create_chart(data: &[RowData], output_path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (1200, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_values: Vec<u32> = (0..data.len() as u32).collect();

    let time_values: Vec<String> = data
        .iter()
        .map(|row: &RowData| {
            NaiveTime::parse_from_str(&row.time, "%H:%M:%S%.3f")
                .unwrap()
                .to_string()
        })
        .collect();

    let start_time = NaiveTime::parse_from_str(&time_values[0], "%H:%M:%S%.3f").unwrap();
    let x_labels: Vec<String> = time_values
        .iter()
        .map(|time_str| {
            let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S%.3f").unwrap();
            let duration = time.signed_duration_since(start_time);
            let seconds = duration.num_seconds();
            format!("{}s", seconds)
        })
        .collect();

    let ram_values: Vec<f64> = data.iter().map(|row| row.ram).collect();
    let cpu_values: Vec<f64> = data.iter().map(|row| row.cpu).collect();
    let gpu_values: Vec<f64> = data.iter().map(|row| row.gpu).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("System Usage", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data.len() as u32, 0.0..100.0)?;

    chart
        .configure_mesh()
        .y_desc("Usage (%)")
        .y_labels(10)
        .x_desc("Time")
        .x_label_style(("sans-serif", 12).into_font().color(&BLACK))
        .x_label_formatter(&|x| x_labels[*x as usize].clone())
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            x_values
                .iter()
                .zip(ram_values.iter())
                .map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("RAM")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            x_values
                .iter()
                .zip(cpu_values.iter())
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<_>>(),
            &RED,
        ))?
        .label("CPU")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            x_values
                .iter()
                .zip(gpu_values.iter())
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<_>>(),
            &GREEN,
        ))?
        .label("GPU")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
