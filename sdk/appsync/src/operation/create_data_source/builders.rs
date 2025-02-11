// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_data_source::_create_data_source_output::CreateDataSourceOutputBuilder;

pub use crate::operation::create_data_source::_create_data_source_input::CreateDataSourceInputBuilder;

/// Fluent builder constructing a request to `CreateDataSource`.
///
/// <p>Creates a <code>DataSource</code> object.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataSourceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_data_source::builders::CreateDataSourceInputBuilder,
}
impl CreateDataSourceFluentBuilder {
    /// Creates a new `CreateDataSource`.
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
            crate::operation::create_data_source::CreateDataSource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_data_source::CreateDataSourceError,
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
        crate::operation::create_data_source::CreateDataSourceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_data_source::CreateDataSourceError,
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
    /// <p>The API ID for the GraphQL API for the <code>DataSource</code>.</p>
    pub fn api_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API ID for the GraphQL API for the <code>DataSource</code>.</p>
    pub fn set_api_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>A user-supplied name for the <code>DataSource</code>.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A user-supplied name for the <code>DataSource</code>.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A description of the <code>DataSource</code>.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the <code>DataSource</code>.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The type of the <code>DataSource</code>.</p>
    pub fn r#type(mut self, input: crate::types::DataSourceType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of the <code>DataSource</code>.</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::DataSourceType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    pub fn service_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_role_arn(input.into());
        self
    }
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    pub fn set_service_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service_role_arn(input);
        self
    }
    /// <p>Amazon DynamoDB settings.</p>
    pub fn dynamodb_config(mut self, input: crate::types::DynamodbDataSourceConfig) -> Self {
        self.inner = self.inner.dynamodb_config(input);
        self
    }
    /// <p>Amazon DynamoDB settings.</p>
    pub fn set_dynamodb_config(
        mut self,
        input: std::option::Option<crate::types::DynamodbDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_dynamodb_config(input);
        self
    }
    /// <p>Lambda settings.</p>
    pub fn lambda_config(mut self, input: crate::types::LambdaDataSourceConfig) -> Self {
        self.inner = self.inner.lambda_config(input);
        self
    }
    /// <p>Lambda settings.</p>
    pub fn set_lambda_config(
        mut self,
        input: std::option::Option<crate::types::LambdaDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_lambda_config(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    /// <p>As of September 2021, Amazon Elasticsearch service is Amazon OpenSearch Service. This configuration is deprecated. For new data sources, use <code>CreateDataSourceRequest$openSearchServiceConfig</code> to create an OpenSearch data source.</p>
    pub fn elasticsearch_config(
        mut self,
        input: crate::types::ElasticsearchDataSourceConfig,
    ) -> Self {
        self.inner = self.inner.elasticsearch_config(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    /// <p>As of September 2021, Amazon Elasticsearch service is Amazon OpenSearch Service. This configuration is deprecated. For new data sources, use <code>CreateDataSourceRequest$openSearchServiceConfig</code> to create an OpenSearch data source.</p>
    pub fn set_elasticsearch_config(
        mut self,
        input: std::option::Option<crate::types::ElasticsearchDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_elasticsearch_config(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn open_search_service_config(
        mut self,
        input: crate::types::OpenSearchServiceDataSourceConfig,
    ) -> Self {
        self.inner = self.inner.open_search_service_config(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn set_open_search_service_config(
        mut self,
        input: std::option::Option<crate::types::OpenSearchServiceDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_open_search_service_config(input);
        self
    }
    /// <p>HTTP endpoint settings.</p>
    pub fn http_config(mut self, input: crate::types::HttpDataSourceConfig) -> Self {
        self.inner = self.inner.http_config(input);
        self
    }
    /// <p>HTTP endpoint settings.</p>
    pub fn set_http_config(
        mut self,
        input: std::option::Option<crate::types::HttpDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_http_config(input);
        self
    }
    /// <p>Relational database settings.</p>
    pub fn relational_database_config(
        mut self,
        input: crate::types::RelationalDatabaseDataSourceConfig,
    ) -> Self {
        self.inner = self.inner.relational_database_config(input);
        self
    }
    /// <p>Relational database settings.</p>
    pub fn set_relational_database_config(
        mut self,
        input: std::option::Option<crate::types::RelationalDatabaseDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_relational_database_config(input);
        self
    }
    /// <p>Amazon EventBridge settings.</p>
    pub fn event_bridge_config(mut self, input: crate::types::EventBridgeDataSourceConfig) -> Self {
        self.inner = self.inner.event_bridge_config(input);
        self
    }
    /// <p>Amazon EventBridge settings.</p>
    pub fn set_event_bridge_config(
        mut self,
        input: std::option::Option<crate::types::EventBridgeDataSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_event_bridge_config(input);
        self
    }
}
