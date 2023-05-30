use chrono::NaiveTime;
use csv::Reader;
use encoding_rs::{UTF_8, WINDOWS_1252};
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

// Modify the style here
const FONT_FAMILY: &str = "Ubuntu Mono";
const FONT_SIZE: u32 = 24;
const BACKGROUND_COLOR: &RGBColor = &WHITE;
const FOREGROUND_COLOR: &RGBColor = &BLACK;
const RAM_COLOR: &RGBColor = &BLUE;
const CPU_COLOR: &RGBColor = &RED;
const GPU_COLOR: &RGBColor = &GREEN;
const TABLE_INDEX: TableIndex = TableIndex {
    time: 1,
    ram: 7,
    cpu: 42,
    gpu: 213,
};

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
    let data = read_csv_file("table.csv", TABLE_INDEX)?;
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
    root.fill(&BACKGROUND_COLOR)?;

    let (x_values, x_labels, ram_values, cpu_values, gpu_values) = parse_csv_data(&data);

    let title_font_style = (FONT_FAMILY, FONT_SIZE * 5 / 4)
        .into_font()
        .color(&FOREGROUND_COLOR);
    let label_font_style = (FONT_FAMILY, FONT_SIZE)
        .into_font()
        .color(&FOREGROUND_COLOR);
    let legend_font_style = (FONT_FAMILY, FONT_SIZE)
        .into_font()
        .color(&FOREGROUND_COLOR);

    let mut chart = ChartBuilder::on(&root)
        // Set the title
        .caption("System Usage", title_font_style)
        // Set the spacing
        .margin(24)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..data.len() as u32, 0.0..100.0)?;

    chart
        .configure_mesh()
        .axis_style(&FOREGROUND_COLOR.mix(0.8))
        // Draw Y (vertical) axis
        .y_desc("Usage (%)")
        .y_labels(10)
        .y_label_style(label_font_style.clone())
        // Draw X (horizontal) axis
        .x_desc("Time")
        .x_label_style(label_font_style)
        .x_label_formatter(&|x| x_labels[*x as usize].clone())
        .draw()?;

    draw_lines!(chart, x_values, ram_values, "RAM", &RAM_COLOR);
    draw_lines!(chart, x_values, cpu_values, "CPU", &CPU_COLOR);
    draw_lines!(chart, x_values, gpu_values, "GPU", &GPU_COLOR);

    // Draw legends
    chart
        .configure_series_labels()
        .background_style(&FOREGROUND_COLOR.mix(0.1))
        .border_style(&FOREGROUND_COLOR.mix(0.1))
        .label_font(legend_font_style)
        .draw()?;

    Ok(())
}

fn parse_csv_data(data: &[RowData]) -> (Vec<u32>, Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>) {
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

    (x_values, x_labels, ram_values, cpu_values, gpu_values)
}

#[macro_export]
macro_rules! draw_lines {
    ($chart:expr, $x_values:expr, $item_values:expr, $label_name:expr, $color:expr) => {
        $chart
            .draw_series(LineSeries::new(
                $x_values
                    .iter()
                    .zip($item_values.iter())
                    .map(|(x, y)| (*x, *y))
                    .collect::<Vec<_>>(),
                $color,
            ))?
            .label($label_name)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], $color));
    };
}
