// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_key::_create_key_output::CreateKeyOutputBuilder;

pub use crate::operation::create_key::_create_key_input::CreateKeyInputBuilder;

/// Fluent builder constructing a request to `CreateKey`.
///
/// <p>Creates an API key resource in your Amazon Web Services account, which lets you grant <code>geo:GetMap*</code> actions for Amazon Location Map resources to the API key bearer.</p> <important>
/// <p>The API keys feature is in preview. We may add, change, or remove features before announcing general availability. For more information, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/using-apikeys.html">Using API keys</a>.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateKeyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_key::builders::CreateKeyInputBuilder,
}
impl CreateKeyFluentBuilder {
    /// Creates a new `CreateKey`.
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
            crate::operation::create_key::CreateKey,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_key::CreateKeyError>,
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
        crate::operation::create_key::CreateKeyOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_key::CreateKeyError>,
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
    /// <p>A custom name for the API key resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_). </p> </li>
    /// <li> <p>Must be a unique API key name.</p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleAPIKey</code>.</p> </li>
    /// </ul>
    pub fn key_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>A custom name for the API key resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_). </p> </li>
    /// <li> <p>Must be a unique API key name.</p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleAPIKey</code>.</p> </li>
    /// </ul>
    pub fn set_key_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// <p>The API key restrictions for the API key resource.</p>
    pub fn restrictions(mut self, input: crate::types::ApiKeyRestrictions) -> Self {
        self.inner = self.inner.restrictions(input);
        self
    }
    /// <p>The API key restrictions for the API key resource.</p>
    pub fn set_restrictions(
        mut self,
        input: std::option::Option<crate::types::ApiKeyRestrictions>,
    ) -> Self {
        self.inner = self.inner.set_restrictions(input);
        self
    }
    /// <p>An optional description for the API key resource.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description for the API key resource.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The optional timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn expire_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.expire_time(input);
        self
    }
    /// <p>The optional timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn set_expire_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_expire_time(input);
        self
    }
    /// <p>Optionally set to <code>true</code> to set no expiration time for the API key. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn no_expiry(mut self, input: bool) -> Self {
        self.inner = self.inner.no_expiry(input);
        self
    }
    /// <p>Optionally set to <code>true</code> to set no expiration time for the API key. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn set_no_expiry(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_no_expiry(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair that helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <p>Format: <code>"key" : "value"</code> </p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li> <p>Maximum 50 tags per resource</p> </li>
    /// <li> <p>Each resource tag must be unique with a maximum of one value.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @. </p> </li>
    /// <li> <p>Cannot use "aws:" as a prefix for a key.</p> </li>
    /// </ul>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair that helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <p>Format: <code>"key" : "value"</code> </p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li> <p>Maximum 50 tags per resource</p> </li>
    /// <li> <p>Each resource tag must be unique with a maximum of one value.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @. </p> </li>
    /// <li> <p>Cannot use "aws:" as a prefix for a key.</p> </li>
    /// </ul>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
