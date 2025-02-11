// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_certificates::_list_certificates_output::ListCertificatesOutputBuilder;

pub use crate::operation::list_certificates::_list_certificates_input::ListCertificatesInputBuilder;

/// Fluent builder constructing a request to `ListCertificates`.
///
/// <p>Lists the certificates registered in your Amazon Web Services account.</p>
/// <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListCertificates</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListCertificatesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_certificates::builders::ListCertificatesInputBuilder,
}
impl ListCertificatesFluentBuilder {
    /// Creates a new `ListCertificates`.
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
            crate::operation::list_certificates::ListCertificates,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_certificates::ListCertificatesError,
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
        crate::operation::list_certificates::ListCertificatesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_certificates::ListCertificatesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_certificates::paginator::ListCertificatesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_certificates::paginator::ListCertificatesPaginator {
        crate::operation::list_certificates::paginator::ListCertificatesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The result page size.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn set_page_size(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
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
    /// <p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>
    pub fn ascending_order(mut self, input: bool) -> Self {
        self.inner = self.inner.ascending_order(input);
        self
    }
    /// <p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>
    pub fn set_ascending_order(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ascending_order(input);
        self
    }
}
