use wasm_bindgen::prelude::*;

// Bind to the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Console log function that works in WASM
pub fn console_log(message: &str) {
    log(message);
}

/// Macro for easy console logging with formatting
#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        $crate::utils::console_log(&format!( $( $t )* ))
    }
}

/// Log an error message to the console
pub fn console_error(message: &str) {
    log(&format!("ERROR: {}", message));
}

/// Log a warning message to the console
pub fn console_warn(message: &str) {
    log(&format!("WARNING: {}", message));
}

/// Log an info message to the console
pub fn console_info(message: &str) {
    log(&format!("INFO: {}", message));
}

/// Macro for error logging
#[macro_export]
macro_rules! console_error {
    ( $( $t:tt )* ) => {
        $crate::utils::console_error(&format!( $( $t )* ))
    }
}

/// Macro for warning logging
#[macro_export]
macro_rules! console_warn {
    ( $( $t:tt )* ) => {
        $crate::utils::console_warn(&format!( $( $t )* ))
    }
}

/// Macro for info logging
#[macro_export]
macro_rules! console_info {
    ( $( $t:tt )* ) => {
        $crate::utils::console_info(&format!( $( $t )* ))
    }
}
