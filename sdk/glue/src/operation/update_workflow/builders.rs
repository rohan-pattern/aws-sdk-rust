// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_workflow::_update_workflow_output::UpdateWorkflowOutputBuilder;

pub use crate::operation::update_workflow::_update_workflow_input::UpdateWorkflowInputBuilder;

/// Fluent builder constructing a request to `UpdateWorkflow`.
///
/// <p>Updates an existing workflow.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkflowFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_workflow::builders::UpdateWorkflowInputBuilder,
}
impl UpdateWorkflowFluentBuilder {
    /// Creates a new `UpdateWorkflow`.
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
            crate::operation::update_workflow::UpdateWorkflow,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_workflow::UpdateWorkflowError>,
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
        crate::operation::update_workflow::UpdateWorkflowOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_workflow::UpdateWorkflowError>,
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
    /// <p>Name of the workflow to be updated.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Name of the workflow to be updated.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The description of the workflow.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the workflow.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Adds a key-value pair to `DefaultRunProperties`.
    ///
    /// To override the contents of this collection use [`set_default_run_properties`](Self::set_default_run_properties).
    ///
    /// <p>A collection of properties to be used as part of each execution of the workflow.</p>
    pub fn default_run_properties(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.default_run_properties(k.into(), v.into());
        self
    }
    /// <p>A collection of properties to be used as part of each execution of the workflow.</p>
    pub fn set_default_run_properties(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_default_run_properties(input);
        self
    }
    /// <p>You can use this parameter to prevent unwanted multiple updates to data, to control costs, or in some cases, to prevent exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.</p>
    pub fn max_concurrent_runs(mut self, input: i32) -> Self {
        self.inner = self.inner.max_concurrent_runs(input);
        self
    }
    /// <p>You can use this parameter to prevent unwanted multiple updates to data, to control costs, or in some cases, to prevent exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.</p>
    pub fn set_max_concurrent_runs(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_concurrent_runs(input);
        self
    }
}
