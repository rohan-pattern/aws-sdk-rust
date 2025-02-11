// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_firewall_policies::_list_firewall_policies_output::ListFirewallPoliciesOutputBuilder;

pub use crate::operation::list_firewall_policies::_list_firewall_policies_input::ListFirewallPoliciesInputBuilder;

/// Fluent builder constructing a request to `ListFirewallPolicies`.
///
/// <p>Retrieves the metadata for the firewall policies that you have defined. Depending on your setting for max results and the number of firewall policies, a single call might not return the full list. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListFirewallPoliciesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_firewall_policies::builders::ListFirewallPoliciesInputBuilder,
}
impl ListFirewallPoliciesFluentBuilder {
    /// Creates a new `ListFirewallPolicies`.
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
            crate::operation::list_firewall_policies::ListFirewallPolicies,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_firewall_policies::ListFirewallPoliciesError,
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
        crate::operation::list_firewall_policies::ListFirewallPoliciesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_firewall_policies::ListFirewallPoliciesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_firewall_policies::paginator::ListFirewallPoliciesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_firewall_policies::paginator::ListFirewallPoliciesPaginator {
        crate::operation::list_firewall_policies::paginator::ListFirewallPoliciesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of objects that you want Network Firewall to return for this request. If more objects are available, in the response, Network Firewall provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of objects that you want Network Firewall to return for this request. If more objects are available, in the response, Network Firewall provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
