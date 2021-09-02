// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct AssociateMemberAccountError {
    pub kind: AssociateMemberAccountErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum AssociateMemberAccountErrorKind {
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    LimitExceededException(crate::error::LimitExceededException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for AssociateMemberAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AssociateMemberAccountErrorKind::InternalException(_inner) => _inner.fmt(f),
            AssociateMemberAccountErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            AssociateMemberAccountErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            AssociateMemberAccountErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for AssociateMemberAccountError {
    fn code(&self) -> Option<&str> {
        AssociateMemberAccountError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl AssociateMemberAccountError {
    pub fn new(kind: AssociateMemberAccountErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: AssociateMemberAccountErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: AssociateMemberAccountErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateMemberAccountErrorKind::InternalException(_)
        )
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateMemberAccountErrorKind::InvalidInputException(_)
        )
    }
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateMemberAccountErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for AssociateMemberAccountError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AssociateMemberAccountErrorKind::InternalException(_inner) => Some(_inner),
            AssociateMemberAccountErrorKind::InvalidInputException(_inner) => Some(_inner),
            AssociateMemberAccountErrorKind::LimitExceededException(_inner) => Some(_inner),
            AssociateMemberAccountErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct AssociateS3ResourcesError {
    pub kind: AssociateS3ResourcesErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum AssociateS3ResourcesErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    LimitExceededException(crate::error::LimitExceededException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for AssociateS3ResourcesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AssociateS3ResourcesErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            AssociateS3ResourcesErrorKind::InternalException(_inner) => _inner.fmt(f),
            AssociateS3ResourcesErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            AssociateS3ResourcesErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            AssociateS3ResourcesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for AssociateS3ResourcesError {
    fn code(&self) -> Option<&str> {
        AssociateS3ResourcesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl AssociateS3ResourcesError {
    pub fn new(kind: AssociateS3ResourcesErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: AssociateS3ResourcesErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: AssociateS3ResourcesErrorKind::Unhandled(err.into()),
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
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateS3ResourcesErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateS3ResourcesErrorKind::InternalException(_)
        )
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateS3ResourcesErrorKind::InvalidInputException(_)
        )
    }
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            AssociateS3ResourcesErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for AssociateS3ResourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AssociateS3ResourcesErrorKind::AccessDeniedException(_inner) => Some(_inner),
            AssociateS3ResourcesErrorKind::InternalException(_inner) => Some(_inner),
            AssociateS3ResourcesErrorKind::InvalidInputException(_inner) => Some(_inner),
            AssociateS3ResourcesErrorKind::LimitExceededException(_inner) => Some(_inner),
            AssociateS3ResourcesErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DisassociateMemberAccountError {
    pub kind: DisassociateMemberAccountErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DisassociateMemberAccountErrorKind {
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DisassociateMemberAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DisassociateMemberAccountErrorKind::InternalException(_inner) => _inner.fmt(f),
            DisassociateMemberAccountErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            DisassociateMemberAccountErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DisassociateMemberAccountError {
    fn code(&self) -> Option<&str> {
        DisassociateMemberAccountError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DisassociateMemberAccountError {
    pub fn new(kind: DisassociateMemberAccountErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DisassociateMemberAccountErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DisassociateMemberAccountErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_exception(&self) -> bool {
        matches!(
            &self.kind,
            DisassociateMemberAccountErrorKind::InternalException(_)
        )
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            DisassociateMemberAccountErrorKind::InvalidInputException(_)
        )
    }
}
impl std::error::Error for DisassociateMemberAccountError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DisassociateMemberAccountErrorKind::InternalException(_inner) => Some(_inner),
            DisassociateMemberAccountErrorKind::InvalidInputException(_inner) => Some(_inner),
            DisassociateMemberAccountErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DisassociateS3ResourcesError {
    pub kind: DisassociateS3ResourcesErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DisassociateS3ResourcesErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DisassociateS3ResourcesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DisassociateS3ResourcesErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            DisassociateS3ResourcesErrorKind::InternalException(_inner) => _inner.fmt(f),
            DisassociateS3ResourcesErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            DisassociateS3ResourcesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DisassociateS3ResourcesError {
    fn code(&self) -> Option<&str> {
        DisassociateS3ResourcesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DisassociateS3ResourcesError {
    pub fn new(kind: DisassociateS3ResourcesErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DisassociateS3ResourcesErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DisassociateS3ResourcesErrorKind::Unhandled(err.into()),
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
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            DisassociateS3ResourcesErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_exception(&self) -> bool {
        matches!(
            &self.kind,
            DisassociateS3ResourcesErrorKind::InternalException(_)
        )
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            DisassociateS3ResourcesErrorKind::InvalidInputException(_)
        )
    }
}
impl std::error::Error for DisassociateS3ResourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DisassociateS3ResourcesErrorKind::AccessDeniedException(_inner) => Some(_inner),
            DisassociateS3ResourcesErrorKind::InternalException(_inner) => Some(_inner),
            DisassociateS3ResourcesErrorKind::InvalidInputException(_inner) => Some(_inner),
            DisassociateS3ResourcesErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListMemberAccountsError {
    pub kind: ListMemberAccountsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListMemberAccountsErrorKind {
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListMemberAccountsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListMemberAccountsErrorKind::InternalException(_inner) => _inner.fmt(f),
            ListMemberAccountsErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            ListMemberAccountsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for ListMemberAccountsError {
    fn code(&self) -> Option<&str> {
        ListMemberAccountsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListMemberAccountsError {
    pub fn new(kind: ListMemberAccountsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListMemberAccountsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListMemberAccountsErrorKind::Unhandled(err.into()),
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
    pub fn is_internal_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListMemberAccountsErrorKind::InternalException(_)
        )
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListMemberAccountsErrorKind::InvalidInputException(_)
        )
    }
}
impl std::error::Error for ListMemberAccountsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListMemberAccountsErrorKind::InternalException(_inner) => Some(_inner),
            ListMemberAccountsErrorKind::InvalidInputException(_inner) => Some(_inner),
            ListMemberAccountsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListS3ResourcesError {
    pub kind: ListS3ResourcesErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListS3ResourcesErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListS3ResourcesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListS3ResourcesErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            ListS3ResourcesErrorKind::InternalException(_inner) => _inner.fmt(f),
            ListS3ResourcesErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            ListS3ResourcesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for ListS3ResourcesError {
    fn code(&self) -> Option<&str> {
        ListS3ResourcesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListS3ResourcesError {
    pub fn new(kind: ListS3ResourcesErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListS3ResourcesErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListS3ResourcesErrorKind::Unhandled(err.into()),
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
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListS3ResourcesErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_exception(&self) -> bool {
        matches!(&self.kind, ListS3ResourcesErrorKind::InternalException(_))
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListS3ResourcesErrorKind::InvalidInputException(_)
        )
    }
}
impl std::error::Error for ListS3ResourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListS3ResourcesErrorKind::AccessDeniedException(_inner) => Some(_inner),
            ListS3ResourcesErrorKind::InternalException(_inner) => Some(_inner),
            ListS3ResourcesErrorKind::InvalidInputException(_inner) => Some(_inner),
            ListS3ResourcesErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UpdateS3ResourcesError {
    pub kind: UpdateS3ResourcesErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UpdateS3ResourcesErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UpdateS3ResourcesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UpdateS3ResourcesErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            UpdateS3ResourcesErrorKind::InternalException(_inner) => _inner.fmt(f),
            UpdateS3ResourcesErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            UpdateS3ResourcesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for UpdateS3ResourcesError {
    fn code(&self) -> Option<&str> {
        UpdateS3ResourcesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl UpdateS3ResourcesError {
    pub fn new(kind: UpdateS3ResourcesErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UpdateS3ResourcesErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UpdateS3ResourcesErrorKind::Unhandled(err.into()),
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
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateS3ResourcesErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_exception(&self) -> bool {
        matches!(&self.kind, UpdateS3ResourcesErrorKind::InternalException(_))
    }
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateS3ResourcesErrorKind::InvalidInputException(_)
        )
    }
}
impl std::error::Error for UpdateS3ResourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UpdateS3ResourcesErrorKind::AccessDeniedException(_inner) => Some(_inner),
            UpdateS3ResourcesErrorKind::InternalException(_inner) => Some(_inner),
            UpdateS3ResourcesErrorKind::InvalidInputException(_inner) => Some(_inner),
            UpdateS3ResourcesErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The request was rejected because an invalid or out-of-range value was supplied for an
/// input parameter. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidInputException {
    /// Error code for the exception
    pub error_code: std::option::Option<std::string::String>,
    pub message: std::option::Option<std::string::String>,
    /// Field that has invalid input
    pub field_name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidInputException");
        formatter.field("error_code", &self.error_code);
        formatter.field("message", &self.message);
        formatter.field("field_name", &self.field_name);
        formatter.finish()
    }
}
impl InvalidInputException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidInputException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidInputException {}
/// See [`InvalidInputException`](crate::error::InvalidInputException)
pub mod invalid_input_exception {
    /// A builder for [`InvalidInputException`](crate::error::InvalidInputException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_code: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) field_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Error code for the exception
        pub fn error_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_code = Some(input.into());
            self
        }
        pub fn set_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_code = input;
            self
        }
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Field that has invalid input
        pub fn field_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.field_name = Some(input.into());
            self
        }
        pub fn set_field_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.field_name = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidInputException`](crate::error::InvalidInputException)
        pub fn build(self) -> crate::error::InvalidInputException {
            crate::error::InvalidInputException {
                error_code: self.error_code,
                message: self.message,
                field_name: self.field_name,
            }
        }
    }
}
impl InvalidInputException {
    /// Creates a new builder-style object to manufacture [`InvalidInputException`](crate::error::InvalidInputException)
    pub fn builder() -> crate::error::invalid_input_exception::Builder {
        crate::error::invalid_input_exception::Builder::default()
    }
}

/// <p>Internal server error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalException {
    /// Error code for the exception
    pub error_code: std::option::Option<std::string::String>,
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalException");
        formatter.field("error_code", &self.error_code);
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalException {}
/// See [`InternalException`](crate::error::InternalException)
pub mod internal_exception {
    /// A builder for [`InternalException`](crate::error::InternalException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_code: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Error code for the exception
        pub fn error_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_code = Some(input.into());
            self
        }
        pub fn set_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_code = input;
            self
        }
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalException`](crate::error::InternalException)
        pub fn build(self) -> crate::error::InternalException {
            crate::error::InternalException {
                error_code: self.error_code,
                message: self.message,
            }
        }
    }
}
impl InternalException {
    /// Creates a new builder-style object to manufacture [`InternalException`](crate::error::InternalException)
    pub fn builder() -> crate::error::internal_exception::Builder {
        crate::error::internal_exception::Builder::default()
    }
}

/// <p>You do not have required permissions to access the requested resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessDeniedException {
    pub message: std::option::Option<std::string::String>,
    /// Resource type that caused the exception
    pub resource_type: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.field("resource_type", &self.resource_type);
        formatter.finish()
    }
}
impl AccessDeniedException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException)
pub mod access_denied_exception {
    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) resource_type: std::option::Option<std::string::String>,
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
        /// Resource type that caused the exception
        pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_type = Some(input.into());
            self
        }
        pub fn set_resource_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_type = input;
            self
        }
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException)
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message,
                resource_type: self.resource_type,
            }
        }
    }
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException)
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}

/// <p>The request was rejected because it attempted to create resources beyond the current
/// AWS account limits. The error code describes the limit exceeded. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LimitExceededException {
    /// Error code for the exception
    pub error_code: std::option::Option<std::string::String>,
    pub message: std::option::Option<std::string::String>,
    /// Resource type that caused the exception
    pub resource_type: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LimitExceededException");
        formatter.field("error_code", &self.error_code);
        formatter.field("message", &self.message);
        formatter.field("resource_type", &self.resource_type);
        formatter.finish()
    }
}
impl LimitExceededException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededException {}
/// See [`LimitExceededException`](crate::error::LimitExceededException)
pub mod limit_exceeded_exception {
    /// A builder for [`LimitExceededException`](crate::error::LimitExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_code: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) resource_type: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Error code for the exception
        pub fn error_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_code = Some(input.into());
            self
        }
        pub fn set_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_code = input;
            self
        }
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Resource type that caused the exception
        pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_type = Some(input.into());
            self
        }
        pub fn set_resource_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_type = input;
            self
        }
        /// Consumes the builder and constructs a [`LimitExceededException`](crate::error::LimitExceededException)
        pub fn build(self) -> crate::error::LimitExceededException {
            crate::error::LimitExceededException {
                error_code: self.error_code,
                message: self.message,
                resource_type: self.resource_type,
            }
        }
    }
}
impl LimitExceededException {
    /// Creates a new builder-style object to manufacture [`LimitExceededException`](crate::error::LimitExceededException)
    pub fn builder() -> crate::error::limit_exceeded_exception::Builder {
        crate::error::limit_exceeded_exception::Builder::default()
    }
}
