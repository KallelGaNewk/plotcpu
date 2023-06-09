# PlotCPU

The PlotCPU is a Rust program that reads data from a CSV file containing system monitoring information (and auto-converts to UTF-8 because HWiNFO exports in Windows-1252 🙄) and generates a line chart to visualize the usage of different system components over time.

## Features

- Reads data from a CSV file
- Parses the data into time-series records
- Generates a line chart showing the usage of RAM, CPU, and GPU over time
- Saves the chart as a PNG image

![](table.png)

## Prerequisites

- Rust programming language (>= 1.69)
- Cargo package manager (usually comes with Rust)
- [UbuntuMono Regular](https://design.ubuntu.com/font) (included in the project directory, install pls)*

<sub>*If you don't want to use UbuntuMono, you can change the font in the `FONT_FAMILY` constant in `src/main.rs`.</sub>

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/KallelGaNewk/plotcpu.git
   ```

2. Navigate to the project directory:

   ```bash
   cd plotcpu
   ```

3. Place your CSV file containing the system monitoring data in the project directory.

4. Update the `main` function in `src/main.rs` with the correct column indexes for the time, RAM, CPU, and GPU fields in the CSV file (this should already be right). This can be done by modifying the `TableIndex` struct and its usage in the `read_csv_file` function.

5. Run the program using Cargo:

    ```bash
    cargo run
    ```

6. Feel free to modify whatever you want to suit your needs.

## Roadmap

- [x] Auto-convert CSV file to UTF-8
- [ ] Command-line arguments to specify the input and output files
- [ ] Config file to specify the column indexes for the time, RAM, CPU, and GPU fields, chart title, labels, and colors
- [ ] Add functionality to compare multiple data sets
- [ ] Support for other system components (e.g. disk, network, etc.)

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

The PlotCPU program was inspired by the need to visualize system monitoring data exported from HWiNFO64 (v7.40-5000).

## Contributing

Contributions to this project are welcome. If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.
