#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>This is the <i>Certificate Manager Private Certificate Authority (PCA) API Reference</i>. It provides descriptions,
//! syntax, and usage examples for each of the actions and data types involved in creating
//! and managing a private certificate authority (CA) for your organization.</p>
//! <p>The documentation for each action shows the API request parameters and the JSON
//! response. Alternatively, you can use one of the Amazon Web Services SDKs to access an API that is
//! tailored to the programming language or platform that you prefer. For more information,
//! see <a href="https://aws.amazon.com/tools/#SDKs">Amazon Web Services SDKs</a>.</p>
//! <p>Each ACM Private CA API operation has a quota that determines the number of times the
//! operation can be called per second. ACM Private CA throttles API requests at different rates
//! depending on the operation. Throttling means that ACM Private CA rejects an otherwise valid
//! request because the request exceeds the operation's quota for the number of requests per
//! second. When a request is throttled, ACM Private CA returns a <a href="https://docs.aws.amazon.com/acm-pca/latest/APIReference/CommonErrors.html">ThrottlingException</a> error. ACM Private CA does not guarantee a minimum request
//! rate for APIs. </p>
//! <p>To see an up-to-date list of your ACM Private CA quotas, or to request a quota increase,
//! log into your Amazon Web Services account and visit the <a href="https://console.aws.amazon.com/servicequotas/">Service Quotas</a>
//! console.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}
pub use aws_smithy_async::rt::sleep::AsyncSleep;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_smithy_types::timeout::Config as TimeoutConfig;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("acmpca", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
