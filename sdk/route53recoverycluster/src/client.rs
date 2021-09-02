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

/// An ergonomic service client for `ToggleCustomerAPI`.
///
/// This client allows ergonomic access to a `ToggleCustomerAPI`-shaped service.
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
    pub fn get_routing_control_state(&self) -> fluent_builders::GetRoutingControlState<C, M, R> {
        fluent_builders::GetRoutingControlState::new(self.handle.clone())
    }
    pub fn update_routing_control_state(
        &self,
    ) -> fluent_builders::UpdateRoutingControlState<C, M, R> {
        fluent_builders::UpdateRoutingControlState::new(self.handle.clone())
    }
    pub fn update_routing_control_states(
        &self,
    ) -> fluent_builders::UpdateRoutingControlStates<C, M, R> {
        fluent_builders::UpdateRoutingControlStates::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct GetRoutingControlState<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_routing_control_state_input::Builder,
    }
    impl<C, M, R> GetRoutingControlState<C, M, R>
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
            crate::output::GetRoutingControlStateOutput,
            smithy_http::result::SdkError<crate::error::GetRoutingControlStateError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRoutingControlStateInputOperationOutputAlias,
                crate::output::GetRoutingControlStateOutput,
                crate::error::GetRoutingControlStateError,
                crate::input::GetRoutingControlStateInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
        pub fn routing_control_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.routing_control_arn(inp);
            self
        }
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateRoutingControlState<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_routing_control_state_input::Builder,
    }
    impl<C, M, R> UpdateRoutingControlState<C, M, R>
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
            crate::output::UpdateRoutingControlStateOutput,
            smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateRoutingControlStateInputOperationOutputAlias,
                crate::output::UpdateRoutingControlStateOutput,
                crate::error::UpdateRoutingControlStateError,
                crate::input::UpdateRoutingControlStateInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
        pub fn routing_control_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.routing_control_arn(inp);
            self
        }
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_arn(input);
            self
        }
        /// <p>The state of the routing control. You can set the value to be On or Off.</p>
        pub fn routing_control_state(mut self, inp: crate::model::RoutingControlState) -> Self {
            self.inner = self.inner.routing_control_state(inp);
            self
        }
        pub fn set_routing_control_state(
            mut self,
            input: std::option::Option<crate::model::RoutingControlState>,
        ) -> Self {
            self.inner = self.inner.set_routing_control_state(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateRoutingControlStates<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_routing_control_states_input::Builder,
    }
    impl<C, M, R> UpdateRoutingControlStates<C, M, R>
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
            crate::output::UpdateRoutingControlStatesOutput,
            smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateRoutingControlStatesInputOperationOutputAlias,
                crate::output::UpdateRoutingControlStatesOutput,
                crate::error::UpdateRoutingControlStatesError,
                crate::input::UpdateRoutingControlStatesInputOperationRetryAlias,
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
        /// Appends an item to `UpdateRoutingControlStateEntries`.
        ///
        /// To override the contents of this collection use [`set_update_routing_control_state_entries`](Self::set_update_routing_control_state_entries).
        /// <p>A set of routing control entries that you want to update.</p>
        pub fn update_routing_control_state_entries(
            mut self,
            inp: impl Into<crate::model::UpdateRoutingControlStateEntry>,
        ) -> Self {
            self.inner = self.inner.update_routing_control_state_entries(inp);
            self
        }
        pub fn set_update_routing_control_state_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UpdateRoutingControlStateEntry>>,
        ) -> Self {
            self.inner = self.inner.set_update_routing_control_state_entries(input);
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
