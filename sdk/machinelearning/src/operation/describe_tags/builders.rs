// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_tags::_describe_tags_output::DescribeTagsOutputBuilder;

pub use crate::operation::describe_tags::_describe_tags_input::DescribeTagsInputBuilder;

/// Fluent builder constructing a request to `DescribeTags`.
///
/// <p>Describes one or more of the tags for your Amazon ML object.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTagsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_tags::builders::DescribeTagsInputBuilder,
}
impl DescribeTagsFluentBuilder {
    /// Creates a new `DescribeTags`.
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
            crate::operation::describe_tags::DescribeTags,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_tags::DescribeTagsError>,
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
        crate::operation::describe_tags::DescribeTagsOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_tags::DescribeTagsError>,
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
    /// <p>The ID of the ML object. For example, <code>exampleModelId</code>. </p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the ML object. For example, <code>exampleModelId</code>. </p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The type of the ML object.</p>
    pub fn resource_type(mut self, input: crate::types::TaggableResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of the ML object.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::TaggableResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
}
