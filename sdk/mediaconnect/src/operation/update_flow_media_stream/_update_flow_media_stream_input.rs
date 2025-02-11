// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// The fields that you want to update in the media stream.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateFlowMediaStreamInput {
    /// The attributes that you want to assign to the media stream.
    #[doc(hidden)]
    pub attributes: std::option::Option<crate::types::MediaStreamAttributesRequest>,
    /// The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.
    #[doc(hidden)]
    pub clock_rate: i32,
    /// Description
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// The Amazon Resource Name (ARN) of the flow.
    #[doc(hidden)]
    pub flow_arn: std::option::Option<std::string::String>,
    /// The name of the media stream that you want to update.
    #[doc(hidden)]
    pub media_stream_name: std::option::Option<std::string::String>,
    /// The type of media stream.
    #[doc(hidden)]
    pub media_stream_type: std::option::Option<crate::types::MediaStreamType>,
    /// The resolution of the video.
    #[doc(hidden)]
    pub video_format: std::option::Option<std::string::String>,
}
impl UpdateFlowMediaStreamInput {
    /// The attributes that you want to assign to the media stream.
    pub fn attributes(&self) -> std::option::Option<&crate::types::MediaStreamAttributesRequest> {
        self.attributes.as_ref()
    }
    /// The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.
    pub fn clock_rate(&self) -> i32 {
        self.clock_rate
    }
    /// Description
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn flow_arn(&self) -> std::option::Option<&str> {
        self.flow_arn.as_deref()
    }
    /// The name of the media stream that you want to update.
    pub fn media_stream_name(&self) -> std::option::Option<&str> {
        self.media_stream_name.as_deref()
    }
    /// The type of media stream.
    pub fn media_stream_type(&self) -> std::option::Option<&crate::types::MediaStreamType> {
        self.media_stream_type.as_ref()
    }
    /// The resolution of the video.
    pub fn video_format(&self) -> std::option::Option<&str> {
        self.video_format.as_deref()
    }
}
impl UpdateFlowMediaStreamInput {
    /// Creates a new builder-style object to manufacture [`UpdateFlowMediaStreamInput`](crate::operation::update_flow_media_stream::UpdateFlowMediaStreamInput).
    pub fn builder(
    ) -> crate::operation::update_flow_media_stream::builders::UpdateFlowMediaStreamInputBuilder
    {
        crate::operation::update_flow_media_stream::builders::UpdateFlowMediaStreamInputBuilder::default()
    }
}

/// A builder for [`UpdateFlowMediaStreamInput`](crate::operation::update_flow_media_stream::UpdateFlowMediaStreamInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct UpdateFlowMediaStreamInputBuilder {
    pub(crate) attributes: std::option::Option<crate::types::MediaStreamAttributesRequest>,
    pub(crate) clock_rate: std::option::Option<i32>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) flow_arn: std::option::Option<std::string::String>,
    pub(crate) media_stream_name: std::option::Option<std::string::String>,
    pub(crate) media_stream_type: std::option::Option<crate::types::MediaStreamType>,
    pub(crate) video_format: std::option::Option<std::string::String>,
}
impl UpdateFlowMediaStreamInputBuilder {
    /// The attributes that you want to assign to the media stream.
    pub fn attributes(mut self, input: crate::types::MediaStreamAttributesRequest) -> Self {
        self.attributes = Some(input);
        self
    }
    /// The attributes that you want to assign to the media stream.
    pub fn set_attributes(
        mut self,
        input: std::option::Option<crate::types::MediaStreamAttributesRequest>,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.
    pub fn clock_rate(mut self, input: i32) -> Self {
        self.clock_rate = Some(input);
        self
    }
    /// The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.
    pub fn set_clock_rate(mut self, input: std::option::Option<i32>) -> Self {
        self.clock_rate = input;
        self
    }
    /// Description
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// Description
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn flow_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.flow_arn = Some(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn set_flow_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.flow_arn = input;
        self
    }
    /// The name of the media stream that you want to update.
    pub fn media_stream_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.media_stream_name = Some(input.into());
        self
    }
    /// The name of the media stream that you want to update.
    pub fn set_media_stream_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.media_stream_name = input;
        self
    }
    /// The type of media stream.
    pub fn media_stream_type(mut self, input: crate::types::MediaStreamType) -> Self {
        self.media_stream_type = Some(input);
        self
    }
    /// The type of media stream.
    pub fn set_media_stream_type(
        mut self,
        input: std::option::Option<crate::types::MediaStreamType>,
    ) -> Self {
        self.media_stream_type = input;
        self
    }
    /// The resolution of the video.
    pub fn video_format(mut self, input: impl Into<std::string::String>) -> Self {
        self.video_format = Some(input.into());
        self
    }
    /// The resolution of the video.
    pub fn set_video_format(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.video_format = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateFlowMediaStreamInput`](crate::operation::update_flow_media_stream::UpdateFlowMediaStreamInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_flow_media_stream::UpdateFlowMediaStreamInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::update_flow_media_stream::UpdateFlowMediaStreamInput {
                attributes: self.attributes,
                clock_rate: self.clock_rate.unwrap_or_default(),
                description: self.description,
                flow_arn: self.flow_arn,
                media_stream_name: self.media_stream_name,
                media_stream_type: self.media_stream_type,
                video_format: self.video_format,
            },
        )
    }
}
