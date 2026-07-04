use crate::types_handler::*;
use anyhow::Result;
use calamine::{DataType, Reader, Xlsx, open_workbook};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub type_name: String,
    pub is_optional: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SheetDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct SheetData {
    pub name: String,
    pub rows: Vec<HashMap<String, String>>,
}

pub struct ExcelReader;

impl ExcelReader {
    pub fn read_file(path: &str, registry: &TypeRegistry) -> Result<Vec<(SheetDef, SheetData)>> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let mut result = Vec::new();

        for sheet_name in workbook.sheet_names() {
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                if range.height() < 3 || range.width() == 0 {
                    continue;
                }

                let sheet_def = Self::parse_sheet_def(&sheet_name, &range, registry)?;
                let sheet_data = Self::parse_sheet_data(&sheet_name, &sheet_def, &range, registry)?;
                result.push((sheet_def, sheet_data));
            }
        }

        Ok(result)
    }

    fn parse_sheet_def(
        name: &str,
        range: &calamine::Range<calamine::Data>,
        registry: &TypeRegistry,
    ) -> Result<SheetDef> {
        let mut fields = Vec::new();
        let cols = range.width();

        for col in 0..cols {
            let type_cell = range.get((0, col)).and_then(|c| c.get_string());
            let name_cell = range.get((1, col)).and_then(|c| c.get_string());

            if let (Some(type_str), Some(field_name)) = (type_cell, name_cell) {
                let (type_name, is_optional) = Self::parse_type_str(type_str);
                // validate type exists
                if registry.find_handler(&type_name).is_none() {
                    eprintln!(
                        "Warning: Unknown type '{}' for field '{}', treating as String",
                        type_name, field_name
                    );
                }
                let comment = range
                    .get((2, col))
                    .and_then(|c| c.get_string())
                    .map(|s| s.to_string());

                fields.push(FieldDef {
                    name: field_name.to_string(),
                    type_name,
                    is_optional,
                    comment,
                });
            }
        }

        Ok(SheetDef {
            name: name.to_string(),
            fields,
        })
    }

    fn parse_type_str(s: &str) -> (String, bool) {
        let s = s.trim();
        let is_optional = s.ends_with('?');
        let type_name = if is_optional {
            s[..s.len() - 1].trim().to_string()
        } else {
            s.to_string()
        };
        (type_name, is_optional)
    }

    fn parse_sheet_data(
        name: &str,
        sheet_def: &SheetDef,
        range: &calamine::Range<calamine::Data>,
        registry: &TypeRegistry,
    ) -> Result<SheetData> {
        let mut rows = Vec::new();
        let start_row = 3;

        for row_idx in start_row..range.height() {
            let mut row = HashMap::new();

            for (col_idx, field) in sheet_def.fields.iter().enumerate() {
                if let Some(cell) = range.get((row_idx, col_idx)) {
                    if let Some(handler) = registry.find_handler(&field.type_name) {
                        if let Some(value) = handler.parse_from_excel(cell) {
                            row.insert(field.name.clone(), value);
                        }
                    } else {
                        // 未知类型，作为字符串处理
                        if let Some(s) = cell.get_string() {
                            row.insert(field.name.clone(), s.to_string());
                        }
                    }
                }
            }

            if !row.is_empty() {
                rows.push(row);
            }
        }

        Ok(SheetData {
            name: name.to_string(),
            rows,
        })
    }
}
