// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_resources::_get_resources_output::GetResourcesOutputBuilder;

pub use crate::operation::get_resources::_get_resources_input::GetResourcesInputBuilder;

/// Fluent builder constructing a request to `GetResources`.
///
/// <p>Lists information about a collection of Resource resources.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetResourcesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_resources::builders::GetResourcesInputBuilder,
}
impl GetResourcesFluentBuilder {
    /// Creates a new `GetResources`.
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
            crate::operation::get_resources::GetResources,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_resources::GetResourcesError>,
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
        crate::operation::get_resources::GetResourcesOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_resources::GetResourcesError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_resources::paginator::GetResourcesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_resources::paginator::GetResourcesPaginator {
        crate::operation::get_resources::paginator::GetResourcesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn position(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.position(input.into());
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn set_position(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_position(input);
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// Appends an item to `embed`.
    ///
    /// To override the contents of this collection use [`set_embed`](Self::set_embed).
    ///
    /// <p>A query parameter used to retrieve the specified resources embedded in the returned Resources resource in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded Method resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources?embed=methods</code>.</p>
    pub fn embed(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.embed(input.into());
        self
    }
    /// <p>A query parameter used to retrieve the specified resources embedded in the returned Resources resource in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded Method resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources?embed=methods</code>.</p>
    pub fn set_embed(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_embed(input);
        self
    }
}
