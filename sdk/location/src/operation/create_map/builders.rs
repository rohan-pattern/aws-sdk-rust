// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_map::_create_map_output::CreateMapOutputBuilder;

pub use crate::operation::create_map::_create_map_input::CreateMapInputBuilder;

/// Fluent builder constructing a request to `CreateMap`.
///
/// <p>Creates a map resource in your Amazon Web Services account, which provides map tiles of different styles sourced from global location data providers.</p> <note>
/// <p>If your application is tracking or routing assets you use in your business, such as delivery vehicles or employees, you must not use Esri as your geolocation provider. See section 82 of the <a href="http://aws.amazon.com/service-terms">Amazon Web Services service terms</a> for more details.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateMapFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_map::builders::CreateMapInputBuilder,
}
impl CreateMapFluentBuilder {
    /// Creates a new `CreateMap`.
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
            crate::operation::create_map::CreateMap,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_map::CreateMapError>,
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
        crate::operation::create_map::CreateMapOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_map::CreateMapError>,
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
    /// <p>The name for the map resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Must contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_). </p> </li>
    /// <li> <p>Must be a unique map resource name. </p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleMap</code>.</p> </li>
    /// </ul>
    pub fn map_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.map_name(input.into());
        self
    }
    /// <p>The name for the map resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Must contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_). </p> </li>
    /// <li> <p>Must be a unique map resource name. </p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleMap</code>.</p> </li>
    /// </ul>
    pub fn set_map_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_map_name(input);
        self
    }
    /// <p>Specifies the <code>MapConfiguration</code>, including the map style, for the map resource that you create. The map style defines the look of maps and the data provider for your map resource.</p>
    pub fn configuration(mut self, input: crate::types::MapConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>Specifies the <code>MapConfiguration</code>, including the map style, for the map resource that you create. The map style defines the look of maps and the data provider for your map resource.</p>
    pub fn set_configuration(
        mut self,
        input: std::option::Option<crate::types::MapConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.inner = self.inner.pricing_plan(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn set_pricing_plan(
        mut self,
        input: std::option::Option<crate::types::PricingPlan>,
    ) -> Self {
        self.inner = self.inner.set_pricing_plan(input);
        self
    }
    /// <p>An optional description for the map resource.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description for the map resource.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair helps manage, identify, search, and filter your resources by labelling them.</p>
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
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair helps manage, identify, search, and filter your resources by labelling them.</p>
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
