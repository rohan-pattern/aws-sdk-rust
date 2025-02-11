// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_user_id::_get_user_id_output::GetUserIdOutputBuilder;

pub use crate::operation::get_user_id::_get_user_id_input::GetUserIdInputBuilder;

/// Fluent builder constructing a request to `GetUserId`.
///
/// <p>Retrieves the <code>UserId</code> in an identity store.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetUserIdFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_user_id::builders::GetUserIdInputBuilder,
}
impl GetUserIdFluentBuilder {
    /// Creates a new `GetUserId`.
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
            crate::operation::get_user_id::GetUserId,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_user_id::GetUserIdError>,
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
        crate::operation::get_user_id::GetUserIdOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_user_id::GetUserIdError>,
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
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.identity_store_id(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_identity_store_id(input);
        self
    }
    /// <p>A unique identifier for a user or group that is not the primary identifier. This value can be an identifier from an external identity provider (IdP) that is associated with the user, the group, or a unique attribute. For the unique attribute, the only valid paths are <code>userName</code> and <code>emails.value</code>.</p>
    pub fn alternate_identifier(mut self, input: crate::types::AlternateIdentifier) -> Self {
        self.inner = self.inner.alternate_identifier(input);
        self
    }
    /// <p>A unique identifier for a user or group that is not the primary identifier. This value can be an identifier from an external identity provider (IdP) that is associated with the user, the group, or a unique attribute. For the unique attribute, the only valid paths are <code>userName</code> and <code>emails.value</code>.</p>
    pub fn set_alternate_identifier(
        mut self,
        input: std::option::Option<crate::types::AlternateIdentifier>,
    ) -> Self {
        self.inner = self.inner.set_alternate_identifier(input);
        self
    }
}
