// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput)
pub mod get_personalized_ranking_input {
    /// A builder for [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) campaign_arn: std::option::Option<std::string::String>,
        pub(crate) input_list: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) user_id: std::option::Option<std::string::String>,
        pub(crate) context: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) filter_arn: std::option::Option<std::string::String>,
        pub(crate) filter_values: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized
        /// ranking.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.campaign_arn = Some(input.into());
            self
        }
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.campaign_arn = input;
            self
        }
        pub fn input_list(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.input_list.unwrap_or_default();
            v.push(input.into());
            self.input_list = Some(v);
            self
        }
        pub fn set_input_list(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.input_list = input;
            self
        }
        /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        pub fn context(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.context.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.context = Some(hash_map);
            self
        }
        pub fn set_context(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.context = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user.
        /// For more information, see
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.filter_arn = Some(input.into());
            self
        }
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.filter_arn = input;
            self
        }
        pub fn filter_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.filter_values.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.filter_values = Some(hash_map);
            self
        }
        pub fn set_filter_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.filter_values = input;
            self
        }
        /// Consumes the builder and constructs a [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetPersonalizedRankingInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetPersonalizedRankingInput {
                campaign_arn: self.campaign_arn,
                input_list: self.input_list,
                user_id: self.user_id,
                context: self.context,
                filter_arn: self.filter_arn,
                filter_values: self.filter_values,
            })
        }
    }
}
#[doc(hidden)]
pub type GetPersonalizedRankingInputOperationOutputAlias = crate::operation::GetPersonalizedRanking;
#[doc(hidden)]
pub type GetPersonalizedRankingInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetPersonalizedRankingInput {
    /// Consumes the builder and constructs an Operation<[`GetPersonalizedRanking`](crate::operation::GetPersonalizedRanking)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetPersonalizedRanking,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_get_personalized_ranking(&self)
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
                crate::operation::GetPersonalizedRanking::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetPersonalizedRanking",
                "personalizeruntime",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/personalize-ranking").expect("formatting should succeed");
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
    /// Creates a new builder-style object to manufacture [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput)
    pub fn builder() -> crate::input::get_personalized_ranking_input::Builder {
        crate::input::get_personalized_ranking_input::Builder::default()
    }
}

/// See [`GetRecommendationsInput`](crate::input::GetRecommendationsInput)
pub mod get_recommendations_input {
    /// A builder for [`GetRecommendationsInput`](crate::input::GetRecommendationsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) campaign_arn: std::option::Option<std::string::String>,
        pub(crate) item_id: std::option::Option<std::string::String>,
        pub(crate) user_id: std::option::Option<std::string::String>,
        pub(crate) num_results: std::option::Option<i32>,
        pub(crate) context: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) filter_arn: std::option::Option<std::string::String>,
        pub(crate) filter_values: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.campaign_arn = Some(input.into());
            self
        }
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.campaign_arn = input;
            self
        }
        /// <p>The item ID to provide recommendations for.</p>
        /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.item_id = Some(input.into());
            self
        }
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.item_id = input;
            self
        }
        /// <p>The user ID to provide recommendations for.</p>
        /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
        pub fn num_results(mut self, input: i32) -> Self {
            self.num_results = Some(input);
            self
        }
        pub fn set_num_results(mut self, input: std::option::Option<i32>) -> Self {
            self.num_results = input;
            self
        }
        pub fn context(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.context.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.context = Some(hash_map);
            self
        }
        pub fn set_context(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.context = input;
            self
        }
        /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.filter_arn = Some(input.into());
            self
        }
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.filter_arn = input;
            self
        }
        pub fn filter_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.filter_values.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.filter_values = Some(hash_map);
            self
        }
        pub fn set_filter_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.filter_values = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecommendationsInput`](crate::input::GetRecommendationsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetRecommendationsInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetRecommendationsInput {
                campaign_arn: self.campaign_arn,
                item_id: self.item_id,
                user_id: self.user_id,
                num_results: self.num_results.unwrap_or_default(),
                context: self.context,
                filter_arn: self.filter_arn,
                filter_values: self.filter_values,
            })
        }
    }
}
#[doc(hidden)]
pub type GetRecommendationsInputOperationOutputAlias = crate::operation::GetRecommendations;
#[doc(hidden)]
pub type GetRecommendationsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetRecommendationsInput {
    /// Consumes the builder and constructs an Operation<[`GetRecommendations`](crate::operation::GetRecommendations)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetRecommendations,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_get_recommendations(&self)
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
                crate::operation::GetRecommendations::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetRecommendations",
                "personalizeruntime",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/recommendations").expect("formatting should succeed");
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
    /// Creates a new builder-style object to manufacture [`GetRecommendationsInput`](crate::input::GetRecommendationsInput)
    pub fn builder() -> crate::input::get_recommendations_input::Builder {
        crate::input::get_recommendations_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRecommendationsInput {
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
    pub campaign_arn: std::option::Option<std::string::String>,
    /// <p>The item ID to provide recommendations for.</p>
    /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
    pub item_id: std::option::Option<std::string::String>,
    /// <p>The user ID to provide recommendations for.</p>
    /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
    pub user_id: std::option::Option<std::string::String>,
    /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
    pub num_results: i32,
    /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes
    /// any interaction information that might be relevant when getting a user's recommendations, such
    /// as the user's current location or device type.</p>
    pub context:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    pub filter_arn: std::option::Option<std::string::String>,
    /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case)
    /// as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma.
    /// </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items,
    /// you must provide values for all parameters that are defined in the expression. For
    /// filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you
    /// can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of
    /// the expression to filter recommendations.</p>
    /// <p>For more information, see
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    pub filter_values:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for GetRecommendationsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRecommendationsInput");
        formatter.field("campaign_arn", &self.campaign_arn);
        formatter.field("item_id", &self.item_id);
        formatter.field("user_id", &self.user_id);
        formatter.field("num_results", &self.num_results);
        formatter.field("context", &self.context);
        formatter.field("filter_arn", &self.filter_arn);
        formatter.field("filter_values", &self.filter_values);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetPersonalizedRankingInput {
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized
    /// ranking.</p>
    pub campaign_arn: std::option::Option<std::string::String>,
    /// <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset,
    /// the item is appended to the end of the reranked list. The maximum is 500.</p>
    pub input_list: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
    pub user_id: std::option::Option<std::string::String>,
    /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes
    /// any interaction information that might be relevant when getting a user's recommendations, such
    /// as the user's current location or device type.</p>
    pub context:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user.
    /// For more information, see
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    pub filter_arn: std::option::Option<std::string::String>,
    /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case)
    /// as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma.
    /// </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items,
    /// you must provide values for all parameters that are defined in the expression. For
    /// filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you
    /// can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of
    /// the expression to filter recommendations.</p>
    /// <p>For more information, see
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    pub filter_values:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for GetPersonalizedRankingInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetPersonalizedRankingInput");
        formatter.field("campaign_arn", &self.campaign_arn);
        formatter.field("input_list", &self.input_list);
        formatter.field("user_id", &self.user_id);
        formatter.field("context", &self.context);
        formatter.field("filter_arn", &self.filter_arn);
        formatter.field("filter_values", &self.filter_values);
        formatter.finish()
    }
}
