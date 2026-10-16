// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.
#![deny(unused_crate_dependencies)]

use std::path::PathBuf;
use std::sync::OnceLock;

static STORE_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Source store shared by the Rust and Python tests in this checkout.
pub fn get_store_path() -> &'static PathBuf {
    STORE_PATH.get_or_init(|| {
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/schemas/schemas.json"
        ))
    })
}
