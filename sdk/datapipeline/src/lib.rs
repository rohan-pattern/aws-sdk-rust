#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>AWS Data Pipeline configures and manages a data-driven workflow called a pipeline. AWS Data Pipeline
//! handles the details of scheduling and ensuring that data dependencies are met so that your application
//! can focus on processing the data.</p>
//! <p>AWS Data Pipeline provides a JAR implementation of a task runner called AWS Data Pipeline Task Runner.
//! AWS Data Pipeline Task Runner provides logic for common data management scenarios, such as performing
//! database queries and running data analysis using Amazon Elastic MapReduce (Amazon EMR). You can use
//! AWS Data Pipeline Task Runner as your task runner, or you can write your own task runner to provide
//! custom data management.</p>
//! <p>AWS Data Pipeline implements two main sets of functionality. Use the first set to create a pipeline
//! and define data sources, schedules, dependencies, and the transforms to be performed on the data.
//! Use the second set in your task runner application to receive the next task ready for processing.
//! The logic for performing the task, such as querying the data, running data analysis, or converting
//! the data from one format to another, is contained within the task runner. The task runner performs
//! the task assigned to it by the web service, reporting progress to the web service as it does so.
//! When the task is done, the task runner reports the final success or failure of the task to the web service.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("datapipeline", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
