// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ConflictException(crate::error::ConflictException),
    ConnectorAuthenticationException(crate::error::ConnectorAuthenticationException),
    ConnectorServerException(crate::error::ConnectorServerException),
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    UnsupportedOperationException(crate::error::UnsupportedOperationException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::ConnectorAuthenticationException(inner) => inner.fmt(f),
            Error::ConnectorServerException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::UnsupportedOperationException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateConnectorProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateConnectorProfileError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateConnectorProfileErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateConnectorProfileErrorKind::ConnectorAuthenticationException(
                    inner,
                ) => Error::ConnectorAuthenticationException(inner),
                crate::error::CreateConnectorProfileErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateConnectorProfileErrorKind::ServiceQuotaExceededException(
                    inner,
                ) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateConnectorProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateConnectorProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateFlowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateFlowErrorKind::ConnectorAuthenticationException(inner) => {
                    Error::ConnectorAuthenticationException(inner)
                }
                crate::error::CreateFlowErrorKind::ConnectorServerException(inner) => {
                    Error::ConnectorServerException(inner)
                }
                crate::error::CreateFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateFlowErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CreateFlowErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteConnectorProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteConnectorProfileError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteConnectorProfileErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteConnectorProfileErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteConnectorProfileErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteConnectorProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteFlowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeConnectorEntityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeConnectorEntityError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeConnectorEntityErrorKind::ConnectorAuthenticationException(inner) => Error::ConnectorAuthenticationException(inner),
                crate::error::DescribeConnectorEntityErrorKind::ConnectorServerException(inner) => Error::ConnectorServerException(inner),
                crate::error::DescribeConnectorEntityErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeConnectorEntityErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeConnectorEntityErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DescribeConnectorEntityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeConnectorProfilesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeConnectorProfilesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeConnectorProfilesErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeConnectorProfilesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeConnectorProfilesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeConnectorsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeConnectorsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeConnectorsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeConnectorsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeConnectorsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeFlowExecutionRecordsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeFlowExecutionRecordsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeFlowExecutionRecordsErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeFlowExecutionRecordsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeFlowExecutionRecordsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeFlowExecutionRecordsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListConnectorEntitiesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListConnectorEntitiesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListConnectorEntitiesErrorKind::ConnectorAuthenticationException(
                    inner,
                ) => Error::ConnectorAuthenticationException(inner),
                crate::error::ListConnectorEntitiesErrorKind::ConnectorServerException(inner) => {
                    Error::ConnectorServerException(inner)
                }
                crate::error::ListConnectorEntitiesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListConnectorEntitiesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListConnectorEntitiesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListConnectorEntitiesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListFlowsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListFlowsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListFlowsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListFlowsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListFlowsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StartFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::StartFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartFlowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StartFlowErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::StartFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StopFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::StopFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopFlowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StopFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopFlowErrorKind::UnsupportedOperationException(inner) => {
                    Error::UnsupportedOperationException(inner)
                }
                crate::error::StopFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateConnectorProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateConnectorProfileError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateConnectorProfileErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateConnectorProfileErrorKind::ConnectorAuthenticationException(
                    inner,
                ) => Error::ConnectorAuthenticationException(inner),
                crate::error::UpdateConnectorProfileErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateConnectorProfileErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateConnectorProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateConnectorProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateFlowError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateFlowErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateFlowErrorKind::ConnectorAuthenticationException(inner) => {
                    Error::ConnectorAuthenticationException(inner)
                }
                crate::error::UpdateFlowErrorKind::ConnectorServerException(inner) => {
                    Error::ConnectorServerException(inner)
                }
                crate::error::UpdateFlowErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateFlowErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateFlowErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::UpdateFlowErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateFlowErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
