// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

use std::env;

fn main() -> Result<(), &'static str> {
    println!("cargo:rerun-if-env-changed=SGX_MODE");
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(all(feature = "sim", feature = "hyper")) {
        println!("cargo:warning=Sim feature and hyper feature, only one or neither of them can be enabled.");
        return Err("Defined feature is illegal.");
    }

    let mode = env::var("SGX_MODE").unwrap_or_else(|_| "HW".to_string());
    match mode.as_ref() {
        "SIM" | "SW" => println!("cargo:rustc-cfg=feature=\"sim\""),
        "HYPER" => println!("cargo:rustc-cfg=feature=\"hyper\""),
        _ => (),
    }

    let target = env::var("TARGET").unwrap();
    if target == "x86_64-sgx_sdk-linux-sgx" {
        println!("cargo:rustc-cfg=feature=\"use_sgx_sdk\"");
    }
    Ok(())
}
