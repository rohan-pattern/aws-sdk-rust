// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_chunks::_list_chunks_output::ListChunksOutputBuilder;

pub use crate::operation::list_chunks::_list_chunks_input::ListChunksInputBuilder;

/// Fluent builder constructing a request to `ListChunks`.
///
/// List chunks in a given Object
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListChunksFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_chunks::builders::ListChunksInputBuilder,
}
impl ListChunksFluentBuilder {
    /// Creates a new `ListChunks`.
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
            crate::operation::list_chunks::ListChunks,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_chunks::ListChunksError>,
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
        crate::operation::list_chunks::ListChunksOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_chunks::ListChunksError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_chunks::paginator::ListChunksPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_chunks::paginator::ListChunksPaginator {
        crate::operation::list_chunks::paginator::ListChunksPaginator::new(self.handle, self.inner)
    }
    /// Storage job id
    pub fn storage_job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.storage_job_id(input.into());
        self
    }
    /// Storage job id
    pub fn set_storage_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_storage_job_id(input);
        self
    }
    /// Object token
    pub fn object_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.object_token(input.into());
        self
    }
    /// Object token
    pub fn set_object_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_object_token(input);
        self
    }
    /// Maximum number of chunks
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// Maximum number of chunks
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Pagination token
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// Pagination token
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
