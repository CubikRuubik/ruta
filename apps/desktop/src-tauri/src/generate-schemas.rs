// use std::fs;
// use std::path::Path;

// use schemars::JsonSchema;
// use serde::Serialize;
// use serde_json::Value;
// use inventory;

// pub struct SchemaWrapper {
//     pub func: fn() -> Value,
//     pub name: &'static str,
// }

// inventory::collect!(SchemaWrapper);

// #[macro_export]
// macro_rules! register_schema {
//     ($t:ty) => {
//         inventory::submit! {
//             SchemaWrapper {
//                 func: || serde_json::to_value(schemars::schema_for!($t)).unwrap(),
//                 name: stringify!($t),
//             }
//         }
//     };
// }

// pub fn get_all_schemas() -> Value {
//     let mut map = serde_json::Map::new();

//     for schema in inventory::iter::<SchemaWrapper> {
//         map.insert(schema.name.to_string(), (schema.func)());
//     }

//     Value::Object(map)
// }

// #[derive(Serialize, JsonSchema)]
// pub struct Transfer {
//     pub time: String,
//     pub from: String,
//     pub to: String,
//     pub token: String,
// }
// register_schema!(Transfer);

// #[derive(Serialize, JsonSchema)]
// pub struct Account {
//     pub id: u64,
//     pub name: String,
//     pub balance: f64,
// }
// register_schema!(Account);

// #[derive(Serialize, JsonSchema)]
// pub struct GeneratedAccount {
//     pub id: u64,
//     pub name: String,
//     pub balance: f64,
// }
// register_schema!(GeneratedAccount);


// pub fn write_ts_types(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let schemas = get_all_schemas();

//     fs::create_dir_all(output_dir)?;

//     for (name, schema) in schemas.as_object().unwrap() {
//         let path = Path::new(output_dir).join(format!("{name}.schema.json"));
//         fs::write(path, serde_json::to_string_pretty(&schema)?)?;
//     }

//     Ok(())
// }

// fn main() {
//     write_ts_types("../gen").unwrap();
// }