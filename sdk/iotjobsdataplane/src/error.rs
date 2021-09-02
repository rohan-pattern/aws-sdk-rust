// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeJobExecutionError {
    pub kind: DescribeJobExecutionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeJobExecutionErrorKind {
    CertificateValidationException(crate::error::CertificateValidationException),
    InvalidRequestException(crate::error::InvalidRequestException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    TerminalStateException(crate::error::TerminalStateException),
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeJobExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeJobExecutionErrorKind::CertificateValidationException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::InvalidRequestException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::TerminalStateException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            DescribeJobExecutionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DescribeJobExecutionError {
    fn code(&self) -> Option<&str> {
        DescribeJobExecutionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeJobExecutionError {
    pub fn new(kind: DescribeJobExecutionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeJobExecutionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeJobExecutionErrorKind::Unhandled(err.into()),
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
    pub fn is_certificate_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::CertificateValidationException(_)
        )
    }
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::InvalidRequestException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::ServiceUnavailableException(_)
        )
    }
    pub fn is_terminal_state_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::TerminalStateException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeJobExecutionErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for DescribeJobExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeJobExecutionErrorKind::CertificateValidationException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::InvalidRequestException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::TerminalStateException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::ThrottlingException(_inner) => Some(_inner),
            DescribeJobExecutionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetPendingJobExecutionsError {
    pub kind: GetPendingJobExecutionsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetPendingJobExecutionsErrorKind {
    CertificateValidationException(crate::error::CertificateValidationException),
    InvalidRequestException(crate::error::InvalidRequestException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetPendingJobExecutionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetPendingJobExecutionsErrorKind::CertificateValidationException(_inner) => {
                _inner.fmt(f)
            }
            GetPendingJobExecutionsErrorKind::InvalidRequestException(_inner) => _inner.fmt(f),
            GetPendingJobExecutionsErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetPendingJobExecutionsErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            GetPendingJobExecutionsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetPendingJobExecutionsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for GetPendingJobExecutionsError {
    fn code(&self) -> Option<&str> {
        GetPendingJobExecutionsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetPendingJobExecutionsError {
    pub fn new(kind: GetPendingJobExecutionsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetPendingJobExecutionsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetPendingJobExecutionsErrorKind::Unhandled(err.into()),
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
    pub fn is_certificate_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPendingJobExecutionsErrorKind::CertificateValidationException(_)
        )
    }
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPendingJobExecutionsErrorKind::InvalidRequestException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPendingJobExecutionsErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPendingJobExecutionsErrorKind::ServiceUnavailableException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPendingJobExecutionsErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for GetPendingJobExecutionsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetPendingJobExecutionsErrorKind::CertificateValidationException(_inner) => {
                Some(_inner)
            }
            GetPendingJobExecutionsErrorKind::InvalidRequestException(_inner) => Some(_inner),
            GetPendingJobExecutionsErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetPendingJobExecutionsErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            GetPendingJobExecutionsErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetPendingJobExecutionsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartNextPendingJobExecutionError {
    pub kind: StartNextPendingJobExecutionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartNextPendingJobExecutionErrorKind {
    CertificateValidationException(crate::error::CertificateValidationException),
    InvalidRequestException(crate::error::InvalidRequestException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StartNextPendingJobExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartNextPendingJobExecutionErrorKind::CertificateValidationException(_inner) => {
                _inner.fmt(f)
            }
            StartNextPendingJobExecutionErrorKind::InvalidRequestException(_inner) => _inner.fmt(f),
            StartNextPendingJobExecutionErrorKind::ResourceNotFoundException(_inner) => {
                _inner.fmt(f)
            }
            StartNextPendingJobExecutionErrorKind::ServiceUnavailableException(_inner) => {
                _inner.fmt(f)
            }
            StartNextPendingJobExecutionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            StartNextPendingJobExecutionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for StartNextPendingJobExecutionError {
    fn code(&self) -> Option<&str> {
        StartNextPendingJobExecutionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartNextPendingJobExecutionError {
    pub fn new(kind: StartNextPendingJobExecutionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartNextPendingJobExecutionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartNextPendingJobExecutionErrorKind::Unhandled(err.into()),
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
    pub fn is_certificate_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartNextPendingJobExecutionErrorKind::CertificateValidationException(_)
        )
    }
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartNextPendingJobExecutionErrorKind::InvalidRequestException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartNextPendingJobExecutionErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartNextPendingJobExecutionErrorKind::ServiceUnavailableException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartNextPendingJobExecutionErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for StartNextPendingJobExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartNextPendingJobExecutionErrorKind::CertificateValidationException(_inner) => {
                Some(_inner)
            }
            StartNextPendingJobExecutionErrorKind::InvalidRequestException(_inner) => Some(_inner),
            StartNextPendingJobExecutionErrorKind::ResourceNotFoundException(_inner) => {
                Some(_inner)
            }
            StartNextPendingJobExecutionErrorKind::ServiceUnavailableException(_inner) => {
                Some(_inner)
            }
            StartNextPendingJobExecutionErrorKind::ThrottlingException(_inner) => Some(_inner),
            StartNextPendingJobExecutionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UpdateJobExecutionError {
    pub kind: UpdateJobExecutionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UpdateJobExecutionErrorKind {
    CertificateValidationException(crate::error::CertificateValidationException),
    InvalidRequestException(crate::error::InvalidRequestException),
    InvalidStateTransitionException(crate::error::InvalidStateTransitionException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UpdateJobExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UpdateJobExecutionErrorKind::CertificateValidationException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::InvalidRequestException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::InvalidStateTransitionException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            UpdateJobExecutionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for UpdateJobExecutionError {
    fn code(&self) -> Option<&str> {
        UpdateJobExecutionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl UpdateJobExecutionError {
    pub fn new(kind: UpdateJobExecutionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UpdateJobExecutionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UpdateJobExecutionErrorKind::Unhandled(err.into()),
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
    pub fn is_certificate_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::CertificateValidationException(_)
        )
    }
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::InvalidRequestException(_)
        )
    }
    pub fn is_invalid_state_transition_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::InvalidStateTransitionException(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::ResourceNotFoundException(_)
        )
    }
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::ServiceUnavailableException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateJobExecutionErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for UpdateJobExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UpdateJobExecutionErrorKind::CertificateValidationException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::InvalidRequestException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::InvalidStateTransitionException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::ThrottlingException(_inner) => Some(_inner),
            UpdateJobExecutionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The rate exceeds the limit.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    /// <p>The message associated with the exception.</p>
    pub message: std::option::Option<std::string::String>,
    /// <p>The payload associated with the exception.</p>
    pub payload: std::option::Option<smithy_types::Blob>,
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.field("payload", &self.payload);
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
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
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
        pub(crate) payload: std::option::Option<smithy_types::Blob>,
    }
    impl Builder {
        /// <p>The message associated with the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// <p>The payload associated with the exception.</p>
        pub fn payload(mut self, input: smithy_types::Blob) -> Self {
            self.payload = Some(input);
            self
        }
        pub fn set_payload(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.payload = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException)
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
                payload: self.payload,
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

/// <p>The service is temporarily unavailable.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceUnavailableException {
    /// <p>The message for the exception.</p>
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceUnavailableException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceUnavailableException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailableException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailableException {}
/// See [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
pub mod service_unavailable_exception {
    /// A builder for [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The message for the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
        pub fn build(self) -> crate::error::ServiceUnavailableException {
            crate::error::ServiceUnavailableException {
                message: self.message,
            }
        }
    }
}
impl ServiceUnavailableException {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
    pub fn builder() -> crate::error::service_unavailable_exception::Builder {
        crate::error::service_unavailable_exception::Builder::default()
    }
}

/// <p>The specified resource does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    /// <p>The message for the exception.</p>
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
        /// <p>The message for the exception.</p>
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

/// <p>An update attempted to change the job execution to a state that is invalid because of the job execution's
/// current state (for example, an attempt to change a request in state SUCCESS to state IN_PROGRESS). In this
/// case, the body of the error message also contains the executionState field.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidStateTransitionException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidStateTransitionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidStateTransitionException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidStateTransitionException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidStateTransitionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidStateTransitionException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidStateTransitionException {}
/// See [`InvalidStateTransitionException`](crate::error::InvalidStateTransitionException)
pub mod invalid_state_transition_exception {
    /// A builder for [`InvalidStateTransitionException`](crate::error::InvalidStateTransitionException)
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
        /// Consumes the builder and constructs a [`InvalidStateTransitionException`](crate::error::InvalidStateTransitionException)
        pub fn build(self) -> crate::error::InvalidStateTransitionException {
            crate::error::InvalidStateTransitionException {
                message: self.message,
            }
        }
    }
}
impl InvalidStateTransitionException {
    /// Creates a new builder-style object to manufacture [`InvalidStateTransitionException`](crate::error::InvalidStateTransitionException)
    pub fn builder() -> crate::error::invalid_state_transition_exception::Builder {
        crate::error::invalid_state_transition_exception::Builder::default()
    }
}

/// <p>The contents of the request were invalid. For example, this code is returned when an UpdateJobExecution request contains invalid status details. The message contains details about the error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidRequestException {
    /// <p>The message for the exception.</p>
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidRequestException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidRequestException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidRequestException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidRequestException {}
/// See [`InvalidRequestException`](crate::error::InvalidRequestException)
pub mod invalid_request_exception {
    /// A builder for [`InvalidRequestException`](crate::error::InvalidRequestException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The message for the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidRequestException`](crate::error::InvalidRequestException)
        pub fn build(self) -> crate::error::InvalidRequestException {
            crate::error::InvalidRequestException {
                message: self.message,
            }
        }
    }
}
impl InvalidRequestException {
    /// Creates a new builder-style object to manufacture [`InvalidRequestException`](crate::error::InvalidRequestException)
    pub fn builder() -> crate::error::invalid_request_exception::Builder {
        crate::error::invalid_request_exception::Builder::default()
    }
}

/// <p>The certificate is invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CertificateValidationException {
    /// <p>Additional information about the exception.</p>
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CertificateValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CertificateValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl CertificateValidationException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for CertificateValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CertificateValidationException")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for CertificateValidationException {}
/// See [`CertificateValidationException`](crate::error::CertificateValidationException)
pub mod certificate_validation_exception {
    /// A builder for [`CertificateValidationException`](crate::error::CertificateValidationException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Additional information about the exception.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`CertificateValidationException`](crate::error::CertificateValidationException)
        pub fn build(self) -> crate::error::CertificateValidationException {
            crate::error::CertificateValidationException {
                message: self.message,
            }
        }
    }
}
impl CertificateValidationException {
    /// Creates a new builder-style object to manufacture [`CertificateValidationException`](crate::error::CertificateValidationException)
    pub fn builder() -> crate::error::certificate_validation_exception::Builder {
        crate::error::certificate_validation_exception::Builder::default()
    }
}

/// <p>The job is in a terminal state.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TerminalStateException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for TerminalStateException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TerminalStateException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl TerminalStateException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for TerminalStateException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TerminalStateException")?;
        if let Some(inner_7) = &self.message {
            write!(f, ": {}", inner_7)?;
        }
        Ok(())
    }
}
impl std::error::Error for TerminalStateException {}
/// See [`TerminalStateException`](crate::error::TerminalStateException)
pub mod terminal_state_exception {
    /// A builder for [`TerminalStateException`](crate::error::TerminalStateException)
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
        /// Consumes the builder and constructs a [`TerminalStateException`](crate::error::TerminalStateException)
        pub fn build(self) -> crate::error::TerminalStateException {
            crate::error::TerminalStateException {
                message: self.message,
            }
        }
    }
}
impl TerminalStateException {
    /// Creates a new builder-style object to manufacture [`TerminalStateException`](crate::error::TerminalStateException)
    pub fn builder() -> crate::error::terminal_state_exception::Builder {
        crate::error::terminal_state_exception::Builder::default()
    }
}
