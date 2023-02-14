use anyhow::Result;
use nutype::nutype;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use schemars::schema_for;

#[nutype(
    sanitize(trim, lowercase)
    validate(present, max_len = 20)
)]
#[derive(*, Serialize, Deserialize, JsonSchema)]
//#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Username(String);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Person {
    name: Username,
}

fn main() -> Result<()> {
    let username = Username::new("foo")?;
    println!("{username:?}");

    let data = r#"
{
    "name": "bar"
}"#;
    let person: Person = serde_json::from_str(data)?;

    println!("{person:#?}");

    let schema = schema_for!(Person);
    let pretty = serde_json::to_string_pretty(&schema)?;
    println!("{pretty}");

    Ok(())
}
