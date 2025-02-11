// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_rule::_create_rule_output::CreateRuleOutputBuilder;

pub use crate::operation::create_rule::_create_rule_input::CreateRuleInputBuilder;

/// Fluent builder constructing a request to `CreateRule`.
///
/// <p>Creates a rule for use with the specified detector. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateRuleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_rule::builders::CreateRuleInputBuilder,
}
impl CreateRuleFluentBuilder {
    /// Creates a new `CreateRule`.
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
            crate::operation::create_rule::CreateRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_rule::CreateRuleError>,
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
        crate::operation::create_rule::CreateRuleOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_rule::CreateRuleError>,
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
    /// <p>The rule ID.</p>
    pub fn rule_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.rule_id(input.into());
        self
    }
    /// <p>The rule ID.</p>
    pub fn set_rule_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_rule_id(input);
        self
    }
    /// <p>The detector ID for the rule's parent detector.</p>
    pub fn detector_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The detector ID for the rule's parent detector.</p>
    pub fn set_detector_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The rule description.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The rule description.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The rule expression.</p>
    pub fn expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.expression(input.into());
        self
    }
    /// <p>The rule expression.</p>
    pub fn set_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_expression(input);
        self
    }
    /// <p>The language of the rule.</p>
    pub fn language(mut self, input: crate::types::Language) -> Self {
        self.inner = self.inner.language(input);
        self
    }
    /// <p>The language of the rule.</p>
    pub fn set_language(mut self, input: std::option::Option<crate::types::Language>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// Appends an item to `outcomes`.
    ///
    /// To override the contents of this collection use [`set_outcomes`](Self::set_outcomes).
    ///
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    pub fn outcomes(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.outcomes(input.into());
        self
    }
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    pub fn set_outcomes(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_outcomes(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A collection of key and value pairs.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A collection of key and value pairs.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
