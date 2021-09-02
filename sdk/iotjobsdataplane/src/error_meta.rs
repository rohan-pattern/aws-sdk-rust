// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    CertificateValidationException(crate::error::CertificateValidationException),
    InvalidRequestException(crate::error::InvalidRequestException),
    InvalidStateTransitionException(crate::error::InvalidStateTransitionException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    TerminalStateException(crate::error::TerminalStateException),
    ThrottlingException(crate::error::ThrottlingException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CertificateValidationException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::InvalidStateTransitionException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TerminalStateException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeJobExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeJobExecutionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeJobExecutionErrorKind::CertificateValidationException(
                    inner,
                ) => Error::CertificateValidationException(inner),
                crate::error::DescribeJobExecutionErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DescribeJobExecutionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeJobExecutionErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DescribeJobExecutionErrorKind::TerminalStateException(inner) => {
                    Error::TerminalStateException(inner)
                }
                crate::error::DescribeJobExecutionErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DescribeJobExecutionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetPendingJobExecutionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetPendingJobExecutionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetPendingJobExecutionsErrorKind::CertificateValidationException(
                    inner,
                ) => Error::CertificateValidationException(inner),
                crate::error::GetPendingJobExecutionsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetPendingJobExecutionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::GetPendingJobExecutionsErrorKind::ServiceUnavailableException(
                    inner,
                ) => Error::ServiceUnavailableException(inner),
                crate::error::GetPendingJobExecutionsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetPendingJobExecutionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StartNextPendingJobExecutionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartNextPendingJobExecutionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartNextPendingJobExecutionErrorKind::CertificateValidationException(inner) => Error::CertificateValidationException(inner),
                crate::error::StartNextPendingJobExecutionErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::StartNextPendingJobExecutionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::StartNextPendingJobExecutionErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::StartNextPendingJobExecutionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartNextPendingJobExecutionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateJobExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateJobExecutionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateJobExecutionErrorKind::CertificateValidationException(
                    inner,
                ) => Error::CertificateValidationException(inner),
                crate::error::UpdateJobExecutionErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateJobExecutionErrorKind::InvalidStateTransitionException(
                    inner,
                ) => Error::InvalidStateTransitionException(inner),
                crate::error::UpdateJobExecutionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateJobExecutionErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::UpdateJobExecutionErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateJobExecutionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
