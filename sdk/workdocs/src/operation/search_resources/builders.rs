// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_resources::_search_resources_output::SearchResourcesOutputBuilder;

pub use crate::operation::search_resources::_search_resources_input::SearchResourcesInputBuilder;

/// Fluent builder constructing a request to `SearchResources`.
///
/// <p>Searches metadata and the content of folders, documents, document versions, and comments.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SearchResourcesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_resources::builders::SearchResourcesInputBuilder,
}
impl SearchResourcesFluentBuilder {
    /// Creates a new `SearchResources`.
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
            crate::operation::search_resources::SearchResources,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::search_resources::SearchResourcesError>,
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
        crate::operation::search_resources::SearchResourcesOutput,
        aws_smithy_http::result::SdkError<crate::operation::search_resources::SearchResourcesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_resources::paginator::SearchResourcesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::search_resources::paginator::SearchResourcesPaginator {
        crate::operation::search_resources::paginator::SearchResourcesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>The String to search for. Searches across different text fields based on request parameters. Use double quotes around the query string for exact phrase matches.</p>
    pub fn query_text(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.query_text(input.into());
        self
    }
    /// <p>The String to search for. Searches across different text fields based on request parameters. Use double quotes around the query string for exact phrase matches.</p>
    pub fn set_query_text(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_query_text(input);
        self
    }
    /// Appends an item to `QueryScopes`.
    ///
    /// To override the contents of this collection use [`set_query_scopes`](Self::set_query_scopes).
    ///
    /// <p>Filter based on the text field type. A Folder has only a name and no content. A Comment has only content and no name. A Document or Document Version has a name and content</p>
    pub fn query_scopes(mut self, input: crate::types::SearchQueryScopeType) -> Self {
        self.inner = self.inner.query_scopes(input);
        self
    }
    /// <p>Filter based on the text field type. A Folder has only a name and no content. A Comment has only content and no name. A Document or Document Version has a name and content</p>
    pub fn set_query_scopes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SearchQueryScopeType>>,
    ) -> Self {
        self.inner = self.inner.set_query_scopes(input);
        self
    }
    /// <p>Filters based on the resource owner OrgId. This is a mandatory parameter when using Admin SigV4 credentials.</p>
    pub fn organization_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>Filters based on the resource owner OrgId. This is a mandatory parameter when using Admin SigV4 credentials.</p>
    pub fn set_organization_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// Appends an item to `AdditionalResponseFields`.
    ///
    /// To override the contents of this collection use [`set_additional_response_fields`](Self::set_additional_response_fields).
    ///
    /// <p>A list of attributes to include in the response. Used to request fields that are not normally returned in a standard response.</p>
    pub fn additional_response_fields(
        mut self,
        input: crate::types::AdditionalResponseFieldType,
    ) -> Self {
        self.inner = self.inner.additional_response_fields(input);
        self
    }
    /// <p>A list of attributes to include in the response. Used to request fields that are not normally returned in a standard response.</p>
    pub fn set_additional_response_fields(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AdditionalResponseFieldType>>,
    ) -> Self {
        self.inner = self.inner.set_additional_response_fields(input);
        self
    }
    /// <p>Filters results based on entity metadata.</p>
    pub fn filters(mut self, input: crate::types::Filters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filters results based on entity metadata.</p>
    pub fn set_filters(mut self, input: std::option::Option<crate::types::Filters>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// Appends an item to `OrderBy`.
    ///
    /// To override the contents of this collection use [`set_order_by`](Self::set_order_by).
    ///
    /// <p>Order by results in one or more categories.</p>
    pub fn order_by(mut self, input: crate::types::SearchSortResult) -> Self {
        self.inner = self.inner.order_by(input);
        self
    }
    /// <p>Order by results in one or more categories.</p>
    pub fn set_order_by(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SearchSortResult>>,
    ) -> Self {
        self.inner = self.inner.set_order_by(input);
        self
    }
    /// <p>Max results count per page.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>Max results count per page.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The marker for the next set of results.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of results.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
}
