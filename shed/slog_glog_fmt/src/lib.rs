/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License found in the LICENSE file in the root
 * directory of this source tree.
 */

//! # A slog `Drain` for glog-formatted  logs.
//!
//! ## Usage
//!
//! For simple programs where you need glog-formatted logs to standard error, use the `logger`
//! convenience function to create your logger:
//!
//! ```
//! use slog::info;
//!
//! fn main() {
//!     let log = slog_glog_fmt::facebook_logger().unwrap();
//!     info!(log, "glog-formatted logs available");
//! }
//! ```
//!
//! For more complicated scenarios, create a `GlogFormat` instance using a normal slog-term
//! decorator, and use that to construct the root `Logger` instance.  You can combine the
//! `GlogFormat` drain with other drains in the usual way. The `KVCategorizer` controls how the KV
//! values should be printed.
//!
//! ```
//! use std::sync::Mutex;
//! use slog::{debug, o, Drain, Logger};
//! use slog_glog_fmt::kv_categorizer::InlineCategorizer;
//!
//! pub fn main() {
//!     let decorator = slog_term::TermDecorator::new().build();
//!     let drain = slog_glog_fmt::GlogFormat::new(decorator, InlineCategorizer).fuse();
//!     let drain = Mutex::new(drain).fuse();
//!     let log = Logger::root(drain, o!());
//!     debug!(log, "Custom logger built.");
//! }
//! ```

#![deny(warnings, missing_docs, clippy::all, intra_doc_link_resolution_failure)]

pub mod collector_serializer;
mod glog_format;
pub mod kv_categorizer;
pub mod kv_defaults;

pub use crate::glog_format::{default_drain, facebook_logger, GlogFormat};