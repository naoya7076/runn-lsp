use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Runbook {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[schemars(description = "Description of runbook.")]
    desc: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "Labels of runbook. Runbooks to be run can be filtered by labels.")]
    labels: Vec<String>,

    #[serde(default, skip_serializing_if = "Map::is_empty")]
    #[schemars(description = "Mapping of runners that run steps of runbook.")]
    runners: Map<String, Value>,

    #[serde(default, skip_serializing_if = "Map::is_empty")]
    #[schemars(
        description = "Allows remapping any request hostname to another hostname, \
        IP address in HTTP/gRPC/DB/CDP/SSH runners."
    )]
    host_rules: Map<String, Value>,

    #[serde(default, skip_serializing_if = "Map::is_empty")]
    #[schemars(description = "Mapping of variables available in the `steps` of runbook.")]
    vars: Map<String, Value>,

    #[serde(default, skip_serializing_if = "Map::is_empty")]
    needs: Map<String, Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "List of secret var names to be masked.")]
    secrets: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    steps: Vec<Map<String, Value>>,

    #[serde(default)]
    #[schemars(description = "Enable debug output for runn.")]
    debug: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Interval between steps.")]
    interval: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    r#if: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    concurrency: Vec<String>,
    #[serde(default)]
    force: bool,

    #[serde(default)]
    trace: bool,
}

// #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
// #[serde(untagged)]
// enum Loop {
//     Integer(u64),
//     String(String),
//     Mapping(Map<String, Value>),
// }

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
enum Concurrency {
    Single(String),
    Multiple(Vec<String>),
}

// JSONスキーマを生成する関数の例
pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Runbook);
    serde_json::to_string_pretty(&schema).unwrap()
}
