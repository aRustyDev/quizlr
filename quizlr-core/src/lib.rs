#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod quiz;
pub mod curriculum;
pub mod adaptive;
pub mod llm;
pub mod storage;
pub mod auth;
pub mod graph;
pub mod error;

// FFI module for future iOS/Android support
// #[cfg(not(target_arch = "wasm32"))]
// pub mod ffi;

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