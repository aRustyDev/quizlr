#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod adaptive;
pub mod curriculum;
pub mod error;
pub mod graph;
pub mod quiz;

// Features that require networking (not available in WASM)
#[cfg(feature = "native")]
pub mod auth;
#[cfg(feature = "native")]
pub mod llm;
#[cfg(feature = "native")]
pub mod storage;

// FFI module for future iOS/Android support
// #[cfg(not(target_arch = "wasm32"))]
// pub mod ffi;

// Test utilities (only available in tests)
#[cfg(test)]
pub mod test_utils;

pub use error::{QuizlrError, Result};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct QuizlrCore {
    // Core application state will be managed here
}

#[wasm_bindgen]
impl QuizlrCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        init_panic_hook();
        Self {}
    }
}

impl Default for QuizlrCore {
    fn default() -> Self {
        Self::new()
    }
}
