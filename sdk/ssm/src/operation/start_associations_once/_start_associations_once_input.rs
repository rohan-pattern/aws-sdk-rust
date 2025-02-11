// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartAssociationsOnceInput {
    /// <p>The association IDs that you want to run immediately and only one time.</p>
    #[doc(hidden)]
    pub association_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl StartAssociationsOnceInput {
    /// <p>The association IDs that you want to run immediately and only one time.</p>
    pub fn association_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.association_ids.as_deref()
    }
}
impl StartAssociationsOnceInput {
    /// Creates a new builder-style object to manufacture [`StartAssociationsOnceInput`](crate::operation::start_associations_once::StartAssociationsOnceInput).
    pub fn builder(
    ) -> crate::operation::start_associations_once::builders::StartAssociationsOnceInputBuilder
    {
        crate::operation::start_associations_once::builders::StartAssociationsOnceInputBuilder::default()
    }
}

/// A builder for [`StartAssociationsOnceInput`](crate::operation::start_associations_once::StartAssociationsOnceInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct StartAssociationsOnceInputBuilder {
    pub(crate) association_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl StartAssociationsOnceInputBuilder {
    /// Appends an item to `association_ids`.
    ///
    /// To override the contents of this collection use [`set_association_ids`](Self::set_association_ids).
    ///
    /// <p>The association IDs that you want to run immediately and only one time.</p>
    pub fn association_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.association_ids.unwrap_or_default();
        v.push(input.into());
        self.association_ids = Some(v);
        self
    }
    /// <p>The association IDs that you want to run immediately and only one time.</p>
    pub fn set_association_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.association_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`StartAssociationsOnceInput`](crate::operation::start_associations_once::StartAssociationsOnceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::start_associations_once::StartAssociationsOnceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::start_associations_once::StartAssociationsOnceInput {
                association_ids: self.association_ids,
            },
        )
    }
}
