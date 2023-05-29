use chrono::NaiveTime;
use csv::Reader;
use encoding_rs::{UTF_8, WINDOWS_1252};
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

struct RowData {
    time: String,
    ram: f64,
    cpu: f64,
    gpu: f64,
}

struct TableIndex {
    time: usize,
    ram: usize,
    cpu: usize,
    gpu: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Based on sensors logging CSV export of HWiNFO64 v7.40-5000
    convert_to_utf8("table.csv")?;
    let data = read_csv_file(
        "table.csv",
        TableIndex {
            time: 1,
            ram: 7,
            cpu: 42,
            gpu: 213,
        },
    )?;
    create_chart(&data, "table.png")?;
    Ok(())
}

// Function to convert all non-UTF8 characters from a string
fn convert_to_utf8(filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (cow, _, _) = WINDOWS_1252.decode(&buffer);
    let (cow, _, _) = UTF_8.encode(&cow);

    let mut file = File::create(filepath)?;
    file.write_all(&cow)?;

    Ok(())
}

fn read_csv_file(file_path: &str, indexes: TableIndex) -> Result<Vec<RowData>, Box<dyn Error>> {
    let file = File::open(file_path);

    if file.is_err() {
        println!("[read_csv_file] File not found");
        return Ok(Vec::new());
    }

    let file = file.unwrap();

    let mut reader = Reader::from_reader(file);

    let mut data = Vec::new();
    for result in reader.records() {
        let record = result?;

        if record[indexes.time].eq("Time") {
            println!("Time column: {:?}", record[indexes.time].to_string());
            println!("RAM column: {:?}", record[indexes.ram].to_string());
            println!("CPU column: {:?}", record[indexes.cpu].to_string());
            println!("GPU column: {:?}", record[indexes.gpu].to_string());
            continue;
        }

        if record[indexes.time].eq("") {
            continue;
        }

        let row_data = RowData {
            time: record[indexes.time].to_string(),
            ram: record[indexes.ram].parse()?,
            cpu: record[indexes.cpu].parse()?,
            gpu: record[indexes.gpu].parse()?,
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
