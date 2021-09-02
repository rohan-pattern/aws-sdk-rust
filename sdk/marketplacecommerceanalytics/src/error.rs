// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GenerateDataSetError {
    pub kind: GenerateDataSetErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GenerateDataSetErrorKind {
    MarketplaceCommerceAnalyticsException(crate::error::MarketplaceCommerceAnalyticsException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GenerateDataSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                _inner.fmt(f)
            }
            GenerateDataSetErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for GenerateDataSetError {
    fn code(&self) -> Option<&str> {
        GenerateDataSetError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl GenerateDataSetError {
    pub fn new(kind: GenerateDataSetErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GenerateDataSetErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GenerateDataSetErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_marketplace_commerce_analytics_exception(&self) -> bool {
        matches!(
            &self.kind,
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_)
        )
    }
}
impl std::error::Error for GenerateDataSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_inner) => Some(_inner),
            GenerateDataSetErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartSupportDataExportError {
    pub kind: StartSupportDataExportErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartSupportDataExportErrorKind {
    MarketplaceCommerceAnalyticsException(crate::error::MarketplaceCommerceAnalyticsException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StartSupportDataExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                _inner.fmt(f)
            }
            StartSupportDataExportErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for StartSupportDataExportError {
    fn code(&self) -> Option<&str> {
        StartSupportDataExportError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartSupportDataExportError {
    pub fn new(kind: StartSupportDataExportErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartSupportDataExportErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartSupportDataExportErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_marketplace_commerce_analytics_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_)
        )
    }
}
impl std::error::Error for StartSupportDataExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                Some(_inner)
            }
            StartSupportDataExportErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// This exception is thrown when an internal service error occurs.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MarketplaceCommerceAnalyticsException {
    /// This message describes details of the error.
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for MarketplaceCommerceAnalyticsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MarketplaceCommerceAnalyticsException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl MarketplaceCommerceAnalyticsException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MarketplaceCommerceAnalyticsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MarketplaceCommerceAnalyticsException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for MarketplaceCommerceAnalyticsException {}
/// See [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException)
pub mod marketplace_commerce_analytics_exception {
    /// A builder for [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// This message describes details of the error.
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException)
        pub fn build(self) -> crate::error::MarketplaceCommerceAnalyticsException {
            crate::error::MarketplaceCommerceAnalyticsException {
                message: self.message,
            }
        }
    }
}
impl MarketplaceCommerceAnalyticsException {
    /// Creates a new builder-style object to manufacture [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException)
    pub fn builder() -> crate::error::marketplace_commerce_analytics_exception::Builder {
        crate::error::marketplace_commerce_analytics_exception::Builder::default()
    }
}
