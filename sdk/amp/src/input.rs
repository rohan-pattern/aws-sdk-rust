// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
pub mod create_workspace_input {
    /// A builder for [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) alias: std::option::Option<std::string::String>,
        pub(crate) client_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// An optional user-assigned alias for this workspace. This alias is for user reference and does not need to be unique.
        pub fn alias(mut self, input: impl Into<std::string::String>) -> Self {
            self.alias = Some(input.into());
            self
        }
        pub fn set_alias(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.alias = input;
            self
        }
        /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_token = Some(input.into());
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_token = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::CreateWorkspaceInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::CreateWorkspaceInput {
                alias: self.alias,
                client_token: self.client_token,
            })
        }
    }
}
#[doc(hidden)]
pub type CreateWorkspaceInputOperationOutputAlias = crate::operation::CreateWorkspace;
#[doc(hidden)]
pub type CreateWorkspaceInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl CreateWorkspaceInput {
    /// Consumes the builder and constructs an Operation<[`CreateWorkspace`](crate::operation::CreateWorkspace)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::CreateWorkspace,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            if self.client_token.is_none() {
                self.client_token = Some(_config.make_token.make_idempotency_token());
            }
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_create_workspace(&self).map_err(
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
                crate::operation::CreateWorkspace::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "CreateWorkspace",
                "amp",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/workspaces").expect("formatting should succeed");
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
    /// Creates a new builder-style object to manufacture [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
    pub fn builder() -> crate::input::create_workspace_input::Builder {
        crate::input::create_workspace_input::Builder::default()
    }
}

/// See [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
pub mod delete_workspace_input {
    /// A builder for [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) workspace_id: std::option::Option<std::string::String>,
        pub(crate) client_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// The ID of the workspace to delete.
        pub fn workspace_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.workspace_id = Some(input.into());
            self
        }
        pub fn set_workspace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.workspace_id = input;
            self
        }
        /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_token = Some(input.into());
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DeleteWorkspaceInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DeleteWorkspaceInput {
                workspace_id: self.workspace_id,
                client_token: self.client_token,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteWorkspaceInputOperationOutputAlias = crate::operation::DeleteWorkspace;
#[doc(hidden)]
pub type DeleteWorkspaceInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DeleteWorkspaceInput {
    /// Consumes the builder and constructs an Operation<[`DeleteWorkspace`](crate::operation::DeleteWorkspace)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DeleteWorkspace,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            if self.client_token.is_none() {
                self.client_token = Some(_config.make_token.make_idempotency_token());
            }
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
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
                crate::operation::DeleteWorkspace::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DeleteWorkspace",
                "amp",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let input_1 = &self.workspace_id;
        let input_1 = input_1
            .as_ref()
            .ok_or(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            })?;
        let workspace_id = smithy_http::label::fmt_string(input_1, false);
        if workspace_id.is_empty() {
            return Err(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            });
        }
        write!(
            output,
            "/workspaces/{workspaceId}",
            workspaceId = workspace_id
        )
        .expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_2) = &self.client_token {
            query.push_kv("clientToken", &smithy_http::query::fmt_string(&inner_2));
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("DELETE").uri(uri))
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
    /// Creates a new builder-style object to manufacture [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
    pub fn builder() -> crate::input::delete_workspace_input::Builder {
        crate::input::delete_workspace_input::Builder::default()
    }
}

/// See [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
pub mod describe_workspace_input {
    /// A builder for [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) workspace_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// The ID of the workspace to describe.
        pub fn workspace_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.workspace_id = Some(input.into());
            self
        }
        pub fn set_workspace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.workspace_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DescribeWorkspaceInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DescribeWorkspaceInput {
                workspace_id: self.workspace_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeWorkspaceInputOperationOutputAlias = crate::operation::DescribeWorkspace;
#[doc(hidden)]
pub type DescribeWorkspaceInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeWorkspaceInput {
    /// Consumes the builder and constructs an Operation<[`DescribeWorkspace`](crate::operation::DescribeWorkspace)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DescribeWorkspace,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
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
                crate::operation::DescribeWorkspace::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DescribeWorkspace",
                "amp",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let input_3 = &self.workspace_id;
        let input_3 = input_3
            .as_ref()
            .ok_or(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            })?;
        let workspace_id = smithy_http::label::fmt_string(input_3, false);
        if workspace_id.is_empty() {
            return Err(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            });
        }
        write!(
            output,
            "/workspaces/{workspaceId}",
            workspaceId = workspace_id
        )
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
        Ok(builder.method("GET").uri(uri))
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
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
    pub fn builder() -> crate::input::describe_workspace_input::Builder {
        crate::input::describe_workspace_input::Builder::default()
    }
}

