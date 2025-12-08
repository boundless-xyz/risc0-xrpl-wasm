// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    path::{Path, PathBuf},
    process::Command,
};
use wasm_host_simulator::mock_data::MockData;
use wasm_host_simulator::vm_wamr::run_func;

fn build_groth16_wasm() -> PathBuf {
    let status = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            "examples/groth16_verify_simple/Cargo.toml",
            "--release",
            "--target",
            "wasm32v1-none",
        ])
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        panic!("cargo build failed with status: {status}");
    }

    Path::new("target/wasm32v1-none/release/groth16_verify_simple.wasm").to_path_buf()
}

fn build_risc0_wasm() -> PathBuf {
    let status = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            "examples/risc0_verify_simple/Cargo.toml",
            "--release",
            "--target",
            "wasm32v1-none",
        ])
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        panic!("cargo build failed with status: {status}");
    }

    Path::new("target/wasm32v1-none/release/risc0_verify_simple.wasm").to_path_buf()
}

#[test]
fn run_groth16_verify_simple() {
    let path = build_groth16_wasm();
    run_func(
        path.to_str().unwrap().to_string(),
        Some("finish"),
        None,
        MockData::new("{}", "{}", "{}", "[]", "[]"),
    )
    .expect("Executing test wasm function failed");
}

#[test]
fn run_risc0_verify_simple() {
    let path = build_risc0_wasm();
    run_func(
        path.to_str().unwrap().to_string(),
        Some("finish"),
        None,
        MockData::new("{}", "{}", "{}", "[]", "[]"),
    )
    .expect("Executing test wasm function failed");
}
