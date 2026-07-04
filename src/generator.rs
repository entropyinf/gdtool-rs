use crate::excel_reader::*;
use anyhow::Result;
use std::path::Path;

pub struct DataGenerator;

impl DataGenerator {
    pub fn generate_csv(data: &[SheetData], output_dir: &str, csv_ext: &str) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;

        for sheet in data {
            if sheet.rows.is_empty() {
                continue;
            }

            let file_name = format!("{}.{}", sheet.name, csv_ext);
            let file_path = Path::new(output_dir).join(file_name);

            let mut wtr = csv::Writer::from_path(file_path)?;

            let headers: Vec<_> = sheet.rows[0].keys().cloned().collect();
            wtr.write_record(&headers)?;

            for row in &sheet.rows {
                let mut record = Vec::new();
                for header in &headers {
                    let value = row.get(header).map(|s| s.as_str()).unwrap_or("");
                    record.push(value);
                }
                wtr.write_record(&record)?;
            }

            wtr.flush()?;
        }

        Ok(())
    }
}
