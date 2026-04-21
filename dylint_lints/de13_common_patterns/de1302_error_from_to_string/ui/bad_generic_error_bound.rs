// Created: 2026-04-20 by Constructor Tech
// Updated: 2026-04-20 by Constructor Tech
#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct AppError(String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

#[derive(Debug)]
struct OtherError(String);

impl fmt::Display for OtherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for OtherError {}

// `.to_string()` on a field access of an Error value must still be flagged.
#[derive(Debug)]
struct Wrapper {
    inner: OtherError,
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Wrapper {}

impl From<Wrapper> for AppError {
    fn from(w: Wrapper) -> Self {
        // Should trigger DE1302 - to_string
        AppError(w.inner.to_string())
    }
}

fn main() {}
