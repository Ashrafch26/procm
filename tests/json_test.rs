
/// tests for load and unload functions from `json.rs` moduel

use pyrust::json::{load, unload, EngineTemplate, Model, TaskOptions};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// helpers

fn example_model() -> Model {
    let mut tasks = HashMap::new();
    tasks.insert(
        "1".to_string(),
        TaskOptions { cmd: "sleep 5".to_string(), deps: vec![] },
    );
    tasks.insert(
        "2".to_string(),
        TaskOptions { cmd: "sleep 10".to_string(), deps: vec!["1".to_string()] },
    );
    Model { limite_tasks: Some(2), tasks, ..Default::default() }
}

fn tmp_path(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("pyrust_json_{}_{}.json", name, std::process::id()));
    p
}

// tests

// `unload`
#[test]
fn unload_writes_json_file() {
    let path = tmp_path("unload");
    unload(path.to_str().unwrap(), &example_model()).unwrap();
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("sleep 5"));
    assert!(contents.contains("limite_tasks"));
    fs::remove_file(&path).ok();
}

// `load`
#[test]
fn load_parses_json_file() {
    let path = tmp_path("load");
    fs::write(&path, r#"{ "limite_tasks": 2, "tasks": { "1": { "cmd": "sleep 5" }, "2": { "cmd": "sleep 10", "deps": ["1"] } } }"#).unwrap();
    let model = load(path.to_str().unwrap()).unwrap();
    assert_eq!(model.limite_tasks, Some(2));
    assert_eq!(model.tasks.len(), 2);
    assert_eq!(model.tasks["1"].cmd, "sleep 5");
    assert!(model.tasks["1"].deps.is_empty());
    assert_eq!(model.tasks["2"].deps, vec!["1".to_string()]);
    fs::remove_file(&path).ok();
}

// tests both together
#[test]
fn round_trip_load_after_unload() {
    let path = tmp_path("roundtrip");
    let model = example_model();
    unload(path.to_str().unwrap(), &model).unwrap();
    let loaded = load(path.to_str().unwrap()).unwrap();
    assert_eq!(loaded, model); // works now that PartialEq is derived
    fs::remove_file(&path).ok();
}

// load error handling
#[test]
fn load_missing_file_errors() {
    assert!(load("/definitely/not/here.json").is_err());
}

#[test]
fn load_invalid_json_errors() {
    let path = tmp_path("invalid");
    fs::write(&path, "{ not valid json").unwrap();
    assert!(load(path.to_str().unwrap()).is_err());
    fs::remove_file(&path).ok();
}

// `EngineTemplate::parse`

#[test]
fn parse_substitutes_bindings() {
    let path = tmp_path("parse");
    fs::write(
        &path,
        r#"{
            "limite_tasks": 2,
            "exit_on_first_error": true,
            "bind": { "duration": "5", "name": "procm" },
            "tasks": {
                "1": { "cmd": "sleep ^duration^" },
                "2": { "cmd": "echo ^name^ after ^duration^s", "deps": ["1"] }
            }
        }"#,
    )
    .unwrap();

    let model = EngineTemplate::new(path.to_str().unwrap()).parse();

    assert_eq!(model.limite_tasks, Some(2));
    assert_eq!(model.exit_on_first_error, Some(true));
    assert_eq!(model.tasks["1"].cmd, "sleep 5");
    assert_eq!(model.tasks["2"].cmd, "echo procm after 5s");
    assert_eq!(model.tasks["2"].deps, vec!["1".to_string()]);
    assert_eq!(model.bind.unwrap().get("name").map(String::as_str), Some("procm"));
    fs::remove_file(&path).ok();
}

#[test]
fn parse_keeps_unknown_bindings_untouched() {
    let path = tmp_path("parse_unknown");
    fs::write(
        &path,
        r#"{ "bind": { "a": "1" }, "tasks": { "1": { "cmd": "run ^a^ ^b^ ^" } } }"#,
    )
    .unwrap();

    let model = EngineTemplate::new(path.to_str().unwrap()).parse();

    assert_eq!(model.tasks["1"].cmd, "run 1 ^b^ ^");
    fs::remove_file(&path).ok();
}

#[test]
fn parse_without_bind_keeps_cmd_untouched() {
    let path = tmp_path("parse_nobind");
    fs::write(&path, r#"{ "tasks": { "1": { "cmd": "sleep ^duration^" } } }"#).unwrap();

    let model = EngineTemplate::new(path.to_str().unwrap()).parse();

    assert_eq!(model.tasks["1"].cmd, "sleep ^duration^");
    assert!(model.bind.is_none());
    fs::remove_file(&path).ok();
}
