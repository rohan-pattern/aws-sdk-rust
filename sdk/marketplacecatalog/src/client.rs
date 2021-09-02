// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AWSMPSeymour`.
///
/// This client allows ergonomic access to a `AWSMPSeymour`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn cancel_change_set(&self) -> fluent_builders::CancelChangeSet<C, M, R> {
        fluent_builders::CancelChangeSet::new(self.handle.clone())
    }
    pub fn describe_change_set(&self) -> fluent_builders::DescribeChangeSet<C, M, R> {
        fluent_builders::DescribeChangeSet::new(self.handle.clone())
    }
    pub fn describe_entity(&self) -> fluent_builders::DescribeEntity<C, M, R> {
        fluent_builders::DescribeEntity::new(self.handle.clone())
    }
    pub fn list_change_sets(&self) -> fluent_builders::ListChangeSets<C, M, R> {
        fluent_builders::ListChangeSets::new(self.handle.clone())
    }
    pub fn list_entities(&self) -> fluent_builders::ListEntities<C, M, R> {
        fluent_builders::ListEntities::new(self.handle.clone())
    }
    pub fn start_change_set(&self) -> fluent_builders::StartChangeSet<C, M, R> {
        fluent_builders::StartChangeSet::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CancelChangeSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::cancel_change_set_input::Builder,
    }
    impl<C, M, R> CancelChangeSet<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CancelChangeSetOutput,
            smithy_http::result::SdkError<crate::error::CancelChangeSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CancelChangeSetInputOperationOutputAlias,
                crate::output::CancelChangeSetOutput,
                crate::error::CancelChangeSetError,
                crate::input::CancelChangeSetInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>.</p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique identifier of the <code>StartChangeSet</code> request that you
        /// want to cancel.</p>
        pub fn change_set_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_id(inp);
            self
        }
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeChangeSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_change_set_input::Builder,
    }
    impl<C, M, R> DescribeChangeSet<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeChangeSetOutput,
            smithy_http::result::SdkError<crate::error::DescribeChangeSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeChangeSetInputOperationOutputAlias,
                crate::output::DescribeChangeSetOutput,
                crate::error::DescribeChangeSetError,
                crate::input::DescribeChangeSetInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique identifier for the <code>StartChangeSet</code> request that you
        /// want to describe the details for.</p>
        pub fn change_set_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_id(inp);
            self
        }
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeEntity<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_entity_input::Builder,
    }
    impl<C, M, R> DescribeEntity<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeEntityOutput,
            smithy_http::result::SdkError<crate::error::DescribeEntityError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeEntityInputOperationOutputAlias,
                crate::output::DescribeEntityOutput,
                crate::error::DescribeEntityError,
                crate::input::DescribeEntityInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique ID of the entity to describe.</p>
        pub fn entity_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.entity_id(inp);
            self
        }
        pub fn set_entity_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_entity_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListChangeSets<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_change_sets_input::Builder,
    }
    impl<C, M, R> ListChangeSets<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListChangeSetsOutput,
            smithy_http::result::SdkError<crate::error::ListChangeSetsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListChangeSetsInputOperationOutputAlias,
                crate::output::ListChangeSetsOutput,
                crate::error::ListChangeSetsError,
                crate::input::ListChangeSetsInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// Appends an item to `FilterList`.
        ///
        /// To override the contents of this collection use [`set_filter_list`](Self::set_filter_list).
        /// <p>An array of filter objects.</p>
        pub fn filter_list(mut self, inp: impl Into<crate::model::Filter>) -> Self {
            self.inner = self.inner.filter_list(inp);
            self
        }
        pub fn set_filter_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filter_list(input);
            self
        }
        /// <p>An object that contains two attributes, <code>SortBy</code> and
        /// <code>SortOrder</code>.</p>
        pub fn sort(mut self, inp: crate::model::Sort) -> Self {
            self.inner = self.inner.sort(inp);
            self
        }
        pub fn set_sort(mut self, input: std::option::Option<crate::model::Sort>) -> Self {
            self.inner = self.inner.set_sort(input);
            self
        }
        /// <p>The maximum number of results returned by a single call. This value must be provided
        /// in the next call to retrieve the next set of results. By default, this value is
        /// 20.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token value retrieved from a previous call to access the next page of
        /// results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListEntities<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_entities_input::Builder,
    }
    impl<C, M, R> ListEntities<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListEntitiesOutput,
            smithy_http::result::SdkError<crate::error::ListEntitiesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListEntitiesInputOperationOutputAlias,
                crate::output::ListEntitiesOutput,
                crate::error::ListEntitiesError,
                crate::input::ListEntitiesInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>The type of entities to retrieve.</p>
        pub fn entity_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.entity_type(inp);
            self
        }
        pub fn set_entity_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_entity_type(input);
            self
        }
        /// Appends an item to `FilterList`.
        ///
        /// To override the contents of this collection use [`set_filter_list`](Self::set_filter_list).
        /// <p>An array of filter objects. Each filter object contains two attributes,
        /// <code>filterName</code> and <code>filterValues</code>.</p>
        pub fn filter_list(mut self, inp: impl Into<crate::model::Filter>) -> Self {
            self.inner = self.inner.filter_list(inp);
            self
        }
        pub fn set_filter_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filter_list(input);
            self
        }
        /// <p>An object that contains two attributes, <code>SortBy</code> and
        /// <code>SortOrder</code>.</p>
        pub fn sort(mut self, inp: crate::model::Sort) -> Self {
            self.inner = self.inner.sort(inp);
            self
        }
        pub fn set_sort(mut self, input: std::option::Option<crate::model::Sort>) -> Self {
            self.inner = self.inner.set_sort(input);
            self
        }
        /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>Specifies the upper limit of the elements on a single page. If a value isn't provided,
        /// the default value is 20.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartChangeSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_change_set_input::Builder,
    }
    impl<C, M, R> StartChangeSet<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartChangeSetOutput,
            smithy_http::result::SdkError<crate::error::StartChangeSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartChangeSetInputOperationOutputAlias,
                crate::output::StartChangeSetOutput,
                crate::error::StartChangeSetError,
                crate::input::StartChangeSetInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(inp);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// Appends an item to `ChangeSet`.
        ///
        /// To override the contents of this collection use [`set_change_set`](Self::set_change_set).
        /// <p>Array of <code>change</code> object.</p>
        pub fn change_set(mut self, inp: impl Into<crate::model::Change>) -> Self {
            self.inner = self.inner.change_set(inp);
            self
        }
        pub fn set_change_set(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Change>>,
        ) -> Self {
            self.inner = self.inner.set_change_set(input);
            self
        }
        /// <p>Optional case sensitive string of up to 100 ASCII characters. The change set name can
        /// be used to filter the list of change sets. </p>
        pub fn change_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_name(inp);
            self
        }
        pub fn set_change_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_name(input);
            self
        }
        /// <p>A unique token to identify the request to ensure idempotency.</p>
        pub fn client_request_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_request_token(inp);
            self
        }
        pub fn set_client_request_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_request_token(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
