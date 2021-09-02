// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ConflictException(crate::error::ConflictException),
    InternalFailureException(crate::error::InternalFailureException),
    InvalidRequestException(crate::error::InvalidRequestException),
    MethodNotAllowedException(crate::error::MethodNotAllowedException),
    RequestEntityTooLargeException(crate::error::RequestEntityTooLargeException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    ThrottlingException(crate::error::ThrottlingException),
    UnauthorizedException(crate::error::UnauthorizedException),
    UnsupportedDocumentEncodingException(crate::error::UnsupportedDocumentEncodingException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::MethodNotAllowedException(inner) => inner.fmt(f),
            Error::RequestEntityTooLargeException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::UnsupportedDocumentEncodingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteThingShadowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteThingShadowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteThingShadowErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::DeleteThingShadowErrorKind::UnsupportedDocumentEncodingException(
                    inner,
                ) => Error::UnsupportedDocumentEncodingException(inner),
                crate::error::DeleteThingShadowErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetRetainedMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetRetainedMessageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRetainedMessageErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::GetRetainedMessageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetThingShadowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetThingShadowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetThingShadowErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::GetThingShadowErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetThingShadowErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::GetThingShadowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetThingShadowErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::GetThingShadowErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetThingShadowErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::GetThingShadowErrorKind::UnsupportedDocumentEncodingException(
                    inner,
                ) => Error::UnsupportedDocumentEncodingException(inner),
                crate::error::GetThingShadowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListNamedShadowsForThingError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListNamedShadowsForThingError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListNamedShadowsForThingErrorKind::InternalFailureException(
                    inner,
                ) => Error::InternalFailureException(inner),
                crate::error::ListNamedShadowsForThingErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListNamedShadowsForThingErrorKind::MethodNotAllowedException(
                    inner,
                ) => Error::MethodNotAllowedException(inner),
                crate::error::ListNamedShadowsForThingErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::ListNamedShadowsForThingErrorKind::ServiceUnavailableException(
                    inner,
                ) => Error::ServiceUnavailableException(inner),
                crate::error::ListNamedShadowsForThingErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListNamedShadowsForThingErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ListNamedShadowsForThingErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListRetainedMessagesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListRetainedMessagesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRetainedMessagesErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ListRetainedMessagesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PublishError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::PublishError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PublishErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::PublishErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::PublishErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::PublishErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::PublishErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateThingShadowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateThingShadowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateThingShadowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::MethodNotAllowedException(inner) => {
                    Error::MethodNotAllowedException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::RequestEntityTooLargeException(inner) => {
                    Error::RequestEntityTooLargeException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::UpdateThingShadowErrorKind::UnsupportedDocumentEncodingException(
                    inner,
                ) => Error::UnsupportedDocumentEncodingException(inner),
                crate::error::UpdateThingShadowErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
