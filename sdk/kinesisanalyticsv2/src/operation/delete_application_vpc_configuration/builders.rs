// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_application_vpc_configuration::_delete_application_vpc_configuration_output::DeleteApplicationVpcConfigurationOutputBuilder;

pub use crate::operation::delete_application_vpc_configuration::_delete_application_vpc_configuration_input::DeleteApplicationVpcConfigurationInputBuilder;

/// Fluent builder constructing a request to `DeleteApplicationVpcConfiguration`.
///
/// <p>Removes a VPC configuration from a Kinesis Data Analytics application.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplicationVpcConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_application_vpc_configuration::builders::DeleteApplicationVpcConfigurationInputBuilder
            }
impl DeleteApplicationVpcConfigurationFluentBuilder {
    /// Creates a new `DeleteApplicationVpcConfiguration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_application_vpc_configuration::DeleteApplicationVpcConfiguration, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_application_vpc_configuration::DeleteApplicationVpcConfigurationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_application_vpc_configuration::DeleteApplicationVpcConfigurationOutput, aws_smithy_http::result::SdkError<crate::operation::delete_application_vpc_configuration::DeleteApplicationVpcConfigurationError>>
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
    /// <p>The name of an existing application.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of an existing application.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The current application version ID. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The current application version ID. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn set_current_application_version_id(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The ID of the VPC configuration to delete.</p>
    pub fn vpc_configuration_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_configuration_id(input.into());
        self
    }
    /// <p>The ID of the VPC configuration to delete.</p>
    pub fn set_vpc_configuration_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpc_configuration_id(input);
        self
    }
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn conditional_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.conditional_token(input.into());
        self
    }
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn set_conditional_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_conditional_token(input);
        self
    }
}
