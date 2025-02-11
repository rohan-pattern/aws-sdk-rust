// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_timeline_event::_create_timeline_event_output::CreateTimelineEventOutputBuilder;

pub use crate::operation::create_timeline_event::_create_timeline_event_input::CreateTimelineEventInputBuilder;

/// Fluent builder constructing a request to `CreateTimelineEvent`.
///
/// <p>Creates a custom timeline event on the incident details page of an incident record. Incident Manager automatically creates timeline events that mark key moments during an incident. You can create custom timeline events to mark important events that Incident Manager can detect automatically.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTimelineEventFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_timeline_event::builders::CreateTimelineEventInputBuilder,
}
impl CreateTimelineEventFluentBuilder {
    /// Creates a new `CreateTimelineEvent`.
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
            crate::operation::create_timeline_event::CreateTimelineEvent,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_timeline_event::CreateTimelineEventError,
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
        crate::operation::create_timeline_event::CreateTimelineEventOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_timeline_event::CreateTimelineEventError,
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
    /// <p>A token that ensures that a client calls the action only once with the specified details.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures that a client calls the action only once with the specified details.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident record that the action adds the incident to.</p>
    pub fn incident_record_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.incident_record_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident record that the action adds the incident to.</p>
    pub fn set_incident_record_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_incident_record_arn(input);
        self
    }
    /// <p>The time that the event occurred.</p>
    pub fn event_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.event_time(input);
        self
    }
    /// <p>The time that the event occurred.</p>
    pub fn set_event_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_event_time(input);
        self
    }
    /// <p>The type of event. You can create timeline events of type <code>Custom Event</code>.</p>
    pub fn event_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.event_type(input.into());
        self
    }
    /// <p>The type of event. You can create timeline events of type <code>Custom Event</code>.</p>
    pub fn set_event_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_event_type(input);
        self
    }
    /// <p>A short description of the event.</p>
    pub fn event_data(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.event_data(input.into());
        self
    }
    /// <p>A short description of the event.</p>
    pub fn set_event_data(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_event_data(input);
        self
    }
    /// Appends an item to `eventReferences`.
    ///
    /// To override the contents of this collection use [`set_event_references`](Self::set_event_references).
    ///
    /// <p>Adds one or more references to the <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with a resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use the table's ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p>
    pub fn event_references(mut self, input: crate::types::EventReference) -> Self {
        self.inner = self.inner.event_references(input);
        self
    }
    /// <p>Adds one or more references to the <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with a resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use the table's ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p>
    pub fn set_event_references(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::EventReference>>,
    ) -> Self {
        self.inner = self.inner.set_event_references(input);
        self
    }
}
