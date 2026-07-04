extern crate simple_excel_writer as excel;

pub mod demo_generator;
pub mod excel_reader;
pub mod generator;
pub mod godot;
pub mod types_handler;

pub use demo_generator::DemoGenerator;
pub use excel_reader::ExcelReader;
pub use generator::DataGenerator;
pub use godot::GodotGenerator;
pub use types_handler::*;

pub fn process_excel(
    input_path: &str,
    csv_output_dir: &str,
    godot_output_dir: &str,
    csv_ext: &str,
    registry: &TypeRegistry,
) -> anyhow::Result<()> {
    let sheets = ExcelReader::read_file(input_path, registry)?;

    let sheet_data: Vec<_> = sheets.iter().map(|(_, data)| data.clone()).collect();

    DataGenerator::generate_csv(&sheet_data, csv_output_dir, csv_ext)?;
    GodotGenerator::generate(&sheets, registry, godot_output_dir)?;

    Ok(())
}
