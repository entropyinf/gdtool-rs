
# gdtool-rs

Godot Game Configuration Tool

## Features

gdtool-rs is a tool written in Rust for processing Excel configuration files for the Godot game engine. It primarily provides the following features:

1. **Excel File Parsing and Conversion**
   - Read Excel configuration files
   - Convert Excel data to CSV format
   - Generate Godot-compatible GDScript class files and configuration loaders

2. **Type System Support**
   - Built-in support for multiple data types: Int, Float, String, Bool, Dictionary, Array[String], Array[Vector2i]
   - Extensible type handler architecture
   - Support for type aliases

3. **Demo Generation**
   - Quickly generate demo Excel files with all supported types
   - Provide common game configuration table examples (items, skills, etc.)

## Usage

### Compilation

First, ensure you have Rust installed, then run:

```bash
cargo build --release
```

After compilation, the executable will be located at `target/release/gdtool-rs`.

### Command Line Usage

#### 1. Generate Demo Excel File

```bash
gdtool-rs demo --output demo.xlsx
```

This command generates an Excel file with multiple example worksheets demonstrating how to use different types.

#### 2. Process Excel Configuration Files

```bash
gdtool-rs convert --input your_config.xlsx --csv_out ./data --gd_out ./data --csv_ext txt
```

Parameter description:
- `--input`: Input Excel file path (required)
- `--csv_out`: CSV output directory path (default: `data`)
- `--gd_out`: Godot output directory path (default: `data`)
- `--csv_ext`: CSV file extension (default: `txt`)

### Excel File Format

Excel files must follow a specific format:

- **Row 0**: Data types (e.g., Int, String, Float, etc.)
- **Row 1**: Field names (e.g., id, name, price, etc.)
- **Row 2**: Field comments (optional, for describing field purposes)
- **Row 3 and beyond**: Actual data

Example:

| Int | String | Float | Bool | String |
|-----|--------|-------|------|--------|
| id  | name   | price | available | description |
| Item ID | Item name | Price | Available | Description |
| 1 | Health Potion | 50.5 | true | Restore 100 HP |

### Integration with Godot Projects

After processing, the generated files include:

1. **GDScript class files**: One resource class per worksheet
2. **config_loader.gd**: Configuration loader for loading and parsing CSV data

In your Godot project:
1. Copy the generated `.gd` files into your project
2. Place the CSV data files in your project directory
3. Use `ConfigLoader` to load and access configuration data

## Supported Types

| Type | Aliases | Godot Type | Description |
|------|---------|------------|-------------|
| int | int, integer, i32, i64 | int | Integer type |
| float | float, f32, f64, double, number | float | Floating point type |
| String | string, str, text | String | String type |
| bool | bool, boolean | bool | Boolean type |
| Dictionary | dict, dictionary, object, json | Dictionary | Dictionary/JSON object |
| Array[String] | - | Array[String] | String array (multi-line format) |
| Array[Vector2i] | - | Array[Vector2i] | 2D integer vector array (format: (x,y),(x,y)) |

