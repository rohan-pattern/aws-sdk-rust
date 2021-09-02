// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteHumanLoopError {
    pub kind: DeleteHumanLoopErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteHumanLoopErrorKind {
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DeleteHumanLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteHumanLoopErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DeleteHumanLoopErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DeleteHumanLoopErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            DeleteHumanLoopErrorKind::ValidationException(_inner) => _inner.fmt(f),
            DeleteHumanLoopErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DeleteHumanLoopError {
    fn code(&self) -> Option<&str> {
        DeleteHumanLoopError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteHumanLoopError {
    pub fn new(kind: DeleteHumanLoopErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteHumanLoopErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteHumanLoopErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteHumanLoopErrorKind::InternalServerException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteHumanLoopErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, DeleteHumanLoopErrorKind::ThrottlingException(_))
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, DeleteHumanLoopErrorKind::ValidationException(_))
    }
}
impl std::error::Error for DeleteHumanLoopError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteHumanLoopErrorKind::InternalServerException(_inner) => Some(_inner),
            DeleteHumanLoopErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DeleteHumanLoopErrorKind::ThrottlingException(_inner) => Some(_inner),
            DeleteHumanLoopErrorKind::ValidationException(_inner) => Some(_inner),
            DeleteHumanLoopErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeHumanLoopError {
    pub kind: DescribeHumanLoopErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeHumanLoopErrorKind {
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeHumanLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeHumanLoopErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DescribeHumanLoopErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DescribeHumanLoopErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            DescribeHumanLoopErrorKind::ValidationException(_inner) => _inner.fmt(f),
            DescribeHumanLoopErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DescribeHumanLoopError {
    fn code(&self) -> Option<&str> {
        DescribeHumanLoopError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeHumanLoopError {
    pub fn new(kind: DescribeHumanLoopErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeHumanLoopErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeHumanLoopErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHumanLoopErrorKind::InternalServerException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHumanLoopErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHumanLoopErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHumanLoopErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for DescribeHumanLoopError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeHumanLoopErrorKind::InternalServerException(_inner) => Some(_inner),
            DescribeHumanLoopErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DescribeHumanLoopErrorKind::ThrottlingException(_inner) => Some(_inner),
            DescribeHumanLoopErrorKind::ValidationException(_inner) => Some(_inner),
            DescribeHumanLoopErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListHumanLoopsError {
    pub kind: ListHumanLoopsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListHumanLoopsErrorKind {
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListHumanLoopsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListHumanLoopsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            ListHumanLoopsErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            ListHumanLoopsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            ListHumanLoopsErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ListHumanLoopsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for ListHumanLoopsError {
    fn code(&self) -> Option<&str> {
        ListHumanLoopsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListHumanLoopsError {
    pub fn new(kind: ListHumanLoopsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListHumanLoopsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListHumanLoopsErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListHumanLoopsErrorKind::InternalServerException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListHumanLoopsErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, ListHumanLoopsErrorKind::ThrottlingException(_))
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, ListHumanLoopsErrorKind::ValidationException(_))
    }
}
impl std::error::Error for ListHumanLoopsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListHumanLoopsErrorKind::InternalServerException(_inner) => Some(_inner),
            ListHumanLoopsErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            ListHumanLoopsErrorKind::ThrottlingException(_inner) => Some(_inner),
            ListHumanLoopsErrorKind::ValidationException(_inner) => Some(_inner),
            ListHumanLoopsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartHumanLoopError {
    pub kind: StartHumanLoopErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartHumanLoopErrorKind {
    ConflictException(crate::error::ConflictException),
    InternalServerException(crate::error::InternalServerException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StartHumanLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartHumanLoopErrorKind::ConflictException(_inner) => _inner.fmt(f),
            StartHumanLoopErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            StartHumanLoopErrorKind::ServiceQuotaExceededException(_inner) => _inner.fmt(f),
            StartHumanLoopErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            StartHumanLoopErrorKind::ValidationException(_inner) => _inner.fmt(f),
            StartHumanLoopErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for StartHumanLoopError {
    fn code(&self) -> Option<&str> {
        StartHumanLoopError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartHumanLoopError {
    pub fn new(kind: StartHumanLoopErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartHumanLoopErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartHumanLoopErrorKind::Unhandled(err.into()),
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
    pub fn is_conflict_exception(&self) -> bool {
        matches!(&self.kind, StartHumanLoopErrorKind::ConflictException(_))
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartHumanLoopErrorKind::InternalServerException(_)
        )
    }
    pub fn is_service_quota_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartHumanLoopErrorKind::ServiceQuotaExceededException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, StartHumanLoopErrorKind::ThrottlingException(_))
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, StartHumanLoopErrorKind::ValidationException(_))
    }
}
impl std::error::Error for StartHumanLoopError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartHumanLoopErrorKind::ConflictException(_inner) => Some(_inner),
            StartHumanLoopErrorKind::InternalServerException(_inner) => Some(_inner),
            StartHumanLoopErrorKind::ServiceQuotaExceededException(_inner) => Some(_inner),
            StartHumanLoopErrorKind::ThrottlingException(_inner) => Some(_inner),
            StartHumanLoopErrorKind::ValidationException(_inner) => Some(_inner),
            StartHumanLoopErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StopHumanLoopError {
    pub kind: StopHumanLoopErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StopHumanLoopErrorKind {
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StopHumanLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StopHumanLoopErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            StopHumanLoopErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            StopHumanLoopErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            StopHumanLoopErrorKind::ValidationException(_inner) => _inner.fmt(f),
            StopHumanLoopErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for StopHumanLoopError {
    fn code(&self) -> Option<&str> {
        StopHumanLoopError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl StopHumanLoopError {
    pub fn new(kind: StopHumanLoopErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StopHumanLoopErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StopHumanLoopErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            StopHumanLoopErrorKind::InternalServerException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            StopHumanLoopErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, StopHumanLoopErrorKind::ThrottlingException(_))
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, StopHumanLoopErrorKind::ValidationException(_))
    }
}
impl std::error::Error for StopHumanLoopError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StopHumanLoopErrorKind::InternalServerException(_inner) => Some(_inner),
            StopHumanLoopErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            StopHumanLoopErrorKind::ThrottlingException(_inner) => Some(_inner),
            StopHumanLoopErrorKind::ValidationException(_inner) => Some(_inner),
            StopHumanLoopErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The
/// request isn't valid. Check the syntax and try again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException)
pub mod validation_exception {
    /// A builder for [`ValidationException`](crate::error::ValidationException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException)
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message,
            }
        }
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException)
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>You exceeded
/// the
/// maximum number of requests.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ThrottlingException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException)
pub mod throttling_exception {
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException)
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException)
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>We couldn't find the requested resource. Check that your resources exists and were created
/// in the same AWS Region as your request, and try your request again. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>We couldn't process your request because of an issue with the server. Try again
/// later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException)
pub mod internal_server_exception {
    /// A builder for [`InternalServerException`](crate::error::InternalServerException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException)
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException)
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>You exceeded your service quota. Service quotas, also referred to as limits, are the
/// maximum number of service resources or operations for your AWS account. For a list of
/// Amazon A2I service quotes, see <a href="https://docs.aws.amazon.com/general/latest/gr/a2i.html">Amazon Augmented AI Service Quotes</a>. Delete some resources or request an increase in your
/// service quota. You can request a quota increase using Service Quotas or the AWS Support
/// Center. To request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">AWS Service Quotas</a> in the
/// <i>AWS General Reference</i>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceQuotaExceededException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceQuotaExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceQuotaExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceQuotaExceededException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceQuotaExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceQuotaExceededException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceQuotaExceededException {}
/// See [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
pub mod service_quota_exceeded_exception {
    /// A builder for [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
        pub fn build(self) -> crate::error::ServiceQuotaExceededException {
            crate::error::ServiceQuotaExceededException {
                message: self.message,
            }
        }
    }
}
impl ServiceQuotaExceededException {
    /// Creates a new builder-style object to manufacture [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
    pub fn builder() -> crate::error::service_quota_exceeded_exception::Builder {
        crate::error::service_quota_exceeded_exception::Builder::default()
    }
}

/// <p>Your request has the same name as another active human loop but has different input data. You cannot start two
/// human loops with the same name and different input data.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ConflictException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConflictException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ConflictException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConflictException")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for ConflictException {}
/// See [`ConflictException`](crate::error::ConflictException)
pub mod conflict_exception {
    /// A builder for [`ConflictException`](crate::error::ConflictException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ConflictException`](crate::error::ConflictException)
        pub fn build(self) -> crate::error::ConflictException {
            crate::error::ConflictException {
                message: self.message,
            }
        }
    }
}
impl ConflictException {
    /// Creates a new builder-style object to manufacture [`ConflictException`](crate::error::ConflictException)
    pub fn builder() -> crate::error::conflict_exception::Builder {
        crate::error::conflict_exception::Builder::default()
    }
}