/// See [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
pub mod list_workspaces_input {
    /// A builder for [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) alias: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
        pub fn alias(mut self, input: impl Into<std::string::String>) -> Self {
            self.alias = Some(input.into());
            self
        }
        pub fn set_alias(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.alias = input;
            self
        }
        /// Maximum results to return in response (default=100, maximum=1000).
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ListWorkspacesInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ListWorkspacesInput {
                next_token: self.next_token,
                alias: self.alias,
                max_results: self.max_results,
            })
        }
    }
}
#[doc(hidden)]
pub type ListWorkspacesInputOperationOutputAlias = crate::operation::ListWorkspaces;
#[doc(hidden)]
pub type ListWorkspacesInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListWorkspacesInput {
    /// Consumes the builder and constructs an Operation<[`ListWorkspaces`](crate::operation::ListWorkspaces)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListWorkspaces,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
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
                crate::operation::ListWorkspaces::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "ListWorkspaces",
                "amp",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/workspaces").expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_4) = &self.next_token {
            query.push_kv("nextToken", &smithy_http::query::fmt_string(&inner_4));
        }
        if let Some(inner_5) = &self.alias {
            query.push_kv("alias", &smithy_http::query::fmt_string(&inner_5));
        }
        if let Some(inner_6) = &self.max_results {
            query.push_kv(
                "maxResults",
                &smithy_types::primitive::Encoder::from(*inner_6).encode(),
            );
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("GET").uri(uri))
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
    /// Creates a new builder-style object to manufacture [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
    pub fn builder() -> crate::input::list_workspaces_input::Builder {
        crate::input::list_workspaces_input::Builder::default()
    }
}

/// See [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput)
pub mod update_workspace_alias_input {
    /// A builder for [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) workspace_id: std::option::Option<std::string::String>,
        pub(crate) alias: std::option::Option<std::string::String>,
        pub(crate) client_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// The ID of the workspace being updated.
        pub fn workspace_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.workspace_id = Some(input.into());
            self
        }
        pub fn set_workspace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.workspace_id = input;
            self
        }
        /// The new alias of the workspace.
        pub fn alias(mut self, input: impl Into<std::string::String>) -> Self {
            self.alias = Some(input.into());
            self
        }
        pub fn set_alias(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.alias = input;
            self
        }
        /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_token = Some(input.into());
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_token = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::UpdateWorkspaceAliasInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::UpdateWorkspaceAliasInput {
                workspace_id: self.workspace_id,
                alias: self.alias,
                client_token: self.client_token,
            })
        }
    }
}
#[doc(hidden)]
pub type UpdateWorkspaceAliasInputOperationOutputAlias = crate::operation::UpdateWorkspaceAlias;
#[doc(hidden)]
pub type UpdateWorkspaceAliasInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl UpdateWorkspaceAliasInput {
    /// Consumes the builder and constructs an Operation<[`UpdateWorkspaceAlias`](crate::operation::UpdateWorkspaceAlias)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::UpdateWorkspaceAlias,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            if self.client_token.is_none() {
                self.client_token = Some(_config.make_token.make_idempotency_token());
            }
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_update_workspace_alias(&self)
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
                crate::operation::UpdateWorkspaceAlias::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "UpdateWorkspaceAlias",
                "amp",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let input_7 = &self.workspace_id;
        let input_7 = input_7
            .as_ref()
            .ok_or(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            })?;
        let workspace_id = smithy_http::label::fmt_string(input_7, false);
        if workspace_id.is_empty() {
            return Err(smithy_http::operation::BuildError::MissingField {
                field: "workspace_id",
                details: "cannot be empty or unset",
            });
        }
        write!(
            output,
            "/workspaces/{workspaceId}/alias",
            workspaceId = workspace_id
        )
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
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput)
    pub fn builder() -> crate::input::update_workspace_alias_input::Builder {
        crate::input::update_workspace_alias_input::Builder::default()
    }
}

/// Represents the input of a ListWorkspaces operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListWorkspacesInput {
    /// Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
    pub next_token: std::option::Option<std::string::String>,
    /// Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
    pub alias: std::option::Option<std::string::String>,
    /// Maximum results to return in response (default=100, maximum=1000).
    pub max_results: std::option::Option<i32>,
}
impl std::fmt::Debug for ListWorkspacesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListWorkspacesInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("alias", &self.alias);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}

/// Represents the input of a CreateWorkspace operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateWorkspaceInput {
    /// An optional user-assigned alias for this workspace. This alias is for user reference and does not need to be unique.
    pub alias: std::option::Option<std::string::String>,
    /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
    pub client_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreateWorkspaceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateWorkspaceInput");
        formatter.field("alias", &self.alias);
        formatter.field("client_token", &self.client_token);
        formatter.finish()
    }
}

/// Represents the input of a DeleteWorkspace operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteWorkspaceInput {
    /// The ID of the workspace to delete.
    pub workspace_id: std::option::Option<std::string::String>,
    /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
    pub client_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DeleteWorkspaceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteWorkspaceInput");
        formatter.field("workspace_id", &self.workspace_id);
        formatter.field("client_token", &self.client_token);
        formatter.finish()
    }
}

/// Represents the input of an UpdateWorkspaceAlias operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateWorkspaceAliasInput {
    /// The ID of the workspace being updated.
    pub workspace_id: std::option::Option<std::string::String>,
    /// The new alias of the workspace.
    pub alias: std::option::Option<std::string::String>,
    /// Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
    pub client_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for UpdateWorkspaceAliasInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateWorkspaceAliasInput");
        formatter.field("workspace_id", &self.workspace_id);
        formatter.field("alias", &self.alias);
        formatter.field("client_token", &self.client_token);
        formatter.finish()
    }
}

/// Represents the input of a DescribeWorkspace operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeWorkspaceInput {
    /// The ID of the workspace to describe.
    pub workspace_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeWorkspaceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeWorkspaceInput");
        formatter.field("workspace_id", &self.workspace_id);
        formatter.finish()
    }
}
