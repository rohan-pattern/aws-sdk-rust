// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_inference_scheduler::_create_inference_scheduler_output::CreateInferenceSchedulerOutputBuilder;

pub use crate::operation::create_inference_scheduler::_create_inference_scheduler_input::CreateInferenceSchedulerInputBuilder;

/// Fluent builder constructing a request to `CreateInferenceScheduler`.
///
/// <p> Creates a scheduled inference. Scheduling an inference is setting up a continuous real-time inference plan to analyze new measurement data. When setting up the schedule, you provide an S3 bucket location for the input data, assign it a delimiter between separate entries in the data, set an offset delay if desired, and set the frequency of inferencing. You must also provide an S3 bucket location for the output data. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateInferenceSchedulerFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_inference_scheduler::builders::CreateInferenceSchedulerInputBuilder
            }
impl CreateInferenceSchedulerFluentBuilder {
    /// Creates a new `CreateInferenceScheduler`.
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
            crate::operation::create_inference_scheduler::CreateInferenceScheduler,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_inference_scheduler::CreateInferenceSchedulerError,
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
        crate::operation::create_inference_scheduler::CreateInferenceSchedulerOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_inference_scheduler::CreateInferenceSchedulerError,
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
    /// <p>The name of the previously trained ML model being used to create the inference scheduler. </p>
    pub fn model_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.model_name(input.into());
        self
    }
    /// <p>The name of the previously trained ML model being used to create the inference scheduler. </p>
    pub fn set_model_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_model_name(input);
        self
    }
    /// <p>The name of the inference scheduler being created. </p>
    pub fn inference_scheduler_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inference_scheduler_name(input.into());
        self
    }
    /// <p>The name of the inference scheduler being created. </p>
    pub fn set_inference_scheduler_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_inference_scheduler_name(input);
        self
    }
    /// <p>The interval (in minutes) of planned delay at the start of each inference segment. For example, if inference is set to run every ten minutes, the delay is set to five minutes and the time is 09:08. The inference scheduler will wake up at the configured interval (which, without a delay configured, would be 09:10) plus the additional five minute delay time (so 09:15) to check your Amazon S3 bucket. The delay provides a buffer for you to upload data at the same frequency, so that you don't have to stop and restart the scheduler when uploading new data.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding the inference process</a>.</p>
    pub fn data_delay_offset_in_minutes(mut self, input: i64) -> Self {
        self.inner = self.inner.data_delay_offset_in_minutes(input);
        self
    }
    /// <p>The interval (in minutes) of planned delay at the start of each inference segment. For example, if inference is set to run every ten minutes, the delay is set to five minutes and the time is 09:08. The inference scheduler will wake up at the configured interval (which, without a delay configured, would be 09:10) plus the additional five minute delay time (so 09:15) to check your Amazon S3 bucket. The delay provides a buffer for you to upload data at the same frequency, so that you don't have to stop and restart the scheduler when uploading new data.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding the inference process</a>.</p>
    pub fn set_data_delay_offset_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_data_delay_offset_in_minutes(input);
        self
    }
    /// <p> How often data is uploaded to the source Amazon S3 bucket for the input data. The value chosen is the length of time between data uploads. For instance, if you select 5 minutes, Amazon Lookout for Equipment will upload the real-time data to the source bucket once every 5 minutes. This frequency also determines how often Amazon Lookout for Equipment runs inference on your data.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding the inference process</a>.</p>
    pub fn data_upload_frequency(mut self, input: crate::types::DataUploadFrequency) -> Self {
        self.inner = self.inner.data_upload_frequency(input);
        self
    }
    /// <p> How often data is uploaded to the source Amazon S3 bucket for the input data. The value chosen is the length of time between data uploads. For instance, if you select 5 minutes, Amazon Lookout for Equipment will upload the real-time data to the source bucket once every 5 minutes. This frequency also determines how often Amazon Lookout for Equipment runs inference on your data.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding the inference process</a>.</p>
    pub fn set_data_upload_frequency(
        mut self,
        input: std::option::Option<crate::types::DataUploadFrequency>,
    ) -> Self {
        self.inner = self.inner.set_data_upload_frequency(input);
        self
    }
    /// <p>Specifies configuration information for the input data for the inference scheduler, including delimiter, format, and dataset location. </p>
    pub fn data_input_configuration(
        mut self,
        input: crate::types::InferenceInputConfiguration,
    ) -> Self {
        self.inner = self.inner.data_input_configuration(input);
        self
    }
    /// <p>Specifies configuration information for the input data for the inference scheduler, including delimiter, format, and dataset location. </p>
    pub fn set_data_input_configuration(
        mut self,
        input: std::option::Option<crate::types::InferenceInputConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_data_input_configuration(input);
        self
    }
    /// <p>Specifies configuration information for the output results for the inference scheduler, including the S3 location for the output. </p>
    pub fn data_output_configuration(
        mut self,
        input: crate::types::InferenceOutputConfiguration,
    ) -> Self {
        self.inner = self.inner.data_output_configuration(input);
        self
    }
    /// <p>Specifies configuration information for the output results for the inference scheduler, including the S3 location for the output. </p>
    pub fn set_data_output_configuration(
        mut self,
        input: std::option::Option<crate::types::InferenceOutputConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_data_output_configuration(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source being used for the inference. </p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source being used for the inference. </p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt inference scheduler data by Amazon Lookout for Equipment. </p>
    pub fn server_side_kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.server_side_kms_key_id(input.into());
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt inference scheduler data by Amazon Lookout for Equipment. </p>
    pub fn set_server_side_kms_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_server_side_kms_key_id(input);
        self
    }
    /// <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags associated with the inference scheduler. </p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Any tags associated with the inference scheduler. </p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
