// managing the (de)serialization of the json template file.
//
// this module export two functions for loading and unloading json file
// and types
//
// template example
//
// ```json
// {
//   "limite_tasks": 2,
//   "tasks": {
//     "1": { "cmd": "sleep 5" },
//     "2": { "cmd": "sleep 10", "deps": ["1"] }
//   }
// }
// ```

// Copyright (c) 2026 Computer Science Security Team . All Rights Reserved.

use serde::Deserialize;
use serde::Serialize;

// standerd rust libs
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskOptions {
    pub cmd: String,
    #[serde(default)]
    pub deps: Vec<String>,
}

pub type Tasks = HashMap<String, TaskOptions>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(default)]
    pub limite_tasks: Option<i32>,
    pub tasks: Tasks
}

/// for deserialize
pub fn load(file_path: &str) -> Result<Model> {
    let stream = File::open(file_path)?;
    let reader = BufReader::new(stream);

    serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// for serialize
pub fn unload(file_path: &str, model: &Model) -> Result<()> {
    let stream = File::create(file_path)?;
    let writer = BufWriter::new(stream);

    serde_json::to_writer_pretty(writer, model)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
