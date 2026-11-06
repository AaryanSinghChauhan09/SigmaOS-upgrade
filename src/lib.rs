//! SigmaOS-upgrade — System version upgrade tool for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-upgrade
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub mod cache;
pub mod config;
pub mod error;
pub mod installer;
pub mod package;
pub mod service;
pub use config::Config;
