//! # wasm-zeroize
//!
//! A WebAssembly module for secure sensitive data handling with automatic memory zeroization.
//!
//! This library provides a secure container for sensitive string data that automatically
//! clears memory when the container is dropped, helping to prevent sensitive information
//! from being leaked through memory dumps or other side-channel attacks.
//!
//! ## Example
//!
//! ```
//! use wasm_zeroize::ZeroizedString;
//!
//! // Create a secure string
//! let password = ZeroizedString::new("my-secret-password");
//!
//! // Use the secure string
//! let password_value = password.get_value();
//! assert_eq!(password_value, "my-secret-password");
//!
//! // Memory is automatically zeroized when the object is dropped
//! // Or explicitly clear it:
//! password.zeroize();
//! assert_eq!(password.get_value(), "");
//! ```

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// A secure string container that automatically zeroizes memory when dropped.
///
/// This container is designed to hold sensitive information like passwords,
/// encryption keys, or other secrets that should be cleared from memory
/// as soon as they are no longer needed.
#[wasm_bindgen]
pub struct ZeroizedString {
    inner: RefCell<String>,
}

#[wasm_bindgen]
impl ZeroizedString {
    /// Create a new secure string container with the provided data.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_zeroize::ZeroizedString;
    ///
    /// let secure_string = ZeroizedString::new("sensitive-data");
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str) -> ZeroizedString {
        ZeroizedString {
            inner: RefCell::new(data.to_string()),
        }
    }

    /// Get the current string value.
    ///
    /// # Security Considerations
    ///
    /// This method returns a copy of the sensitive data. Be careful with how you
    /// handle this returned value, as it will not be automatically zeroized.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_zeroize::ZeroizedString;
    ///
    /// let secure_string = ZeroizedString::new("sensitive-data");
    /// let value = secure_string.get_value();
    /// assert_eq!(value, "sensitive-data");
    /// ```
    pub fn get_value(&self) -> String {
        self.inner.borrow().clone()
    }

    /// Explicitly zeroize the string, clearing its contents.
    ///
    /// After calling this method, the string will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_zeroize::ZeroizedString;
    ///
    /// let secure_string = ZeroizedString::new("sensitive-data");
    /// secure_string.zeroize();
    /// assert_eq!(secure_string.get_value(), "");
    /// ```
    pub fn zeroize(&self) {
        let mut data = self.inner.borrow_mut();
        data.zeroize();
    }
}

impl Drop for ZeroizedString {
    fn drop(&mut self) {
        let mut data = self.inner.borrow_mut();
        data.zeroize();
    }
}

#[derive(Debug)]
pub enum ModuleError {
    InvalidInput(String),
    OperationFailed(String),
    // Add other error types as needed
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ModuleError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl Error for ModuleError {}

// Add unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_zeroized_string() {
        let secret = "sensitive-data";
        let zstr = ZeroizedString::new(secret);
        assert_eq!(zstr.get_value(), secret);
    }

    #[test]
    fn test_explicit_zeroize() {
        let secret = "sensitive-data";
        let zstr = ZeroizedString::new(secret);
        zstr.zeroize();
        assert_eq!(zstr.get_value(), "");
    }

    #[test]
    fn test_drop_zeroizes() {
        // This test is more of a demonstration
        let secret = "sensitive-data";
        {
            let _zstr = ZeroizedString::new(secret);
            // _zstr goes out of scope here and should be zeroized
        }
    }

    #[test]
    fn test_memory_isolation() {
        let secret1 = "password123";
        let secret2 = "different-password";
        let zstr1 = ZeroizedString::new(secret1);
        let zstr2 = ZeroizedString::new(secret2);
        zstr1.zeroize();
        assert_eq!(zstr1.get_value(), "");
        assert_eq!(zstr2.get_value(), secret2);
    }

    #[test]
    fn test_empty_string() {
        // Test with an empty string
        let zstr = ZeroizedString::new("");
        assert_eq!(zstr.get_value(), "");

        // Zeroizing an empty string should still work
        zstr.zeroize();
        assert_eq!(zstr.get_value(), "");
    }

    #[test]
    fn test_unicode_content() {
        // Test with Unicode characters
        let unicode = "パスワード123!@#$%^&*()";
        let zstr = ZeroizedString::new(unicode);
        assert_eq!(zstr.get_value(), unicode);

        zstr.zeroize();
        assert_eq!(zstr.get_value(), "");
    }

    #[test]
    fn test_large_string() {
        // Test with a larger string
        let large = "a".repeat(10000);
        let zstr = ZeroizedString::new(&large);
        assert_eq!(zstr.get_value(), large);

        zstr.zeroize();
        assert_eq!(zstr.get_value(), "");
    }
}
