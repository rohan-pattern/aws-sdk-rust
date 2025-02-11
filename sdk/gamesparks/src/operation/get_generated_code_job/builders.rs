// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_generated_code_job::_get_generated_code_job_output::GetGeneratedCodeJobOutputBuilder;

pub use crate::operation::get_generated_code_job::_get_generated_code_job_input::GetGeneratedCodeJobInputBuilder;

/// Fluent builder constructing a request to `GetGeneratedCodeJob`.
///
/// <p>Gets details about a job that is generating code for a snapshot.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetGeneratedCodeJobFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_generated_code_job::builders::GetGeneratedCodeJobInputBuilder,
}
impl GetGeneratedCodeJobFluentBuilder {
    /// Creates a new `GetGeneratedCodeJob`.
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
            crate::operation::get_generated_code_job::GetGeneratedCodeJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_generated_code_job::GetGeneratedCodeJobError,
        >,
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
        crate::operation::get_generated_code_job::GetGeneratedCodeJobOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_generated_code_job::GetGeneratedCodeJobError,
        >,
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
    /// <p>The name of the game.</p>
    pub fn game_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.game_name(input.into());
        self
    }
    /// <p>The name of the game.</p>
    pub fn set_game_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_game_name(input);
        self
    }
    /// <p>The identifier of the snapshot for the code generation job.</p>
    pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The identifier of the snapshot for the code generation job.</p>
    pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>The identifier of the code generation job.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The identifier of the code generation job.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
}
