use crate::excel_reader::*;
use crate::types_handler::*;
use anyhow::Result;
use chrono::Local;
use convert_case::{Case, Casing};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Serialize)]
struct ClassTemplateData {
    class_name: String,
    fields: Vec<ClassField>,
    generator_name: String,
    generate_time: String,
}

#[derive(Serialize)]
struct ClassField {
    var_name: String,
    godot_type: String,
    comment: Option<String>,
}

#[derive(Serialize)]
struct LoaderTemplateData {
    sheets: Vec<LoaderSheetData>,
    type_cases: Vec<TypeCase>,
    generator_name: String,
    generate_time: String,
}

#[derive(Serialize)]
struct LoaderSheetData {
    sheet_name: String,
    class_name: String,
    func_name_get: String,
    func_name_all: String,
    func_name_filter: String,
    fields: Vec<SheetField>,
}

#[derive(Serialize)]
struct SheetField {
    var_name: String,
    field_name: String,
    type_name: String,
}

#[derive(Serialize)]
struct TypeCase {
    type_name: String,
    parse_code: String,
}

pub struct GodotGenerator;

impl GodotGenerator {
    pub fn generate(
        sheets: &[(SheetDef, SheetData)],
        registry: &TypeRegistry,
        output_dir: &str,
    ) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;

        // Initialize templates
        let mut tera = Tera::default();
        tera.add_raw_template("class", CLASS_TEMPLATE)?;
        tera.add_raw_template("loader", LOADER_TEMPLATE)?;

        // Generate class files
        for (sheet_def, _) in sheets {
            Self::generate_class(sheet_def, registry, &tera, output_dir)?;
        }

        // Generate loader file
        Self::generate_loader(sheets, registry, &tera, output_dir)?;

        Ok(())
    }

    fn generate_class(
        sheet_def: &SheetDef,
        registry: &TypeRegistry,
        tera: &Tera,
        output_dir: &str,
    ) -> Result<()> {
        let class_name = sheet_def.name.to_case(Case::Pascal);
        let file_name = format!("{}.gd", sheet_def.name.to_case(Case::Snake));
        let file_path = Path::new(output_dir).join(file_name);

        let fields = sheet_def
            .fields
            .iter()
            .map(|field| {
                let godot_type = registry
                    .find_handler(&field.type_name)
                    .map(|h| h.godot_type())
                    .unwrap_or("String");
                ClassField {
                    var_name: field.name.to_case(Case::Snake),
                    godot_type: godot_type.to_string(),
                    comment: field.comment.clone(),
                }
            })
            .collect();

        let data = ClassTemplateData {
            class_name,
            fields,
            generator_name: "gdtool-rs".to_string(),
            generate_time: Local::now().format("%Y-%m-%d").to_string(),
        };
        let context = Context::from_serialize(&data)?;
        let content = tera.render("class", &context)?;

        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn generate_loader(
        sheets: &[(SheetDef, SheetData)],
        registry: &TypeRegistry,
        tera: &Tera,
        output_dir: &str,
    ) -> Result<()> {
        let file_path = Path::new(output_dir).join("config_loader.gd");

        let sheets_data = sheets
            .iter()
            .map(|(sheet_def, _)| LoaderSheetData {
                sheet_name: sheet_def.name.clone(),
                class_name: sheet_def.name.to_case(Case::Pascal),
                func_name_get: format!("get_{}", sheet_def.name.to_case(Case::Snake)),
                func_name_all: format!("get_all_{}", sheet_def.name.to_case(Case::Snake)),
                func_name_filter: format!("filter_{}", sheet_def.name.to_case(Case::Snake)),
                fields: sheet_def
                    .fields
                    .iter()
                    .map(|field| SheetField {
                        var_name: field.name.to_case(Case::Snake),
                        field_name: field.name.clone(),
                        type_name: field.type_name.clone(),
                    })
                    .collect(),
            })
            .collect();

        let type_cases = registry
            .all_handlers()
            .iter()
            .map(|h| TypeCase {
                type_name: h.name().to_string(),
                parse_code: h.godot_parse_code().to_string(),
            })
            .collect();

        let data = LoaderTemplateData {
            sheets: sheets_data,
            type_cases,
            generator_name: "gdtool-rs".to_string(),
            generate_time: Local::now().format("%Y-%m-%d").to_string(),
        };
        let context = Context::from_serialize(&data)?;
        let content = tera.render("loader", &context)?;

        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

const CLASS_TEMPLATE: &str = r#"
# @generated
# ==============================================
# Auto-generated code, do not edit manually!
# All changes will be lost when regenerating next time
# Generator: {{ generator_name }}
# Generate Time: {{ generate_time }}
# ==============================================
extends Resource
class_name {{ class_name }}

{% for field in fields -%}
{% if field.comment -%}
## {{ field.comment }}
{% endif %}
@export var {{ field.var_name }}: {{ field.godot_type }}

{% endfor %}
"#;

const LOADER_TEMPLATE: &str = r#"
# @generated
# ==============================================
# Auto-generated code, do not edit manually!
# All changes will be lost when regenerating next time
# Generator: {{ generator_name }}
# Generate Time: {{ generate_time }}
# ==============================================
extends Node
class_name ConfigLoader

var configs: Dictionary = {}

func load_config(csv_path: String) -> void:
    var file = FileAccess.open(csv_path, FileAccess.READ)
    if not file:
        push_error("Failed to open file: " + csv_path)
        return

    var rows: Array = []
    var headers: PackedStringArray = file.get_csv_line()

    while not file.eof_reached():
        var values = file.get_csv_line()
        if values.size() == headers.size():
            var row = {}
            for i in range(headers.size()):
                row[headers[i]] = values[i]
            rows.append(row)

    var sheet_name = csv_path.get_file().get_basename()
    configs[sheet_name] = rows

func get_sheet(sheet_name: String) -> Array:
    return configs.get(sheet_name, [])

{% for sheet in sheets -%}
func {{ sheet.func_name_get }}(id: int) -> {{ sheet.class_name }}:
    var results = {{ sheet.func_name_filter }}(func(obj): return obj.id == id)
    return results[0] if not results.is_empty() else null


func {{ sheet.func_name_all }}() -> Array[{{ sheet.class_name }}]:
    return {{ sheet.func_name_filter }}(func(obj): return true)


func {{ sheet.func_name_filter }}(filter_func: Callable) -> Array[{{ sheet.class_name }}]:
    var sheet_data = get_sheet("{{ sheet.sheet_name }}")
    var result: Array[{{ sheet.class_name }}] = []
    for row in sheet_data:
        var obj = {{ sheet.class_name }}()
{% for field in sheet.fields %}        obj.{{ field.var_name }} = _parse_value(row.get("{{ field.field_name }}"), "{{ field.type_name }}")
{% endfor %}        if filter_func.call(obj):
            result.append(obj)
    return result

{% endfor %}
func _parse_value(value: String, type_name: String):
    match type_name:
{% for case in type_cases %}        "{{ case.type_name }}":
            {{ case.parse_code }}
{% endfor %}        _:
            return value
"#;
