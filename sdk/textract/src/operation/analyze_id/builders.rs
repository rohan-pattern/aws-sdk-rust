// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::analyze_id::_analyze_id_output::AnalyzeIdOutputBuilder;

pub use crate::operation::analyze_id::_analyze_id_input::AnalyzeIdInputBuilder;

/// Fluent builder constructing a request to `AnalyzeID`.
///
/// <p>Analyzes identity documents for relevant information. This information is extracted and returned as <code>IdentityDocumentFields</code>, which records both the normalized field and value of the extracted text. Unlike other Amazon Textract operations, <code>AnalyzeID</code> doesn't return any Geometry data.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AnalyzeIDFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::analyze_id::builders::AnalyzeIdInputBuilder,
}
impl AnalyzeIDFluentBuilder {
    /// Creates a new `AnalyzeID`.
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
            crate::operation::analyze_id::AnalyzeID,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::analyze_id::AnalyzeIDError>,
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
        crate::operation::analyze_id::AnalyzeIdOutput,
        aws_smithy_http::result::SdkError<crate::operation::analyze_id::AnalyzeIDError>,
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
    /// Appends an item to `DocumentPages`.
    ///
    /// To override the contents of this collection use [`set_document_pages`](Self::set_document_pages).
    ///
    /// <p>The document being passed to AnalyzeID.</p>
    pub fn document_pages(mut self, input: crate::types::Document) -> Self {
        self.inner = self.inner.document_pages(input);
        self
    }
    /// <p>The document being passed to AnalyzeID.</p>
    pub fn set_document_pages(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Document>>,
    ) -> Self {
        self.inner = self.inner.set_document_pages(input);
        self
    }
}
