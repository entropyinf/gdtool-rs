use calamine::Data;
use serde_json::Value as JsonValue;

pub trait TypeHandler: Send + Sync {
    // Type name
    fn name(&self) -> &'static str;
    // Type aliases
    fn aliases(&self) -> &'static [&'static str];
    // Godot type
    fn godot_type(&self) -> &'static str;
    // Parse value from Excel cell
    fn parse_from_excel(&self, cell: &Data) -> Option<String>;
    // Convert to CSV string
    fn to_csv_string(&self, value: &str) -> String;
    // Generate demo value
    fn demo_value(&self) -> String;
    // Godot parse code snippet
    fn godot_parse_code(&self) -> &'static str;
}

pub struct TypeRegistry {
    handlers: Vec<Box<dyn TypeHandler>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut registry = TypeRegistry {
            handlers: Vec::new(),
        };
        registry.register_default_types();
        registry
    }

    fn register_default_types(&mut self) {
        self.register(IntHandler);
        self.register(FloatHandler);
        self.register(StringHandler);
        self.register(BoolHandler);
        self.register(DictHandler);
        self.register(ArrayHandler);
        self.register(ArrayVector2iHandler);
    }

    pub fn register(&mut self, handler: impl TypeHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub fn find_handler(&self, type_name: &str) -> Option<&dyn TypeHandler> {
        let type_name = type_name.to_lowercase();
        self.handlers
            .iter()
            .find(|h| {
                h.name().to_lowercase() == type_name || h.aliases().contains(&type_name.as_str())
            })
            .map(|h| h.as_ref())
    }

    pub fn all_handlers(&self) -> &[Box<dyn TypeHandler>] {
        &self.handlers
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntHandler;

impl TypeHandler for IntHandler {
    fn name(&self) -> &'static str {
        "int"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["int", "integer", "i32", "i64"]
    }

    fn godot_type(&self) -> &'static str {
        "int"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::Int(i) => Some(i.to_string()),
            Data::Float(f) => Some((*f as i64).to_string()),
            Data::String(s) => s.parse::<i64>().ok().map(|i| i.to_string()),
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "123".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        "return int(value)"
    }
}

pub struct FloatHandler;

impl TypeHandler for FloatHandler {
    fn name(&self) -> &'static str {
        "float"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["float", "f32", "f64", "double", "number"]
    }

    fn godot_type(&self) -> &'static str {
        "float"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::Int(i) => Some((*i as f64).to_string()),
            Data::Float(f) => Some(f.to_string()),
            Data::String(s) => s.parse::<f64>().ok().map(|f| f.to_string()),
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "123.45".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        "return float(value)"
    }
}

pub struct StringHandler;

impl TypeHandler for StringHandler {
    fn name(&self) -> &'static str {
        "String"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["string", "str", "text"]
    }

    fn godot_type(&self) -> &'static str {
        "String"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::String(s) => Some(s.clone()),
            Data::Int(i) => Some(i.to_string()),
            Data::Float(f) => Some(f.to_string()),
            Data::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "Hello World".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        "return value"
    }
}

pub struct BoolHandler;

impl TypeHandler for BoolHandler {
    fn name(&self) -> &'static str {
        "bool"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["bool", "boolean"]
    }

    fn godot_type(&self) -> &'static str {
        "bool"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::Bool(b) => Some(b.to_string()),
            Data::String(s) => {
                let lower = s.to_lowercase();
                match lower.as_str() {
                    "true" | "1" | "yes" | "on" => Some("true".to_string()),
                    "false" | "0" | "no" | "off" => Some("false".to_string()),
                    _ => None,
                }
            }
            Data::Int(i) => Some(if *i != 0 { "true" } else { "false" }.to_string()),
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "true".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        r#"
            var lower = value.to_lower()
            return lower == "true" or lower == "1" or lower == "yes" or lower == "on"
        "#
    }
}

pub struct DictHandler;

impl TypeHandler for DictHandler {
    fn name(&self) -> &'static str {
        "Dictionary"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["dict", "dictionary", "object", "json"]
    }

    fn godot_type(&self) -> &'static str {
        "Dictionary"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::String(s) => {
                // Try to parse as JSON
                serde_json::from_str::<JsonValue>(s).ok()?;
                Some(s.clone())
            }
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        r#"{"key": "value", "number": 42}"#.to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        r#"
            var json = JSON.new()
            if json.parse(value) == OK: 
                return json.data
            else:
                return {}  
        "#
    }
}

pub struct ArrayHandler;

impl TypeHandler for ArrayHandler {
    fn name(&self) -> &'static str {
        "Array[String]"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    fn godot_type(&self) -> &'static str {
        "Array[String]"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::String(s) => {
                // Accept multi-line format, no validation needed for now
                Some(s.clone())
            }
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "1\n2\n3".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        "return value.split(\"\\n\")"
    }
}

pub struct ArrayVector2iHandler;

impl TypeHandler for ArrayVector2iHandler {
    fn name(&self) -> &'static str {
        "Array[Vector2i]"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    fn godot_type(&self) -> &'static str {
        "Array[Vector2i]"
    }

    fn parse_from_excel(&self, cell: &Data) -> Option<String> {
        match cell {
            Data::String(s) => Self::validate(s).then_some(s.clone()),
            _ => None,
        }
    }

    fn to_csv_string(&self, value: &str) -> String {
        value.to_string()
    }

    fn demo_value(&self) -> String {
        "(1,2),(2,3),(3,4)".to_string()
    }

    fn godot_parse_code(&self) -> &'static str {
        r#"
            var result = [] 
            var pattern = "(([0-9-]+),([0-9-]+))"
            var regex = RegEx.new()
            regex.compile(pattern)
            var matches = regex.search_all(value)
            for m in matches: 
                result.append(Vector2i(m.get_string(1).to_int(), m.get_string(2).to_int()))
            return result
        "#
    }
}

impl ArrayVector2iHandler {
    fn validate(s: &str) -> bool {
        let mut chars = s.chars().peekable();
        let mut state = State::Init;

        while let Some(&c) = chars.peek() {
            state = match (state, c) {
                (State::Init, '(') => {
                    chars.next();
                    State::ExpectX
                }
                (State::ExpectX, '0'..='9' | '-') => {
                    chars.next();
                    State::InX
                }
                (State::InX, '0'..='9') => {
                    chars.next();
                    State::InX
                }
                (State::InX, ',') => {
                    chars.next();
                    State::ExpectY
                }
                (State::ExpectY, '0'..='9' | '-') => {
                    chars.next();
                    State::InY
                }
                (State::InY, '0'..='9') => {
                    chars.next();
                    State::InY
                }
                (State::InY, ')') => {
                    chars.next();
                    State::VectorEnd
                }
                (State::VectorEnd, ',') => {
                    chars.next();
                    State::Init
                }
                (State::VectorEnd, _) if chars.peek().is_none() => State::Done,
                (State::Done, _) => return false,
                _ => return false,
            };
        }

        state == State::VectorEnd || state == State::Done
    }
}

#[derive(PartialEq)]
enum State {
    Init,
    ExpectX,
    InX,
    ExpectY,
    InY,
    VectorEnd,
    Done,
}
