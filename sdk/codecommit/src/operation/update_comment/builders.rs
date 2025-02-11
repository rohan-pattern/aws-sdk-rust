// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_comment::_update_comment_output::UpdateCommentOutputBuilder;

pub use crate::operation::update_comment::_update_comment_input::UpdateCommentInputBuilder;

/// Fluent builder constructing a request to `UpdateComment`.
///
/// <p>Replaces the contents of a comment.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateCommentFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_comment::builders::UpdateCommentInputBuilder,
}
impl UpdateCommentFluentBuilder {
    /// Creates a new `UpdateComment`.
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
            crate::operation::update_comment::UpdateComment,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_comment::UpdateCommentError>,
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
        crate::operation::update_comment::UpdateCommentOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_comment::UpdateCommentError>,
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
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    pub fn comment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.comment_id(input.into());
        self
    }
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    pub fn set_comment_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_comment_id(input);
        self
    }
    /// <p>The updated content to replace the existing content of the comment.</p>
    pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content(input.into());
        self
    }
    /// <p>The updated content to replace the existing content of the comment.</p>
    pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
}
