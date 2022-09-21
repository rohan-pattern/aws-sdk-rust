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
//! <p>The AWS Serverless Application Repository makes it easy for developers and enterprises to quickly find
//! and deploy serverless applications in the AWS Cloud. For more information about serverless applications,
//! see Serverless Computing and Applications on the AWS website.</p><p>The AWS Serverless Application Repository is deeply integrated with the AWS Lambda console, so that developers of
//! all levels can get started with serverless computing without needing to learn anything new. You can use category
//! keywords to browse for applications such as web and mobile backends, data processing applications, or chatbots.
//! You can also search for applications by name, publisher, or event source. To use an application, you simply choose it,
//! configure any required fields, and deploy it with a few clicks. </p><p>You can also easily publish applications, sharing them publicly with the community at large, or privately
//! within your team or across your organization. To publish a serverless application (or app), you can use the
//! AWS Management Console, AWS Command Line Interface (AWS CLI), or AWS SDKs to upload the code. Along with the
//! code, you upload a simple manifest file, also known as the AWS Serverless Application Model (AWS SAM) template.
//! For more information about AWS SAM, see AWS Serverless Application Model (AWS SAM) on the AWS Labs
//! GitHub repository.</p><p>The AWS Serverless Application Repository Developer Guide contains more information about the two developer
//! experiences available:</p><ul>
//! <li>
//! <p>Consuming Applications – Browse for applications and view information about them, including
//! source code and readme files. Also install, configure, and deploy applications of your choosing. </p>
//! <p>Publishing Applications – Configure and upload applications to make them available to other
//! developers, and publish new versions of applications. </p>
//! </li>
//! </ul>
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
}
pub use aws_smithy_async::rt::sleep::AsyncSleep;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_smithy_types::timeout::Config as TimeoutConfig;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("serverlessapplicationrepository", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
