// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ops_metadata::_get_ops_metadata_output::GetOpsMetadataOutputBuilder;

pub use crate::operation::get_ops_metadata::_get_ops_metadata_input::GetOpsMetadataInputBuilder;

/// Fluent builder constructing a request to `GetOpsMetadata`.
///
/// <p>View operational metadata related to an application in Application Manager.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetOpsMetadataFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_ops_metadata::builders::GetOpsMetadataInputBuilder,
}
impl GetOpsMetadataFluentBuilder {
    /// Creates a new `GetOpsMetadata`.
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
            crate::operation::get_ops_metadata::GetOpsMetadata,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_ops_metadata::GetOpsMetadataError>,
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
        crate::operation::get_ops_metadata::GetOpsMetadataOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_ops_metadata::GetOpsMetadataError>,
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
    /// <p>The Amazon Resource Name (ARN) of an OpsMetadata Object to view.</p>
    pub fn ops_metadata_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ops_metadata_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an OpsMetadata Object to view.</p>
    pub fn set_ops_metadata_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ops_metadata_arn(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
