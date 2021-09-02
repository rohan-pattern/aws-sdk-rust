// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`ListRealtimeContactAnalysisSegmentsInput`](crate::input::ListRealtimeContactAnalysisSegmentsInput)
pub mod list_realtime_contact_analysis_segments_input {
    /// A builder for [`ListRealtimeContactAnalysisSegmentsInput`](crate::input::ListRealtimeContactAnalysisSegmentsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) contact_id: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input;
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn contact_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.contact_id = Some(input.into());
            self
        }
        pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.contact_id = input;
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListRealtimeContactAnalysisSegmentsInput`](crate::input::ListRealtimeContactAnalysisSegmentsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ListRealtimeContactAnalysisSegmentsInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ListRealtimeContactAnalysisSegmentsInput {
                instance_id: self.instance_id,
                contact_id: self.contact_id,
                max_results: self.max_results.unwrap_or_default(),
                next_token: self.next_token,
            })
        }
    }
}
#[doc(hidden)]
pub type ListRealtimeContactAnalysisSegmentsInputOperationOutputAlias =
    crate::operation::ListRealtimeContactAnalysisSegments;
#[doc(hidden)]
pub type ListRealtimeContactAnalysisSegmentsInputOperationRetryAlias =
    aws_http::AwsErrorRetryPolicy;
impl ListRealtimeContactAnalysisSegmentsInput {
    /// Consumes the builder and constructs an Operation<[`ListRealtimeContactAnalysisSegments`](crate::operation::ListRealtimeContactAnalysisSegments)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListRealtimeContactAnalysisSegments,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_list_realtime_contact_analysis_segments(
                    &self,
                )
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::ListRealtimeContactAnalysisSegments::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "ListRealtimeContactAnalysisSegments",
                "connectcontactlens",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/realtime-contact-analysis/analysis-segments")
            .expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListRealtimeContactAnalysisSegmentsInput`](crate::input::ListRealtimeContactAnalysisSegmentsInput)
    pub fn builder() -> crate::input::list_realtime_contact_analysis_segments_input::Builder {
        crate::input::list_realtime_contact_analysis_segments_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListRealtimeContactAnalysisSegmentsInput {
    /// <p>The identifier of the Amazon Connect instance.</p>
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The identifier of the contact.</p>
    pub contact_id: std::option::Option<std::string::String>,
    /// <p>The maximimum number of results to return per page.</p>
    pub max_results: i32,
    /// <p>The token for the next set of results. Use the value returned in the previous
    /// response in the next request to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListRealtimeContactAnalysisSegmentsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListRealtimeContactAnalysisSegmentsInput");
        formatter.field("instance_id", &self.instance_id);
        formatter.field("contact_id", &self.contact_id);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
