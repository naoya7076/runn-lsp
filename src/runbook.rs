use std::fs;
use std::path::Path;

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
    #[schemars(description = "Skip all `test:` sections")]
    skip_test: bool,

    #[serde(default)]
    #[schemars(description = "Force all steps to run.")]
    force: bool,

    #[serde(default)]
    #[schemars(
        description = "Add tokens for tracing to headers and queries by default.\n\
        Currently, HTTP runner, gRPC runner and DB runner are supported."
    )]
    trace: bool,

    steps: Option<Steps>,

    r#loop: Option<Loop>,

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

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
#[schemars(description = "Loop setting for runbook.")]
pub enum Loop {
    Scalar(u32),
    Mapping {
        count: Value,
        #[serde(default)]
        until: Option<String>,
        #[serde(default)]
        interval: Option<String>,
        #[serde(default)]
        min_interval: Option<f64>,
        #[serde(default)]
        max_interval: Option<f64>,
    },
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

fn parse_yaml(yaml: &str) -> Result<Runbook, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

pub fn parse_yaml_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Runbook, Box<dyn std::error::Error>> {
    let yaml = fs::read_to_string(path)?;
    let runbook = parse_yaml(&yaml)?;
    Ok(runbook)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;

    #[test]
    fn test_parse_runn_yaml() {
        for path in glob("external/runn/testdata/book/*.yml").unwrap().flatten() {
            let result = parse_yaml_from_file(&path);
            assert!(
                result.is_ok(),
                "Failed to parse {}: {:?}",
                path.display(),
                result.err()
            );
            println!("Successfully parsed: {:?}", path);
        }
    }
}
