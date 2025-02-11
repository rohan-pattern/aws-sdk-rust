// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_test::_start_test_output::StartTestOutputBuilder;

pub use crate::operation::start_test::_start_test_input::StartTestInputBuilder;

/// Fluent builder constructing a request to `StartTest`.
///
/// <p>Launches a Test Instance for specific Source Servers. This command starts a LAUNCH job whose initiatedBy property is StartTest and changes the SourceServer.lifeCycle.state property to TESTING.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartTestFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_test::builders::StartTestInputBuilder,
}
impl StartTestFluentBuilder {
    /// Creates a new `StartTest`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_test::StartTest,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::start_test::StartTestError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::start_test::StartTestOutput,
        aws_smithy_http::result::SdkError<crate::operation::start_test::StartTestError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Appends an item to `sourceServerIDs`.
    ///
    /// To override the contents of this collection use [`set_source_server_i_ds`](Self::set_source_server_i_ds).
    ///
    /// <p>Start Test for Source Server IDs.</p>
    pub fn source_server_i_ds(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_server_i_ds(input.into());
        self
    }
    /// <p>Start Test for Source Server IDs.</p>
    pub fn set_source_server_i_ds(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_source_server_i_ds(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Start Test by Tags.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Start Test by Tags.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
