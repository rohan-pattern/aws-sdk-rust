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

/// An ergonomic service client for `AWSDeepSenseRunTimeServiceApi2_0`.
///
/// This client allows ergonomic access to a `AWSDeepSenseRunTimeServiceApi2_0`-shaped service.
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
    pub fn delete_session(&self) -> fluent_builders::DeleteSession<C, M, R> {
        fluent_builders::DeleteSession::new(self.handle.clone())
    }
    pub fn get_session(&self) -> fluent_builders::GetSession<C, M, R> {
        fluent_builders::GetSession::new(self.handle.clone())
    }
    pub fn put_session(&self) -> fluent_builders::PutSession<C, M, R> {
        fluent_builders::PutSession::new(self.handle.clone())
    }
    pub fn recognize_text(&self) -> fluent_builders::RecognizeText<C, M, R> {
        fluent_builders::RecognizeText::new(self.handle.clone())
    }
    pub fn recognize_utterance(&self) -> fluent_builders::RecognizeUtterance<C, M, R> {
        fluent_builders::RecognizeUtterance::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DeleteSession<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_session_input::Builder,
    }
    impl<C, M, R> DeleteSession<C, M, R>
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
            crate::output::DeleteSessionOutput,
            smithy_http::result::SdkError<crate::error::DeleteSessionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteSessionInputOperationOutputAlias,
                crate::output::DeleteSessionOutput,
                crate::error::DeleteSessionError,
                crate::input::DeleteSessionInputOperationRetryAlias,
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
        /// <p>The identifier of the bot that contains the session data.</p>
        pub fn bot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_id(inp);
            self
        }
        pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_id(input);
            self
        }
        /// <p>The alias identifier in use for the bot that contains the session
        /// data.</p>
        pub fn bot_alias_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_alias_id(inp);
            self
        }
        pub fn set_bot_alias_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_alias_id(input);
            self
        }
        /// <p>The locale where the session is in use.</p>
        pub fn locale_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.locale_id(inp);
            self
        }
        pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_locale_id(input);
            self
        }
        /// <p>The identifier of the session to delete.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetSession<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_session_input::Builder,
    }
    impl<C, M, R> GetSession<C, M, R>
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
            crate::output::GetSessionOutput,
            smithy_http::result::SdkError<crate::error::GetSessionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetSessionInputOperationOutputAlias,
                crate::output::GetSessionOutput,
                crate::error::GetSessionError,
                crate::input::GetSessionInputOperationRetryAlias,
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
        /// <p>The identifier of the bot that contains the session data.</p>
        pub fn bot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_id(inp);
            self
        }
        pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_id(input);
            self
        }
        /// <p>The alias identifier in use for the bot that contains the session
        /// data.</p>
        pub fn bot_alias_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_alias_id(inp);
            self
        }
        pub fn set_bot_alias_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_alias_id(input);
            self
        }
        /// <p>The locale where the session is in use.</p>
        pub fn locale_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.locale_id(inp);
            self
        }
        pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_locale_id(input);
            self
        }
        /// <p>The identifier of the session to return.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutSession<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_session_input::Builder,
    }
    impl<C, M, R> PutSession<C, M, R>
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
            crate::output::PutSessionOutput,
            smithy_http::result::SdkError<crate::error::PutSessionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutSessionInputOperationOutputAlias,
                crate::output::PutSessionOutput,
                crate::error::PutSessionError,
                crate::input::PutSessionInputOperationRetryAlias,
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
        /// <p>The identifier of the bot that receives the session data.</p>
        pub fn bot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_id(inp);
            self
        }
        pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_id(input);
            self
        }
        /// <p>The alias identifier of the bot that receives the session
        /// data.</p>
        pub fn bot_alias_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_alias_id(inp);
            self
        }
        pub fn set_bot_alias_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_alias_id(input);
            self
        }
        /// <p>The locale where the session is in use.</p>
        pub fn locale_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.locale_id(inp);
            self
        }
        pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_locale_id(input);
            self
        }
        /// <p>The identifier of the session that receives the session data.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// Appends an item to `messages`.
        ///
        /// To override the contents of this collection use [`set_messages`](Self::set_messages).
        /// <p>A list of messages to send to the user. Messages are sent in the
        /// order that they are defined in the list.</p>
        pub fn messages(mut self, inp: impl Into<crate::model::Message>) -> Self {
            self.inner = self.inner.messages(inp);
            self
        }
        pub fn set_messages(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Message>>,
        ) -> Self {
            self.inner = self.inner.set_messages(input);
            self
        }
        /// <p>Sets the state of the session with the user. You can use this to set
        /// the current intent, attributes, context, and dialog action. Use the
        /// dialog action to determine the next step that Amazon Lex V2 should use in the
        /// conversation with the user.</p>
        pub fn session_state(mut self, inp: crate::model::SessionState) -> Self {
            self.inner = self.inner.session_state(inp);
            self
        }
        pub fn set_session_state(
            mut self,
            input: std::option::Option<crate::model::SessionState>,
        ) -> Self {
            self.inner = self.inner.set_session_state(input);
            self
        }
        /// Adds a key-value pair to `requestAttributes`.
        ///
        /// To override the contents of this collection use [`set_request_attributes`](Self::set_request_attributes).
        /// <p>Request-specific information passed between Amazon Lex V2 and the client
        /// application.</p>
        /// <p>The namespace <code>x-amz-lex:</code> is reserved for special
        /// attributes. Don't create any request attributes with the prefix
        /// <code>x-amz-lex:</code>.</p>
        pub fn request_attributes(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.request_attributes(k, v);
            self
        }
        pub fn set_request_attributes(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_request_attributes(input);
            self
        }
        /// <p>The message that Amazon Lex V2 returns in the response can be either text or
        /// speech depending on the value of this parameter. </p>
        /// <ul>
        /// <li>
        /// <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex V2
        /// returns text in the response.</p>
        /// </li>
        /// </ul>
        pub fn response_content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.response_content_type(inp);
            self
        }
        pub fn set_response_content_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_response_content_type(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct RecognizeText<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::recognize_text_input::Builder,
    }
    impl<C, M, R> RecognizeText<C, M, R>
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
            crate::output::RecognizeTextOutput,
            smithy_http::result::SdkError<crate::error::RecognizeTextError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::RecognizeTextInputOperationOutputAlias,
                crate::output::RecognizeTextOutput,
                crate::error::RecognizeTextError,
                crate::input::RecognizeTextInputOperationRetryAlias,
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
        /// <p>The identifier of the bot that processes the request.</p>
        pub fn bot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_id(inp);
            self
        }
        pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_id(input);
            self
        }
        /// <p>The alias identifier in use for the bot that processes the
        /// request.</p>
        pub fn bot_alias_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_alias_id(inp);
            self
        }
        pub fn set_bot_alias_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_alias_id(input);
            self
        }
        /// <p>The locale where the session is in use.</p>
        pub fn locale_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.locale_id(inp);
            self
        }
        pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_locale_id(input);
            self
        }
        /// <p>The identifier of the user session that is having the
        /// conversation.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// <p>The text that the user entered. Amazon Lex V2 interprets this text.</p>
        pub fn text(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.text(inp);
            self
        }
        pub fn set_text(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_text(input);
            self
        }
        /// <p>The current state of the dialog between the user and the bot.</p>
        pub fn session_state(mut self, inp: crate::model::SessionState) -> Self {
            self.inner = self.inner.session_state(inp);
            self
        }
        pub fn set_session_state(
            mut self,
            input: std::option::Option<crate::model::SessionState>,
        ) -> Self {
            self.inner = self.inner.set_session_state(input);
            self
        }
        /// Adds a key-value pair to `requestAttributes`.
        ///
        /// To override the contents of this collection use [`set_request_attributes`](Self::set_request_attributes).
        /// <p>Request-specific information passed between the client application
        /// and Amazon Lex V2 </p>
        /// <p>The namespace <code>x-amz-lex:</code> is reserved for special
        /// attributes. Don't create any request attributes with the prefix
        /// <code>x-amz-lex:</code>.</p>
        pub fn request_attributes(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.request_attributes(k, v);
            self
        }
        pub fn set_request_attributes(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_request_attributes(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct RecognizeUtterance<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::recognize_utterance_input::Builder,
    }
    impl<C, M, R> RecognizeUtterance<C, M, R>
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
            crate::output::RecognizeUtteranceOutput,
            smithy_http::result::SdkError<crate::error::RecognizeUtteranceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::RecognizeUtteranceInputOperationOutputAlias,
                crate::output::RecognizeUtteranceOutput,
                crate::error::RecognizeUtteranceError,
                crate::input::RecognizeUtteranceInputOperationRetryAlias,
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
        /// <p>The identifier of the bot that should receive the request.</p>
        pub fn bot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_id(inp);
            self
        }
        pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_id(input);
            self
        }
        /// <p>The alias identifier in use for the bot that should receive the
        /// request.</p>
        pub fn bot_alias_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bot_alias_id(inp);
            self
        }
        pub fn set_bot_alias_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bot_alias_id(input);
            self
        }
        /// <p>The locale where the session is in use.</p>
        pub fn locale_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.locale_id(inp);
            self
        }
        pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_locale_id(input);
            self
        }
        /// <p>The identifier of the session in use.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// <p>Sets the state of the session with the user. You can use this to set
        /// the current intent, attributes, context, and dialog action. Use the
        /// dialog action to determine the next step that Amazon Lex V2 should use in the
        /// conversation with the user.</p>
        /// <p>The <code>sessionState</code> field must be compressed using gzip
        /// and then base64 encoded before sending to Amazon Lex V2.</p>
        pub fn session_state(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_state(inp);
            self
        }
        pub fn set_session_state(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_session_state(input);
            self
        }
        /// <p>Request-specific information passed between the client application
        /// and Amazon Lex V2 </p>
        /// <p>The namespace <code>x-amz-lex:</code> is reserved for special
        /// attributes. Don't create any request attributes for prefix
        /// <code>x-amz-lex:</code>.</p>
        /// <p>The <code>requestAttributes</code> field must be compressed using
        /// gzip and then base64 encoded before sending to Amazon Lex V2.</p>
        pub fn request_attributes(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.request_attributes(inp);
            self
        }
        pub fn set_request_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_request_attributes(input);
            self
        }
        /// <p>Indicates the format for audio input or that the content is text.
        /// The header must start with one of the following prefixes:</p>
        /// <ul>
        /// <li>
        /// <p>PCM format, audio data must be in little-endian byte
        /// order.</p>
        /// <ul>
        /// <li>
        /// <p>audio/l16; rate=16000; channels=1</p>
        /// </li>
        /// <li>
        /// <p>audio/x-l16; sample-rate=16000; channel-count=1</p>
        /// </li>
        /// <li>
        /// <p>audio/lpcm; sample-rate=8000; sample-size-bits=16;
        /// channel-count=1; is-big-endian=false</p>
        /// </li>
        /// </ul>
        /// </li>
        /// <li>
        /// <p>Opus format</p>
        /// <ul>
        /// <li>
        /// <p>audio/x-cbr-opus-with-preamble;preamble-size=0;bit-rate=256000;frame-size-milliseconds=4</p>
        /// </li>
        /// </ul>
        /// </li>
        /// <li>
        /// <p>Text format</p>
        /// <ul>
        /// <li>
        /// <p>text/plain; charset=utf-8</p>
        /// </li>
        /// </ul>
        /// </li>
        /// </ul>
        pub fn request_content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.request_content_type(inp);
            self
        }
        pub fn set_request_content_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_request_content_type(input);
            self
        }
        /// <p>The message that Amazon Lex V2 returns in the response can be either text or
        /// speech based on the <code>responseContentType</code> value.</p>
        /// <ul>
        /// <li>
        /// <p>If the value is <code>text/plain;charset=utf-8</code>, Amazon Lex V2
        /// returns text in the response.</p>
        /// </li>
        /// <li>
        /// <p>If the value begins with <code>audio/</code>, Amazon Lex V2 returns
        /// speech in the response. Amazon Lex V2 uses Amazon Polly to generate the speech
        /// using the configuration that you specified in the
        /// <code>requestContentType</code> parameter. For example, if you
        /// specify <code>audio/mpeg</code> as the value, Amazon Lex V2 returns
        /// speech in the MPEG format.</p>
        /// </li>
        /// <li>
        /// <p>If the value is <code>audio/pcm</code>, the speech returned is
        /// <code>audio/pcm</code> at 16 KHz in 16-bit, little-endian
        /// format.</p>
        /// </li>
        /// <li>
        /// <p>The following are the accepted values:</p>
        /// <ul>
        /// <li>
        /// <p>audio/mpeg</p>
        /// </li>
        /// <li>
        /// <p>audio/ogg</p>
        /// </li>
        /// <li>
        /// <p>audio/pcm (16 KHz)</p>
        /// </li>
        /// <li>
        /// <p>audio/* (defaults to mpeg)</p>
        /// </li>
        /// <li>
        /// <p>text/plain; charset=utf-8</p>
        /// </li>
        /// </ul>
        /// </li>
        /// </ul>
        pub fn response_content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.response_content_type(inp);
            self
        }
        pub fn set_response_content_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_response_content_type(input);
            self
        }
        /// <p>User input in PCM or Opus audio format or text format as described
        /// in the <code>requestContentType</code> parameter.</p>
        pub fn input_stream(mut self, inp: smithy_http::byte_stream::ByteStream) -> Self {
            self.inner = self.inner.input_stream(inp);
            self
        }
        pub fn set_input_stream(
            mut self,
            input: std::option::Option<smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.inner = self.inner.set_input_stream(input);
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
