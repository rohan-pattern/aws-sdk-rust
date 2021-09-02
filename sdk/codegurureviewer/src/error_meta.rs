// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    ConflictException(crate::error::ConflictException),
    InternalServerException(crate::error::InternalServerException),
    NotFoundException(crate::error::NotFoundException),
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
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AssociateRepositoryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::AssociateRepositoryError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssociateRepositoryErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::AssociateRepositoryErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::AssociateRepositoryErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::AssociateRepositoryErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::AssociateRepositoryErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::AssociateRepositoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateCodeReviewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateCodeReviewError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateCodeReviewErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateCodeReviewErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeCodeReviewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeCodeReviewError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeCodeReviewErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeCodeReviewErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeCodeReviewErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeCodeReviewErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DescribeCodeReviewErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeCodeReviewErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeRecommendationFeedbackError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeRecommendationFeedbackError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeRecommendationFeedbackErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DescribeRecommendationFeedbackErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeRecommendationFeedbackErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeRecommendationFeedbackErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DescribeRecommendationFeedbackErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DescribeRecommendationFeedbackErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeRepositoryAssociationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeRepositoryAssociationError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeRepositoryAssociationErrorKind::AccessDeniedException(
                    inner,
                ) => Error::AccessDeniedException(inner),
                crate::error::DescribeRepositoryAssociationErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeRepositoryAssociationErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeRepositoryAssociationErrorKind::ThrottlingException(
                    inner,
                ) => Error::ThrottlingException(inner),
                crate::error::DescribeRepositoryAssociationErrorKind::ValidationException(
                    inner,
                ) => Error::ValidationException(inner),
                crate::error::DescribeRepositoryAssociationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DisassociateRepositoryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DisassociateRepositoryError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisassociateRepositoryErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DisassociateRepositoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListCodeReviewsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListCodeReviewsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListCodeReviewsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListCodeReviewsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListCodeReviewsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListCodeReviewsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListCodeReviewsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListRecommendationFeedbackError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListRecommendationFeedbackError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRecommendationFeedbackErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListRecommendationFeedbackErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::ListRecommendationFeedbackErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::ListRecommendationFeedbackErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListRecommendationFeedbackErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListRecommendationFeedbackErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListRecommendationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListRecommendationsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRecommendationsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListRecommendationsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListRecommendationsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListRecommendationsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListRecommendationsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListRecommendationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListRepositoryAssociationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListRepositoryAssociationsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRepositoryAssociationsErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::ListRepositoryAssociationsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListRepositoryAssociationsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListRepositoryAssociationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
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
impl<R> From<smithy_http::result::SdkError<crate::error::PutRecommendationFeedbackError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::PutRecommendationFeedbackError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRecommendationFeedbackErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::PutRecommendationFeedbackErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::PutRecommendationFeedbackErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::PutRecommendationFeedbackErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::PutRecommendationFeedbackErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::PutRecommendationFeedbackErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
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
impl std::error::Error for Error {}
