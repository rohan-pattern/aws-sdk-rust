// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_link::_delete_link_output::DeleteLinkOutputBuilder;

pub use crate::operation::delete_link::_delete_link_input::DeleteLinkInputBuilder;

/// Fluent builder constructing a request to `DeleteLink`.
///
/// <p>Deletes an existing link. You must first disassociate the link from any devices and customer gateways.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLinkFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_link::builders::DeleteLinkInputBuilder,
}
impl DeleteLinkFluentBuilder {
    /// Creates a new `DeleteLink`.
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
            crate::operation::delete_link::DeleteLink,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_link::DeleteLinkError>,
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
        crate::operation::delete_link::DeleteLinkOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_link::DeleteLinkError>,
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
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the link.</p>
    pub fn link_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.link_id(input.into());
        self
    }
    /// <p>The ID of the link.</p>
    pub fn set_link_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_link_id(input);
        self
    }
}
