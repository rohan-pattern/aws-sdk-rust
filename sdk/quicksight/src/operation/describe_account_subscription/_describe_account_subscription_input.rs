// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeAccountSubscriptionInput {
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight account.</p>
    #[doc(hidden)]
    pub aws_account_id: std::option::Option<std::string::String>,
}
impl DescribeAccountSubscriptionInput {
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight account.</p>
    pub fn aws_account_id(&self) -> std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
}
impl DescribeAccountSubscriptionInput {
    /// Creates a new builder-style object to manufacture [`DescribeAccountSubscriptionInput`](crate::operation::describe_account_subscription::DescribeAccountSubscriptionInput).
    pub fn builder() -> crate::operation::describe_account_subscription::builders::DescribeAccountSubscriptionInputBuilder{
        crate::operation::describe_account_subscription::builders::DescribeAccountSubscriptionInputBuilder::default()
    }
}

/// A builder for [`DescribeAccountSubscriptionInput`](crate::operation::describe_account_subscription::DescribeAccountSubscriptionInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeAccountSubscriptionInputBuilder {
    pub(crate) aws_account_id: std::option::Option<std::string::String>,
}
impl DescribeAccountSubscriptionInputBuilder {
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight account.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.aws_account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight account.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.aws_account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAccountSubscriptionInput`](crate::operation::describe_account_subscription::DescribeAccountSubscriptionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_account_subscription::DescribeAccountSubscriptionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_account_subscription::DescribeAccountSubscriptionInput {
                aws_account_id: self.aws_account_id,
            },
        )
    }
}
