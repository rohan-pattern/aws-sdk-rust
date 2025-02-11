// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_suite_definition::_get_suite_definition_output::GetSuiteDefinitionOutputBuilder;

pub use crate::operation::get_suite_definition::_get_suite_definition_input::GetSuiteDefinitionInputBuilder;

/// Fluent builder constructing a request to `GetSuiteDefinition`.
///
/// <p>Gets information about a Device Advisor test suite.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">GetSuiteDefinition</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetSuiteDefinitionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_suite_definition::builders::GetSuiteDefinitionInputBuilder,
}
impl GetSuiteDefinitionFluentBuilder {
    /// Creates a new `GetSuiteDefinition`.
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
            crate::operation::get_suite_definition::GetSuiteDefinition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_suite_definition::GetSuiteDefinitionError,
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
        crate::operation::get_suite_definition::GetSuiteDefinitionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_suite_definition::GetSuiteDefinitionError,
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
    /// <p>Suite definition ID of the test suite to get.</p>
    pub fn suite_definition_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.suite_definition_id(input.into());
        self
    }
    /// <p>Suite definition ID of the test suite to get.</p>
    pub fn set_suite_definition_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_suite_definition_id(input);
        self
    }
    /// <p>Suite definition version of the test suite to get.</p>
    pub fn suite_definition_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.suite_definition_version(input.into());
        self
    }
    /// <p>Suite definition version of the test suite to get.</p>
    pub fn set_suite_definition_version(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_suite_definition_version(input);
        self
    }
}
