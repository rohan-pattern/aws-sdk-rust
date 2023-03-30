// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `GetEntitlements` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetEntitlementsError {
    /// Kind of error that occurred.
                    pub kind: GetEntitlementsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for GetEntitlementsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetEntitlementsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `GetEntitlements` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetEntitlementsErrorKind {
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceErrorException(crate::error::InternalServiceErrorException),
    /// <p>One or more parameters in your request was invalid.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>The calls to the GetEntitlements API are throttled.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// 
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    /// 
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    /// 
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GetEntitlementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetEntitlementsErrorKind::InternalServiceErrorException(_inner) =>
            _inner.fmt(f)
            ,
            GetEntitlementsErrorKind::InvalidParameterException(_inner) =>
            _inner.fmt(f)
            ,
            GetEntitlementsErrorKind::ThrottlingException(_inner) =>
            _inner.fmt(f)
            ,
            GetEntitlementsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetEntitlementsError {
    fn code(&self) -> Option<&str> {
        GetEntitlementsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetEntitlementsError {
    /// Creates a new `GetEntitlementsError`.
                    pub fn new(kind: GetEntitlementsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `GetEntitlementsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: GetEntitlementsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `GetEntitlementsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: GetEntitlementsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `GetEntitlementsErrorKind::InternalServiceErrorException`.
    pub fn is_internal_service_error_exception(&self) -> bool {
        matches!(&self.kind, GetEntitlementsErrorKind::InternalServiceErrorException(_))
    }
    /// Returns `true` if the error kind is `GetEntitlementsErrorKind::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(&self.kind, GetEntitlementsErrorKind::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `GetEntitlementsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, GetEntitlementsErrorKind::ThrottlingException(_))
    }
}
impl std::error::Error for GetEntitlementsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetEntitlementsErrorKind::InternalServiceErrorException(_inner) =>
            Some(_inner)
            ,
            GetEntitlementsErrorKind::InvalidParameterException(_inner) =>
            Some(_inner)
            ,
            GetEntitlementsErrorKind::ThrottlingException(_inner) =>
            Some(_inner)
            ,
            GetEntitlementsErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>The calls to the GetEntitlements API are throttled.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ThrottlingException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ThrottlingException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException).
pub mod throttling_exception {
    
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException).
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException).
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>One or more parameters in your request was invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidParameterException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InvalidParameterException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InvalidParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidParameterException")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidParameterException {}
/// See [`InvalidParameterException`](crate::error::InvalidParameterException).
pub mod invalid_parameter_exception {
    
    /// A builder for [`InvalidParameterException`](crate::error::InvalidParameterException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InvalidParameterException`](crate::error::InvalidParameterException).
        pub fn build(self) -> crate::error::InvalidParameterException {
            crate::error::InvalidParameterException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InvalidParameterException {
    /// Creates a new builder-style object to manufacture [`InvalidParameterException`](crate::error::InvalidParameterException).
    pub fn builder() -> crate::error::invalid_parameter_exception::Builder {
        crate::error::invalid_parameter_exception::Builder::default()
    }
}

/// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalServiceErrorException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalServiceErrorException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalServiceErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceErrorException")?;
        if let Some(inner_3) = &self.message {
             {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceErrorException {}
/// See [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
pub mod internal_service_error_exception {
    
    /// A builder for [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
        pub fn build(self) -> crate::error::InternalServiceErrorException {
            crate::error::InternalServiceErrorException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InternalServiceErrorException {
    /// Creates a new builder-style object to manufacture [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
    pub fn builder() -> crate::error::internal_service_error_exception::Builder {
        crate::error::internal_service_error_exception::Builder::default()
    }
}

/// 
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
/// 
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
/// 
#[derive(Debug)]
            pub struct Unhandled {
                source: Box<dyn std::error::Error + Send + Sync + 'static>,
            }
            impl Unhandled {
                #[allow(unused)]
                pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
                    Self { source }
                }
            }
            impl std::fmt::Display for Unhandled {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    write!(f, "unhandled error")
                }
            }
            impl std::error::Error for Unhandled {
                fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                    Some(self.source.as_ref() as _)
                }
            }

