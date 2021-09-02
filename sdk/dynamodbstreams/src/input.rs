// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DescribeStreamInput`](crate::input::DescribeStreamInput)
pub mod describe_stream_input {
    /// A builder for [`DescribeStreamInput`](crate::input::DescribeStreamInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) stream_arn: std::option::Option<std::string::String>,
        pub(crate) limit: std::option::Option<i32>,
        pub(crate) exclusive_start_shard_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) for the stream.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_arn = Some(input.into());
            self
        }
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_arn = input;
            self
        }
        /// <p>The maximum number of shard objects to return. The upper limit is 100.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.limit = Some(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.limit = input;
            self
        }
        /// <p>The shard ID of the first item that this operation will evaluate. Use the value that was
        /// returned for <code>LastEvaluatedShardId</code> in the previous operation. </p>
        pub fn exclusive_start_shard_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.exclusive_start_shard_id = Some(input.into());
            self
        }
        pub fn set_exclusive_start_shard_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.exclusive_start_shard_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeStreamInput`](crate::input::DescribeStreamInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DescribeStreamInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DescribeStreamInput {
                stream_arn: self.stream_arn,
                limit: self.limit,
                exclusive_start_shard_id: self.exclusive_start_shard_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeStreamInputOperationOutputAlias = crate::operation::DescribeStream;
#[doc(hidden)]
pub type DescribeStreamInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeStreamInput {
    /// Consumes the builder and constructs an Operation<[`DescribeStream`](crate::operation::DescribeStream)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DescribeStream,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_describe_stream(&self).map_err(
                |err| smithy_http::operation::BuildError::SerializationError(err.into()),
            )?;
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
                crate::operation::DescribeStream::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DescribeStream",
                "dynamodbstreams",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "DynamoDBStreams_20120810.DescribeStream",
        );
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
    /// Creates a new builder-style object to manufacture [`DescribeStreamInput`](crate::input::DescribeStreamInput)
    pub fn builder() -> crate::input::describe_stream_input::Builder {
        crate::input::describe_stream_input::Builder::default()
    }
}

/// See [`GetRecordsInput`](crate::input::GetRecordsInput)
pub mod get_records_input {
    /// A builder for [`GetRecordsInput`](crate::input::GetRecordsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) shard_iterator: std::option::Option<std::string::String>,
        pub(crate) limit: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>A shard iterator that was retrieved from a previous GetShardIterator operation. This iterator can be used to access the stream records in this shard.</p>
        pub fn shard_iterator(mut self, input: impl Into<std::string::String>) -> Self {
            self.shard_iterator = Some(input.into());
            self
        }
        pub fn set_shard_iterator(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.shard_iterator = input;
            self
        }
        /// <p>The maximum number of records to return from the shard. The upper limit is 1000.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.limit = Some(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.limit = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecordsInput`](crate::input::GetRecordsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::GetRecordsInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetRecordsInput {
                shard_iterator: self.shard_iterator,
                limit: self.limit,
            })
        }
    }
}
#[doc(hidden)]
pub type GetRecordsInputOperationOutputAlias = crate::operation::GetRecords;
#[doc(hidden)]
pub type GetRecordsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetRecordsInput {
    /// Consumes the builder and constructs an Operation<[`GetRecords`](crate::operation::GetRecords)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetRecords,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_get_records(&self).map_err(|err| {
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
                crate::operation::GetRecords::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetRecords",
                "dynamodbstreams",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "DynamoDBStreams_20120810.GetRecords",
        );
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
    /// Creates a new builder-style object to manufacture [`GetRecordsInput`](crate::input::GetRecordsInput)
    pub fn builder() -> crate::input::get_records_input::Builder {
        crate::input::get_records_input::Builder::default()
    }
}

