use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Runbook {
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

    #[serde(default)]
    #[schemars(description = "Enable debug output for runn.")]
    debug: Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Interval between steps.")]
    interval: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    r#if: Option<String>,

    #[serde(default)]
    force: bool,

    #[serde(default)]
    #[schemars(
        description = "Add tokens for tracing to headers and queries by default.\n\
        Currently, HTTP runner, gRPC runner and DB runner are supported."
    )]
    trace: bool,

    steps: Option<Steps>,

    concurrency: Option<Concurrency>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
#[schemars(description = "Steps to run in runbook.\n\
        The steps are invoked in order from top to bottom.\n\
        Any return values are recorded for each step.\n\
        When steps: is array, recorded values can be retrieved with `{{ steps[*].* }}`.")]
enum Steps {
    AsMap(Map<String, Value>),
    AsList(Vec<Map<String, Value>>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
#[schemars(
    description = "Runbooks with the same key are assured of a single run at the same time."
)]
enum Concurrency {
    Single(String),
    Multiple(Vec<String>),
}

// JSONスキーマを生成する関数の例
pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Runbook);
    serde_json::to_string_pretty(&schema).unwrap()
}

pub fn parse_yaml(yaml: &str) -> Result<Runbook, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

#[cfg(test)]
mod tests {
    use glob::glob;

    use super::*;
    use std::fs;

    #[test]
    fn test_parse_runn_yaml() {
        for path in glob("external/runn/testdata/book/*.yml").unwrap().flatten() {
            let yml = fs::read_to_string(&path).unwrap();
            let result = parse_yaml(&yml);
            assert!(
                result.is_ok(),
                "Failed to parse {}: {:?}",
                path.display(),
                result.err()
            );
        }
    }
}
