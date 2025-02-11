// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_client_branding::_import_client_branding_output::ImportClientBrandingOutputBuilder;

pub use crate::operation::import_client_branding::_import_client_branding_input::ImportClientBrandingInputBuilder;

/// Fluent builder constructing a request to `ImportClientBranding`.
///
/// <p>Imports client branding. Client branding allows you to customize your WorkSpace's client login portal. You can tailor your login portal company logo, the support email address, support link, link to reset password, and a custom message for users trying to sign in.</p>
/// <p>After you import client branding, the default branding experience for the specified platform type is replaced with the imported experience</p> <note>
/// <ul>
/// <li> <p>You must specify at least one platform type when importing client branding.</p> </li>
/// <li> <p>You can import up to 6 MB of data with each request. If your request exceeds this limit, you can import client branding for different platform types using separate requests.</p> </li>
/// <li> <p>In each platform type, the <code>SupportEmail</code> and <code>SupportLink</code> parameters are mutually exclusive. You can specify only one parameter for each platform type, but not both.</p> </li>
/// <li> <p>Imported data can take up to a minute to appear in the WorkSpaces client.</p> </li>
/// </ul>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportClientBrandingFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_client_branding::builders::ImportClientBrandingInputBuilder,
}
impl ImportClientBrandingFluentBuilder {
    /// Creates a new `ImportClientBranding`.
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
            crate::operation::import_client_branding::ImportClientBranding,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::import_client_branding::ImportClientBrandingError,
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
        crate::operation::import_client_branding::ImportClientBrandingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::import_client_branding::ImportClientBrandingError,
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
    /// <p>The directory identifier of the WorkSpace for which you want to import client branding.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The directory identifier of the WorkSpace for which you want to import client branding.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The branding information to import for Windows devices.</p>
    pub fn device_type_windows(
        mut self,
        input: crate::types::DefaultImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_windows(input);
        self
    }
    /// <p>The branding information to import for Windows devices.</p>
    pub fn set_device_type_windows(
        mut self,
        input: std::option::Option<crate::types::DefaultImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_windows(input);
        self
    }
    /// <p>The branding information to import for macOS devices.</p>
    pub fn device_type_osx(
        mut self,
        input: crate::types::DefaultImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_osx(input);
        self
    }
    /// <p>The branding information to import for macOS devices.</p>
    pub fn set_device_type_osx(
        mut self,
        input: std::option::Option<crate::types::DefaultImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_osx(input);
        self
    }
    /// <p>The branding information to import for Android devices.</p>
    pub fn device_type_android(
        mut self,
        input: crate::types::DefaultImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_android(input);
        self
    }
    /// <p>The branding information to import for Android devices.</p>
    pub fn set_device_type_android(
        mut self,
        input: std::option::Option<crate::types::DefaultImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_android(input);
        self
    }
    /// <p>The branding information to import for iOS devices.</p>
    pub fn device_type_ios(
        mut self,
        input: crate::types::IosImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_ios(input);
        self
    }
    /// <p>The branding information to import for iOS devices.</p>
    pub fn set_device_type_ios(
        mut self,
        input: std::option::Option<crate::types::IosImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_ios(input);
        self
    }
    /// <p>The branding information to import for Linux devices.</p>
    pub fn device_type_linux(
        mut self,
        input: crate::types::DefaultImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_linux(input);
        self
    }
    /// <p>The branding information to import for Linux devices.</p>
    pub fn set_device_type_linux(
        mut self,
        input: std::option::Option<crate::types::DefaultImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_linux(input);
        self
    }
    /// <p>The branding information to import for web access.</p>
    pub fn device_type_web(
        mut self,
        input: crate::types::DefaultImportClientBrandingAttributes,
    ) -> Self {
        self.inner = self.inner.device_type_web(input);
        self
    }
    /// <p>The branding information to import for web access.</p>
    pub fn set_device_type_web(
        mut self,
        input: std::option::Option<crate::types::DefaultImportClientBrandingAttributes>,
    ) -> Self {
        self.inner = self.inner.set_device_type_web(input);
        self
    }
}
