// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListAssociatedRoute53HealthChecks`](crate::operation::ListAssociatedRoute53HealthChecks)
pub struct ListAssociatedRoute53HealthChecksPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_associated_route53_health_checks_input::Builder,
}

impl<C, M, R> ListAssociatedRoute53HealthChecksPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_associated_route53_health_checks_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListAssociatedRoute53HealthChecksOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAssociatedRoute53HealthChecksError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListAssociatedRoute53HealthChecksInputOperationOutputAlias,
            crate::output::ListAssociatedRoute53HealthChecksOutput,
            crate::error::ListAssociatedRoute53HealthChecksError,
            crate::input::ListAssociatedRoute53HealthChecksInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_associated_route53_health_checks_output_next_token(resp);
                            if new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            input.next_token.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListClusters`](crate::operation::ListClusters)
pub struct ListClustersPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_clusters_input::Builder,
}

impl<C, M, R> ListClustersPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_clusters_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListClustersOutput,
            aws_smithy_http::result::SdkError<crate::error::ListClustersError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListClustersInputOperationOutputAlias,
            crate::output::ListClustersOutput,
            crate::error::ListClustersError,
            crate::input::ListClustersInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_clusters_output_next_token(resp);
                            if new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            input.next_token.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListControlPanels`](crate::operation::ListControlPanels)
pub struct ListControlPanelsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_control_panels_input::Builder,
}

impl<C, M, R> ListControlPanelsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_control_panels_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListControlPanelsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListControlPanelsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListControlPanelsInputOperationOutputAlias,
            crate::output::ListControlPanelsOutput,
            crate::error::ListControlPanelsError,
            crate::input::ListControlPanelsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_control_panels_output_next_token(resp);
                            if new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            input.next_token.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListRoutingControls`](crate::operation::ListRoutingControls)
pub struct ListRoutingControlsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_routing_controls_input::Builder,
}

impl<C, M, R> ListRoutingControlsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_routing_controls_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListRoutingControlsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListRoutingControlsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListRoutingControlsInputOperationOutputAlias,
            crate::output::ListRoutingControlsOutput,
            crate::error::ListRoutingControlsError,
            crate::input::ListRoutingControlsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_routing_controls_output_next_token(resp);
                            if new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            input.next_token.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListSafetyRules`](crate::operation::ListSafetyRules)
pub struct ListSafetyRulesPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_safety_rules_input::Builder,
}

impl<C, M, R> ListSafetyRulesPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_safety_rules_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListSafetyRulesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListSafetyRulesError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListSafetyRulesInputOperationOutputAlias,
            crate::output::ListSafetyRulesOutput,
            crate::error::ListSafetyRulesError,
            crate::input::ListSafetyRulesInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_safety_rules_output_next_token(resp);
                            if new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            input.next_token.as_deref().unwrap_or_default().is_empty()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}