/// See [`GetShardIteratorInput`](crate::input::GetShardIteratorInput)
pub mod get_shard_iterator_input {
    /// A builder for [`GetShardIteratorInput`](crate::input::GetShardIteratorInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) stream_arn: std::option::Option<std::string::String>,
        pub(crate) shard_id: std::option::Option<std::string::String>,
        pub(crate) shard_iterator_type: std::option::Option<crate::model::ShardIteratorType>,
        pub(crate) sequence_number: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) for the stream.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_arn = Some(input.into());
            self
        }
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_arn = input;
            self
        }
        /// <p>The identifier of the shard. The iterator will be returned for this shard ID.</p>
        pub fn shard_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.shard_id = Some(input.into());
            self
        }
        pub fn set_shard_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.shard_id = input;
            self
        }
        /// <p>Determines how the shard iterator is used to start reading stream records from the shard:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>AT_SEQUENCE_NUMBER</code> - Start reading exactly from the position denoted by a
        /// specific sequence number.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>AFTER_SEQUENCE_NUMBER</code> - Start reading right after the position denoted by a
        /// specific sequence number.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>TRIM_HORIZON</code> - Start reading at the last (untrimmed) stream record, which is
        /// the oldest record in the shard. In DynamoDB Streams, there is a 24 hour limit on data retention.
        /// Stream records whose age exceeds this limit are subject to removal (trimming) from the
        /// stream.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LATEST</code> - Start reading just after the most recent stream record in the
        /// shard, so that you always read the most recent data in the shard.</p>
        /// </li>
        /// </ul>
        pub fn shard_iterator_type(mut self, input: crate::model::ShardIteratorType) -> Self {
            self.shard_iterator_type = Some(input);
            self
        }
        pub fn set_shard_iterator_type(
            mut self,
            input: std::option::Option<crate::model::ShardIteratorType>,
        ) -> Self {
            self.shard_iterator_type = input;
            self
        }
        /// <p>The sequence number of a stream record in the shard from which to start reading.</p>
        pub fn sequence_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.sequence_number = Some(input.into());
            self
        }
        pub fn set_sequence_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.sequence_number = input;
            self
        }
        /// Consumes the builder and constructs a [`GetShardIteratorInput`](crate::input::GetShardIteratorInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetShardIteratorInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetShardIteratorInput {
                stream_arn: self.stream_arn,
                shard_id: self.shard_id,
                shard_iterator_type: self.shard_iterator_type,
                sequence_number: self.sequence_number,
            })
        }
    }
}
#[doc(hidden)]
pub type GetShardIteratorInputOperationOutputAlias = crate::operation::GetShardIterator;
#[doc(hidden)]
pub type GetShardIteratorInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetShardIteratorInput {
    /// Consumes the builder and constructs an Operation<[`GetShardIterator`](crate::operation::GetShardIterator)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetShardIterator,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_get_shard_iterator(&self)
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
                crate::operation::GetShardIterator::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetShardIterator",
                "dynamodbstreams",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "DynamoDBStreams_20120810.GetShardIterator",
        );
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
    /// Creates a new builder-style object to manufacture [`GetShardIteratorInput`](crate::input::GetShardIteratorInput)
    pub fn builder() -> crate::input::get_shard_iterator_input::Builder {
        crate::input::get_shard_iterator_input::Builder::default()
    }
}

/// See [`ListStreamsInput`](crate::input::ListStreamsInput)
pub mod list_streams_input {
    /// A builder for [`ListStreamsInput`](crate::input::ListStreamsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) table_name: std::option::Option<std::string::String>,
        pub(crate) limit: std::option::Option<i32>,
        pub(crate) exclusive_start_stream_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>If this parameter is provided, then only the streams associated with this table name are returned.</p>
        pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.table_name = Some(input.into());
            self
        }
        pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.table_name = input;
            self
        }
        /// <p>The maximum number of streams to return. The upper limit is 100.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.limit = Some(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.limit = input;
            self
        }
        /// <p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the
        /// value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation.
        /// </p>
        pub fn exclusive_start_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.exclusive_start_stream_arn = Some(input.into());
            self
        }
        pub fn set_exclusive_start_stream_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.exclusive_start_stream_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`ListStreamsInput`](crate::input::ListStreamsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::ListStreamsInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListStreamsInput {
                table_name: self.table_name,
                limit: self.limit,
                exclusive_start_stream_arn: self.exclusive_start_stream_arn,
            })
        }
    }
}
#[doc(hidden)]
pub type ListStreamsInputOperationOutputAlias = crate::operation::ListStreams;
#[doc(hidden)]
pub type ListStreamsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListStreamsInput {
    /// Consumes the builder and constructs an Operation<[`ListStreams`](crate::operation::ListStreams)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListStreams,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_list_streams(&self).map_err(|err| {
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
                crate::operation::ListStreams::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "ListStreams",
                "dynamodbstreams",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "DynamoDBStreams_20120810.ListStreams",
        );
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
    /// Creates a new builder-style object to manufacture [`ListStreamsInput`](crate::input::ListStreamsInput)
    pub fn builder() -> crate::input::list_streams_input::Builder {
        crate::input::list_streams_input::Builder::default()
    }
}

