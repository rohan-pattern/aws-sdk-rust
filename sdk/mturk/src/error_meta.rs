// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    RequestError(crate::error::RequestError),
    ServiceFault(crate::error::ServiceFault),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(inner) => inner.fmt(f),
            Error::ServiceFault(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AcceptQualificationRequestError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::AcceptQualificationRequestError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AcceptQualificationRequestErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::AcceptQualificationRequestErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::AcceptQualificationRequestErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ApproveAssignmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ApproveAssignmentError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ApproveAssignmentErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ApproveAssignmentErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ApproveAssignmentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AssociateQualificationWithWorkerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::AssociateQualificationWithWorkerError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssociateQualificationWithWorkerErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::AssociateQualificationWithWorkerErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::AssociateQualificationWithWorkerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateAdditionalAssignmentsForHITError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateAdditionalAssignmentsForHITError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateAdditionalAssignmentsForHITErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::CreateAdditionalAssignmentsForHITErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::CreateAdditionalAssignmentsForHITErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateHITError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateHITErrorKind::RequestError(inner) => Error::RequestError(inner),
                crate::error::CreateHITErrorKind::ServiceFault(inner) => Error::ServiceFault(inner),
                crate::error::CreateHITErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateHITTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateHITTypeError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateHITTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::CreateHITTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::CreateHITTypeErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateHITWithHITTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateHITWithHITTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateHITWithHITTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::CreateHITWithHITTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::CreateHITWithHITTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateQualificationTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::CreateQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::CreateQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateWorkerBlockError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateWorkerBlockError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateWorkerBlockErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::CreateWorkerBlockErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::CreateWorkerBlockErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteHITError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteHITErrorKind::RequestError(inner) => Error::RequestError(inner),
                crate::error::DeleteHITErrorKind::ServiceFault(inner) => Error::ServiceFault(inner),
                crate::error::DeleteHITErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteQualificationTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::DeleteQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::DeleteQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteWorkerBlockError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteWorkerBlockError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteWorkerBlockErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::DeleteWorkerBlockErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::DeleteWorkerBlockErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<smithy_http::result::SdkError<crate::error::DisassociateQualificationFromWorkerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<
            crate::error::DisassociateQualificationFromWorkerError,
            R,
        >,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisassociateQualificationFromWorkerErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::DisassociateQualificationFromWorkerErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::DisassociateQualificationFromWorkerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetAccountBalanceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetAccountBalanceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAccountBalanceErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::GetAccountBalanceErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::GetAccountBalanceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetAssignmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetAssignmentError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAssignmentErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::GetAssignmentErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::GetAssignmentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetFileUploadURLError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetFileUploadURLError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetFileUploadURLErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::GetFileUploadURLErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::GetFileUploadURLErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetHITError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetHITErrorKind::RequestError(inner) => Error::RequestError(inner),
                crate::error::GetHITErrorKind::ServiceFault(inner) => Error::ServiceFault(inner),
                crate::error::GetHITErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetQualificationScoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetQualificationScoreError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQualificationScoreErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::GetQualificationScoreErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::GetQualificationScoreErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetQualificationTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::GetQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::GetQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListAssignmentsForHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListAssignmentsForHITError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListAssignmentsForHITErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListAssignmentsForHITErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListAssignmentsForHITErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListBonusPaymentsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListBonusPaymentsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListBonusPaymentsErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListBonusPaymentsErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListBonusPaymentsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListHITsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListHITsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListHITsErrorKind::RequestError(inner) => Error::RequestError(inner),
                crate::error::ListHITsErrorKind::ServiceFault(inner) => Error::ServiceFault(inner),
                crate::error::ListHITsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListHITsForQualificationTypeError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListHITsForQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListHITsForQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListHITsForQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListHITsForQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListQualificationRequestsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListQualificationRequestsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQualificationRequestsErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListQualificationRequestsErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListQualificationRequestsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListQualificationTypesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListQualificationTypesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQualificationTypesErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListQualificationTypesErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListQualificationTypesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListReviewableHITsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListReviewableHITsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListReviewableHITsErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListReviewableHITsErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListReviewableHITsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListReviewPolicyResultsForHITError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListReviewPolicyResultsForHITError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListReviewPolicyResultsForHITErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListReviewPolicyResultsForHITErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListReviewPolicyResultsForHITErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListWorkerBlocksError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListWorkerBlocksError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListWorkerBlocksErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListWorkerBlocksErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListWorkerBlocksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListWorkersWithQualificationTypeError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListWorkersWithQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListWorkersWithQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::ListWorkersWithQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::ListWorkersWithQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::NotifyWorkersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::NotifyWorkersError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::NotifyWorkersErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::NotifyWorkersErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::NotifyWorkersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::RejectAssignmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::RejectAssignmentError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RejectAssignmentErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::RejectAssignmentErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::RejectAssignmentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::RejectQualificationRequestError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::RejectQualificationRequestError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RejectQualificationRequestErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::RejectQualificationRequestErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::RejectQualificationRequestErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SendBonusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::SendBonusError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendBonusErrorKind::RequestError(inner) => Error::RequestError(inner),
                crate::error::SendBonusErrorKind::ServiceFault(inner) => Error::ServiceFault(inner),
                crate::error::SendBonusErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SendTestEventNotificationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::SendTestEventNotificationError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendTestEventNotificationErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::SendTestEventNotificationErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::SendTestEventNotificationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateExpirationForHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateExpirationForHITError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateExpirationForHITErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::UpdateExpirationForHITErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::UpdateExpirationForHITErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateHITReviewStatusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateHITReviewStatusError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateHITReviewStatusErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::UpdateHITReviewStatusErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::UpdateHITReviewStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateHITTypeOfHITError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateHITTypeOfHITError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateHITTypeOfHITErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::UpdateHITTypeOfHITErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::UpdateHITTypeOfHITErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateNotificationSettingsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateNotificationSettingsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateNotificationSettingsErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::UpdateNotificationSettingsErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::UpdateNotificationSettingsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateQualificationTypeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateQualificationTypeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateQualificationTypeErrorKind::RequestError(inner) => {
                    Error::RequestError(inner)
                }
                crate::error::UpdateQualificationTypeErrorKind::ServiceFault(inner) => {
                    Error::ServiceFault(inner)
                }
                crate::error::UpdateQualificationTypeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
