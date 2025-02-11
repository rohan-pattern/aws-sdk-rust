// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_revision::_get_revision_output::GetRevisionOutputBuilder;

pub use crate::operation::get_revision::_get_revision_input::GetRevisionInputBuilder;

/// Fluent builder constructing a request to `GetRevision`.
///
/// <p>Returns a revision data object for a specified document ID and block address. Also returns a proof of the specified revision for verification if <code>DigestTipAddress</code> is provided.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetRevisionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_revision::builders::GetRevisionInputBuilder,
}
impl GetRevisionFluentBuilder {
    /// Creates a new `GetRevision`.
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
            crate::operation::get_revision::GetRevision,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_revision::GetRevisionError>,
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
        crate::operation::get_revision::GetRevisionOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_revision::GetRevisionError>,
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
    /// <p>The name of the ledger.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the ledger.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The block location of the document revision to be verified. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code>.</p>
    pub fn block_address(mut self, input: crate::types::ValueHolder) -> Self {
        self.inner = self.inner.block_address(input);
        self
    }
    /// <p>The block location of the document revision to be verified. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code>.</p>
    pub fn set_block_address(
        mut self,
        input: std::option::Option<crate::types::ValueHolder>,
    ) -> Self {
        self.inner = self.inner.set_block_address(input);
        self
    }
    /// <p>The UUID (represented in Base62-encoded text) of the document to be verified.</p>
    pub fn document_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.document_id(input.into());
        self
    }
    /// <p>The UUID (represented in Base62-encoded text) of the document to be verified.</p>
    pub fn set_document_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_document_id(input);
        self
    }
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code>.</p>
    pub fn digest_tip_address(mut self, input: crate::types::ValueHolder) -> Self {
        self.inner = self.inner.digest_tip_address(input);
        self
    }
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code>.</p>
    pub fn set_digest_tip_address(
        mut self,
        input: std::option::Option<crate::types::ValueHolder>,
    ) -> Self {
        self.inner = self.inner.set_digest_tip_address(input);
        self
    }
}
