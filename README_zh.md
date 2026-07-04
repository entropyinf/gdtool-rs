
# gdtool-rs

Godot 游戏配置工具

## 功能描述

gdtool-rs 是一个用 Rust 编写的工具，用于为 Godot 游戏引擎处理 Excel 配置文件。它主要提供以下功能：

1. **Excel 文件解析与转换**
   - 读取 Excel 配置文件
   - 将 Excel 数据转换为 CSV 格式
   - 生成 Godot 可用的 GDScript 类文件和配置加载器

2. **类型系统支持**
   - 内置支持多种数据类型：Int、Float、String、Bool、Dictionary、Array[String]、Array[Vector2i]
   - 可扩展的类型处理器架构
   - 支持类型别名

3. **Demo 生成**
   - 快速生成包含所有支持类型的示例 Excel 文件
   - 提供常见游戏配置表示例（物品、技能等）

## 使用方法

### 编译

首先，确保你已经安装了 Rust 环境，然后运行：

```bash
cargo build --release
```

编译完成后，可执行文件位于 `target/release/gdtool-rs`。

### 命令行使用

#### 1. 生成 Demo Excel 文件

```bash
gdtool-rs demo --output demo.xlsx
```

这个命令会生成一个包含多个示例工作表的 Excel 文件，展示了如何使用不同类型。

#### 2. 处理 Excel 配置文件

```bash
gdtool-rs convert --input your_config.xlsx --csv_out ./data --gd_out ./data --csv_ext txt
```

参数说明：
- `--input`: 输入的 Excel 文件路径（必需）
- `--csv_out`: CSV 输出目录路径（默认：`data`）
- `--gd_out`: Godot 输出目录路径（默认：`data`）
- `--csv_ext`: CSV 文件扩展名（默认：`txt`）

### Excel 文件格式

Excel 文件需要遵循特定的格式：

- **第 0 行**：数据类型（如 Int、String、Float 等）
- **第 1 行**：字段名称（如 id、name、price 等）
- **第 2 行**：字段注释（可选，用于描述字段用途）
- **第 3 行及以后**：实际数据

示例：

| Int | String | Float | Bool | String |
|-----|--------|-------|------|--------|
| id  | name   | price | available | description |
| 物品ID | 物品名称 | 价格 | 是否可用 | 描述 |
| 1 | 生命药水 | 50.5 | true | 恢复100点生命 |

### Godot 项目集成

处理完成后，生成的文件包括：

1. **GDScript 类文件**：每个工作表对应一个资源类
2. **config_loader.gd**：配置加载器，用于加载和解析 CSV 数据

在你的 Godot 项目中：
1. 将生成的 `.gd` 文件复制到你的项目中
2. 将 CSV 数据文件放入项目目录
3. 使用 `ConfigLoader` 加载和访问配置数据

## 支持的类型

| 类型 | 别名 | Godot 类型 | 说明 |
|------|------|------------|------|
| int | int, integer, i32, i64 | int | 整数类型 |
| float | float, f32, f64, double, number | float | 浮点数类型 |
| String | string, str, text | String | 字符串类型 |
| bool | bool, boolean | bool | 布尔类型 |
| Dictionary | dict, dictionary, object, json | Dictionary | 字典/JSON 对象 |
| Array[String] | - | Array[String] | 字符串数组（多行格式） |
| Array[Vector2i] | - | Array[Vector2i] | 二维整数向量数组（格式：(x,y),(x,y)） |

