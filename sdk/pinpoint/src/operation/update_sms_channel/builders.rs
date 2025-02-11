// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_sms_channel::_update_sms_channel_output::UpdateSmsChannelOutputBuilder;

pub use crate::operation::update_sms_channel::_update_sms_channel_input::UpdateSmsChannelInputBuilder;

/// Fluent builder constructing a request to `UpdateSmsChannel`.
///
/// <p>Enables the SMS channel for an application or updates the status and settings of the SMS channel for an application.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSmsChannelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_sms_channel::builders::UpdateSmsChannelInputBuilder,
}
impl UpdateSmsChannelFluentBuilder {
    /// Creates a new `UpdateSmsChannel`.
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
            crate::operation::update_sms_channel::UpdateSmsChannel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_sms_channel::UpdateSmsChannelError,
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
        crate::operation::update_sms_channel::UpdateSmsChannelOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_sms_channel::UpdateSmsChannelError,
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
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>Specifies the status and settings of the SMS channel for an application.</p>
    pub fn sms_channel_request(mut self, input: crate::types::SmsChannelRequest) -> Self {
        self.inner = self.inner.sms_channel_request(input);
        self
    }
    /// <p>Specifies the status and settings of the SMS channel for an application.</p>
    pub fn set_sms_channel_request(
        mut self,
        input: std::option::Option<crate::types::SmsChannelRequest>,
    ) -> Self {
        self.inner = self.inner.set_sms_channel_request(input);
        self
    }
}
