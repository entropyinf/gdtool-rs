use crate::types_handler::*;
use anyhow::Result;
use excel::*;

pub struct DemoGenerator;

impl DemoGenerator {
    pub fn generate_demo_excel(path: &str, registry: &TypeRegistry) -> Result<()> {
        let mut wb = Workbook::create(path);

        Self::create_item_sheet(&mut wb)?;
        Self::create_skill_sheet(&mut wb)?;
        Self::create_all_types_sheet(&mut wb, registry)?;

        wb.close()?;
        Ok(())
    }

    fn create_item_sheet(wb: &mut Workbook) -> Result<()> {
        let mut sheet = wb.create_sheet("item");

        // 设置列宽
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 30.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;

            // 第 0 行：类型
            sw.append_row(row![
                "Int", "String", "Float", "Bool", "String?", "Int", "String"
            ])?;
            // 第 1 行：字段名
            sw.append_row(row![
                "id",
                "name",
                "price",
                "is_stackable",
                "description",
                "stack_max",
                "rarity"
            ])?;
            // 第 2 行：注释
            sw.append_row(row![
                "ID",
                "道具名称",
                "价格",
                "是否可堆叠",
                "描述",
                "堆叠上限",
                "稀有度"
            ])?;
            // 数据行
            sw.append_row(row![
                1.0,
                "生命药水",
                50.5,
                true,
                "恢复100点生命值",
                99.0,
                "Common"
            ])?;
            sw.append_row(row![
                2.0,
                "魔法药水",
                80.0,
                true,
                "恢复50点魔力值",
                99.0,
                "Common"
            ])?;
            sw.append_row(row![
                3.0,
                "传说之剑",
                9999.99,
                false,
                "攻击力+100",
                1.0,
                "Legendary"
            ])?;

            Ok(())
        })?;

        Ok(())
    }

    fn create_skill_sheet(wb: &mut Workbook) -> Result<()> {
        let mut sheet = wb.create_sheet("skill");

        // 设置列宽
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 30.0 });
        sheet.add_column(Column { width: 30.0 });

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;

            // 第 0 行：类型
            sw.append_row(row!["Int", "String", "Int", "Int", "Array", "Dict"])?;
            // 第 1 行：字段名
            sw.append_row(row!["id", "name", "damage", "cooldown", "tags", "config"])?;
            // 第 2 行：注释
            sw.append_row(row![
                "技能ID",
                "技能名称",
                "伤害值",
                "冷却时间",
                "标签",
                "配置"
            ])?;
            // 数据行
            sw.append_row(row![
                101.0,
                "火球术",
                150.0,
                3.0,
                r#"["Fire","Magic"]"#,
                r#"{"range": 5, "aoe": true}"#
            ])?;
            sw.append_row(row![
                102.0,
                "治愈术",
                -100.0,
                5.0,
                r#"["Heal","Support"]"#,
                r#"{"range": 3, "aoe": false}"#
            ])?;

            Ok(())
        })?;

        Ok(())
    }

    fn create_all_types_sheet(wb: &mut Workbook, registry: &TypeRegistry) -> Result<()> {
        let mut sheet = wb.create_sheet("all_types");

        // 获取所有类型
        let handlers = registry.all_handlers();

        // 设置列宽
        for _ in handlers {
            sheet.add_column(Column { width: 20.0 });
        }

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;

            // 第 0 行：类型名称
            let type_names: Vec<_> = handlers.iter().map(|h| h.name()).collect();
            sw.append_row(Row::from_iter(type_names.into_iter()))?;

            // 第 1 行：字段名
            let field_names: Vec<_> = handlers
                .iter()
                .map(|h| format!("field_{}", h.name().replace("[", "_").replace("]", "").to_lowercase()))
                .collect();
            sw.append_row(Row::from_iter(field_names.into_iter()))?;

            // 第 2 行：注释
            let comments: Vec<_> = handlers
                .iter()
                .map(|h| format!("{}类型字段", h.name()))
                .collect();
            sw.append_row(Row::from_iter(comments.into_iter()))?;

            // 数据行
            let demo_values: Vec<_> = handlers.iter().map(|h| h.demo_value()).collect();
            sw.append_row(Row::from_iter(demo_values.into_iter()))?;

            Ok(())
        })?;

        Ok(())
    }
}