/// <p>Represents the input of a <code>ListStreams</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListStreamsInput {
    /// <p>If this parameter is provided, then only the streams associated with this table name are returned.</p>
    pub table_name: std::option::Option<std::string::String>,
    /// <p>The maximum number of streams to return. The upper limit is 100.</p>
    pub limit: std::option::Option<i32>,
    /// <p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the
    /// value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation.
    /// </p>
    pub exclusive_start_stream_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListStreamsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListStreamsInput");
        formatter.field("table_name", &self.table_name);
        formatter.field("limit", &self.limit);
        formatter.field(
            "exclusive_start_stream_arn",
            &self.exclusive_start_stream_arn,
        );
        formatter.finish()
    }
}

/// <p>Represents the input of a <code>GetShardIterator</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetShardIteratorInput {
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    pub stream_arn: std::option::Option<std::string::String>,
    /// <p>The identifier of the shard. The iterator will be returned for this shard ID.</p>
    pub shard_id: std::option::Option<std::string::String>,
    /// <p>Determines how the shard iterator is used to start reading stream records from the shard:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>AT_SEQUENCE_NUMBER</code> - Start reading exactly from the position denoted by a
    /// specific sequence number.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>AFTER_SEQUENCE_NUMBER</code> - Start reading right after the position denoted by a
    /// specific sequence number.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>TRIM_HORIZON</code> - Start reading at the last (untrimmed) stream record, which is
    /// the oldest record in the shard. In DynamoDB Streams, there is a 24 hour limit on data retention.
    /// Stream records whose age exceeds this limit are subject to removal (trimming) from the
    /// stream.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>LATEST</code> - Start reading just after the most recent stream record in the
    /// shard, so that you always read the most recent data in the shard.</p>
    /// </li>
    /// </ul>
    pub shard_iterator_type: std::option::Option<crate::model::ShardIteratorType>,
    /// <p>The sequence number of a stream record in the shard from which to start reading.</p>
    pub sequence_number: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetShardIteratorInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetShardIteratorInput");
        formatter.field("stream_arn", &self.stream_arn);
        formatter.field("shard_id", &self.shard_id);
        formatter.field("shard_iterator_type", &self.shard_iterator_type);
        formatter.field("sequence_number", &self.sequence_number);
        formatter.finish()
    }
}

/// <p>Represents the input of a <code>GetRecords</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRecordsInput {
    /// <p>A shard iterator that was retrieved from a previous GetShardIterator operation. This iterator can be used to access the stream records in this shard.</p>
    pub shard_iterator: std::option::Option<std::string::String>,
    /// <p>The maximum number of records to return from the shard. The upper limit is 1000.</p>
    pub limit: std::option::Option<i32>,
}
impl std::fmt::Debug for GetRecordsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRecordsInput");
        formatter.field("shard_iterator", &self.shard_iterator);
        formatter.field("limit", &self.limit);
        formatter.finish()
    }
}

/// <p>Represents the input of a <code>DescribeStream</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeStreamInput {
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    pub stream_arn: std::option::Option<std::string::String>,
    /// <p>The maximum number of shard objects to return. The upper limit is 100.</p>
    pub limit: std::option::Option<i32>,
    /// <p>The shard ID of the first item that this operation will evaluate. Use the value that was
    /// returned for <code>LastEvaluatedShardId</code> in the previous operation. </p>
    pub exclusive_start_shard_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeStreamInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeStreamInput");
        formatter.field("stream_arn", &self.stream_arn);
        formatter.field("limit", &self.limit);
        formatter.field("exclusive_start_shard_id", &self.exclusive_start_shard_id);
        formatter.finish()
    }
}
