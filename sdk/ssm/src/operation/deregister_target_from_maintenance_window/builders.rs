// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_target_from_maintenance_window::_deregister_target_from_maintenance_window_output::DeregisterTargetFromMaintenanceWindowOutputBuilder;

pub use crate::operation::deregister_target_from_maintenance_window::_deregister_target_from_maintenance_window_input::DeregisterTargetFromMaintenanceWindowInputBuilder;

/// Fluent builder constructing a request to `DeregisterTargetFromMaintenanceWindow`.
///
/// <p>Removes a target from a maintenance window.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterTargetFromMaintenanceWindowFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::deregister_target_from_maintenance_window::builders::DeregisterTargetFromMaintenanceWindowInputBuilder
            }
impl DeregisterTargetFromMaintenanceWindowFluentBuilder {
    /// Creates a new `DeregisterTargetFromMaintenanceWindow`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::deregister_target_from_maintenance_window::DeregisterTargetFromMaintenanceWindow, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::deregister_target_from_maintenance_window::DeregisterTargetFromMaintenanceWindowError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::deregister_target_from_maintenance_window::DeregisterTargetFromMaintenanceWindowOutput, aws_smithy_http::result::SdkError<crate::operation::deregister_target_from_maintenance_window::DeregisterTargetFromMaintenanceWindowError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The ID of the maintenance window the target should be removed from.</p>
    pub fn window_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.window_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window the target should be removed from.</p>
    pub fn set_window_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_window_id(input);
        self
    }
    /// <p>The ID of the target definition to remove.</p>
    pub fn window_target_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.window_target_id(input.into());
        self
    }
    /// <p>The ID of the target definition to remove.</p>
    pub fn set_window_target_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_window_target_id(input);
        self
    }
    /// <p>The system checks if the target is being referenced by a task. If the target is being referenced, the system returns an error and doesn't deregister the target from the maintenance window.</p>
    pub fn safe(mut self, input: bool) -> Self {
        self.inner = self.inner.safe(input);
        self
    }
    /// <p>The system checks if the target is being referenced by a task. If the target is being referenced, the system returns an error and doesn't deregister the target from the maintenance window.</p>
    pub fn set_safe(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_safe(input);
        self
    }
}
