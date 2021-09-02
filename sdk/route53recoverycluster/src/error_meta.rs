// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    ConflictException(crate::error::ConflictException),
    EndpointTemporarilyUnavailableException(crate::error::EndpointTemporarilyUnavailableException),
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::EndpointTemporarilyUnavailableException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetRoutingControlStateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetRoutingControlStateError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetRoutingControlStateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::GetRoutingControlStateErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetRoutingControlStateErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetRoutingControlStateErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetRoutingControlStateErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetRoutingControlStateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRoutingControlStateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRoutingControlStatesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
