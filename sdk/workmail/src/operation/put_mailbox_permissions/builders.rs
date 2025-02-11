// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_mailbox_permissions::_put_mailbox_permissions_output::PutMailboxPermissionsOutputBuilder;

pub use crate::operation::put_mailbox_permissions::_put_mailbox_permissions_input::PutMailboxPermissionsInputBuilder;

/// Fluent builder constructing a request to `PutMailboxPermissions`.
///
/// <p>Sets permissions for a user, group, or resource. This replaces any pre-existing permissions.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutMailboxPermissionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_mailbox_permissions::builders::PutMailboxPermissionsInputBuilder,
}
impl PutMailboxPermissionsFluentBuilder {
    /// Creates a new `PutMailboxPermissions`.
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
            crate::operation::put_mailbox_permissions::PutMailboxPermissions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_mailbox_permissions::PutMailboxPermissionsError,
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
        crate::operation::put_mailbox_permissions::PutMailboxPermissionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_mailbox_permissions::PutMailboxPermissionsError,
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
    /// <p>The identifier of the organization under which the user, group, or resource exists.</p>
    pub fn organization_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The identifier of the organization under which the user, group, or resource exists.</p>
    pub fn set_organization_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The identifier of the user, group, or resource for which to update mailbox permissions.</p>
    pub fn entity_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>The identifier of the user, group, or resource for which to update mailbox permissions.</p>
    pub fn set_entity_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
    /// <p>The identifier of the user, group, or resource to which to grant the permissions.</p>
    pub fn grantee_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.grantee_id(input.into());
        self
    }
    /// <p>The identifier of the user, group, or resource to which to grant the permissions.</p>
    pub fn set_grantee_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_grantee_id(input);
        self
    }
    /// Appends an item to `PermissionValues`.
    ///
    /// To override the contents of this collection use [`set_permission_values`](Self::set_permission_values).
    ///
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    pub fn permission_values(mut self, input: crate::types::PermissionType) -> Self {
        self.inner = self.inner.permission_values(input);
        self
    }
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    pub fn set_permission_values(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PermissionType>>,
    ) -> Self {
        self.inner = self.inner.set_permission_values(input);
        self
    }
}
