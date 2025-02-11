// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_task_set::_delete_task_set_output::DeleteTaskSetOutputBuilder;

pub use crate::operation::delete_task_set::_delete_task_set_input::DeleteTaskSetInputBuilder;

/// Fluent builder constructing a request to `DeleteTaskSet`.
///
/// <p>Deletes a specified task set within a service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTaskSetFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_task_set::builders::DeleteTaskSetInputBuilder,
}
impl DeleteTaskSetFluentBuilder {
    /// Creates a new `DeleteTaskSet`.
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
            crate::operation::delete_task_set::DeleteTaskSet,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_task_set::DeleteTaskSetError>,
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
        crate::operation::delete_task_set::DeleteTaskSetOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_task_set::DeleteTaskSetError>,
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
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set found in to delete.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set found in to delete.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster(input);
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that hosts the task set to delete.</p>
    pub fn service(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that hosts the task set to delete.</p>
    pub fn set_service(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service(input);
        self
    }
    /// <p>The task set ID or full Amazon Resource Name (ARN) of the task set to delete.</p>
    pub fn task_set(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.task_set(input.into());
        self
    }
    /// <p>The task set ID or full Amazon Resource Name (ARN) of the task set to delete.</p>
    pub fn set_task_set(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_task_set(input);
        self
    }
    /// <p>If <code>true</code>, you can delete a task set even if it hasn't been scaled down to zero.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>If <code>true</code>, you can delete a task set even if it hasn't been scaled down to zero.</p>
    pub fn set_force(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
}
