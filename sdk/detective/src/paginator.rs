// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListGraphs`](crate::operation::ListGraphs)
pub struct ListGraphsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_graphs_input::Builder,
}

impl<C, M, R> ListGraphsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_graphs_input::Builder,
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
            crate::output::ListGraphsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListGraphsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListGraphsInputOperationOutputAlias,
            crate::output::ListGraphsOutput,
            crate::error::ListGraphsError,
            crate::input::ListGraphsInputOperationRetryAlias,
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
                            let new_token = crate::lens::reflens_structure_crate_output_list_graphs_output_next_token(resp);
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

/// Paginator for [`ListInvitations`](crate::operation::ListInvitations)
pub struct ListInvitationsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_invitations_input::Builder,
}

impl<C, M, R> ListInvitationsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_invitations_input::Builder,
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
            crate::output::ListInvitationsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListInvitationsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListInvitationsInputOperationOutputAlias,
            crate::output::ListInvitationsOutput,
            crate::error::ListInvitationsError,
            crate::input::ListInvitationsInputOperationRetryAlias,
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
                            let new_token = crate::lens::reflens_structure_crate_output_list_invitations_output_next_token(resp);
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

/// Paginator for [`ListMembers`](crate::operation::ListMembers)
pub struct ListMembersPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_members_input::Builder,
}

impl<C, M, R> ListMembersPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_members_input::Builder,
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
            crate::output::ListMembersOutput,
            aws_smithy_http::result::SdkError<crate::error::ListMembersError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListMembersInputOperationOutputAlias,
            crate::output::ListMembersOutput,
            crate::error::ListMembersError,
            crate::input::ListMembersInputOperationRetryAlias,
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
                            let new_token = crate::lens::reflens_structure_crate_output_list_members_output_next_token(resp);
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

/// Paginator for [`ListOrganizationAdminAccounts`](crate::operation::ListOrganizationAdminAccounts)
pub struct ListOrganizationAdminAccountsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_organization_admin_accounts_input::Builder,
}

impl<C, M, R> ListOrganizationAdminAccountsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_organization_admin_accounts_input::Builder,
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
            crate::output::ListOrganizationAdminAccountsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListOrganizationAdminAccountsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListOrganizationAdminAccountsInputOperationOutputAlias,
            crate::output::ListOrganizationAdminAccountsOutput,
            crate::error::ListOrganizationAdminAccountsError,
            crate::input::ListOrganizationAdminAccountsInputOperationRetryAlias,
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
                            let new_token = crate::lens::reflens_structure_crate_output_list_organization_admin_accounts_output_next_token(resp);
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
