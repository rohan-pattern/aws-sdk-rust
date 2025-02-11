// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_icd10_cm_inference_jobs::_list_icd10_cm_inference_jobs_output::ListIcd10CmInferenceJobsOutputBuilder;

pub use crate::operation::list_icd10_cm_inference_jobs::_list_icd10_cm_inference_jobs_input::ListIcd10CmInferenceJobsInputBuilder;

/// Fluent builder constructing a request to `ListICD10CMInferenceJobs`.
///
/// <p>Gets a list of InferICD10CM jobs that you have submitted.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListICD10CMInferenceJobsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_icd10_cm_inference_jobs::builders::ListIcd10CmInferenceJobsInputBuilder
            }
impl ListICD10CMInferenceJobsFluentBuilder {
    /// Creates a new `ListICD10CMInferenceJobs`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_icd10_cm_inference_jobs::ListICD10CMInferenceJobs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_icd10_cm_inference_jobs::ListICD10CMInferenceJobsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::list_icd10_cm_inference_jobs::ListIcd10CmInferenceJobsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_icd10_cm_inference_jobs::ListICD10CMInferenceJobsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    pub fn filter(mut self, input: crate::types::ComprehendMedicalAsyncJobFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    pub fn set_filter(
        mut self,
        input: std::option::Option<crate::types::ComprehendMedicalAsyncJobFilter>,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Identifies the next page of results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Identifies the next page of results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
