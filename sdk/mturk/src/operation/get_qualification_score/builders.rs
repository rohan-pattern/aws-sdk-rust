// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_qualification_score::_get_qualification_score_output::GetQualificationScoreOutputBuilder;

pub use crate::operation::get_qualification_score::_get_qualification_score_input::GetQualificationScoreInputBuilder;

/// Fluent builder constructing a request to `GetQualificationScore`.
///
/// <p> The <code>GetQualificationScore</code> operation returns the value of a Worker's Qualification for a given Qualification type. </p>
/// <p> To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the assignment data returned by the <code>ListAssignmentsForHIT</code> operation. </p>
/// <p>Only the owner of a Qualification type can query the value of a Worker's Qualification of that type.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetQualificationScoreFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_qualification_score::builders::GetQualificationScoreInputBuilder,
}
impl GetQualificationScoreFluentBuilder {
    /// Creates a new `GetQualificationScore`.
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
            crate::operation::get_qualification_score::GetQualificationScore,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_qualification_score::GetQualificationScoreError,
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
        crate::operation::get_qualification_score::GetQualificationScoreOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_qualification_score::GetQualificationScoreError,
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
    /// <p>The ID of the QualificationType.</p>
    pub fn qualification_type_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.qualification_type_id(input.into());
        self
    }
    /// <p>The ID of the QualificationType.</p>
    pub fn set_qualification_type_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_qualification_type_id(input);
        self
    }
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    pub fn worker_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.worker_id(input.into());
        self
    }
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    pub fn set_worker_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_worker_id(input);
        self
    }
}
