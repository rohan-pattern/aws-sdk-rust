// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_activity_type::_describe_activity_type_output::DescribeActivityTypeOutputBuilder;

pub use crate::operation::describe_activity_type::_describe_activity_type_input::DescribeActivityTypeInputBuilder;

/// Fluent builder constructing a request to `DescribeActivityType`.
///
/// <p>Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type.</p>
/// <p> <b>Access Control</b> </p>
/// <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p>
/// <ul>
/// <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li>
/// <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li>
/// <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p>
/// <ul>
/// <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li>
/// <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li>
/// </ul> </li>
/// </ul>
/// <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeActivityTypeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_activity_type::builders::DescribeActivityTypeInputBuilder,
}
impl DescribeActivityTypeFluentBuilder {
    /// Creates a new `DescribeActivityType`.
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
            crate::operation::describe_activity_type::DescribeActivityType,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_activity_type::DescribeActivityTypeError,
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
        crate::operation::describe_activity_type::DescribeActivityTypeOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_activity_type::DescribeActivityTypeError,
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
    /// <p>The name of the domain in which the activity type is registered.</p>
    pub fn domain(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The name of the domain in which the activity type is registered.</p>
    pub fn set_domain(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The activity type to get information about. Activity types are identified by the <code>name</code> and <code>version</code> that were supplied when the activity was registered.</p>
    pub fn activity_type(mut self, input: crate::types::ActivityType) -> Self {
        self.inner = self.inner.activity_type(input);
        self
    }
    /// <p>The activity type to get information about. Activity types are identified by the <code>name</code> and <code>version</code> that were supplied when the activity was registered.</p>
    pub fn set_activity_type(
        mut self,
        input: std::option::Option<crate::types::ActivityType>,
    ) -> Self {
        self.inner = self.inner.set_activity_type(input);
        self
    }
}
