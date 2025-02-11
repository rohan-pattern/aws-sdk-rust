// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies ranges of integer, continuous, and categorical hyperparameters that a hyperparameter tuning job searches. The hyperparameter tuning job launches training jobs with hyperparameter values within these ranges to find the combination of values that result in the training job with the best performance as measured by the objective metric of the hyperparameter tuning job.</p> <note>
/// <p>The maximum number of items specified for <code>Array Members</code> refers to the maximum number of hyperparameters for each range and also the maximum for the hyperparameter tuning job itself. That is, the sum of the number of hyperparameters for all the ranges can't exceed the maximum number specified.</p>
/// </note>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ParameterRanges {
    /// <p>The array of <code>IntegerParameterRange</code> objects that specify ranges of integer hyperparameters that a hyperparameter tuning job searches.</p>
    #[doc(hidden)]
    pub integer_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::IntegerParameterRange>>,
    /// <p>The array of <code>ContinuousParameterRange</code> objects that specify ranges of continuous hyperparameters that a hyperparameter tuning job searches.</p>
    #[doc(hidden)]
    pub continuous_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::ContinuousParameterRange>>,
    /// <p>The array of <code>CategoricalParameterRange</code> objects that specify ranges of categorical hyperparameters that a hyperparameter tuning job searches.</p>
    #[doc(hidden)]
    pub categorical_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::CategoricalParameterRange>>,
}
impl ParameterRanges {
    /// <p>The array of <code>IntegerParameterRange</code> objects that specify ranges of integer hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn integer_parameter_ranges(
        &self,
    ) -> std::option::Option<&[crate::types::IntegerParameterRange]> {
        self.integer_parameter_ranges.as_deref()
    }
    /// <p>The array of <code>ContinuousParameterRange</code> objects that specify ranges of continuous hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn continuous_parameter_ranges(
        &self,
    ) -> std::option::Option<&[crate::types::ContinuousParameterRange]> {
        self.continuous_parameter_ranges.as_deref()
    }
    /// <p>The array of <code>CategoricalParameterRange</code> objects that specify ranges of categorical hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn categorical_parameter_ranges(
        &self,
    ) -> std::option::Option<&[crate::types::CategoricalParameterRange]> {
        self.categorical_parameter_ranges.as_deref()
    }
}
impl ParameterRanges {
    /// Creates a new builder-style object to manufacture [`ParameterRanges`](crate::types::ParameterRanges).
    pub fn builder() -> crate::types::builders::ParameterRangesBuilder {
        crate::types::builders::ParameterRangesBuilder::default()
    }
}

/// A builder for [`ParameterRanges`](crate::types::ParameterRanges).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ParameterRangesBuilder {
    pub(crate) integer_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::IntegerParameterRange>>,
    pub(crate) continuous_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::ContinuousParameterRange>>,
    pub(crate) categorical_parameter_ranges:
        std::option::Option<std::vec::Vec<crate::types::CategoricalParameterRange>>,
}
impl ParameterRangesBuilder {
    /// Appends an item to `integer_parameter_ranges`.
    ///
    /// To override the contents of this collection use [`set_integer_parameter_ranges`](Self::set_integer_parameter_ranges).
    ///
    /// <p>The array of <code>IntegerParameterRange</code> objects that specify ranges of integer hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn integer_parameter_ranges(mut self, input: crate::types::IntegerParameterRange) -> Self {
        let mut v = self.integer_parameter_ranges.unwrap_or_default();
        v.push(input);
        self.integer_parameter_ranges = Some(v);
        self
    }
    /// <p>The array of <code>IntegerParameterRange</code> objects that specify ranges of integer hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn set_integer_parameter_ranges(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IntegerParameterRange>>,
    ) -> Self {
        self.integer_parameter_ranges = input;
        self
    }
    /// Appends an item to `continuous_parameter_ranges`.
    ///
    /// To override the contents of this collection use [`set_continuous_parameter_ranges`](Self::set_continuous_parameter_ranges).
    ///
    /// <p>The array of <code>ContinuousParameterRange</code> objects that specify ranges of continuous hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn continuous_parameter_ranges(
        mut self,
        input: crate::types::ContinuousParameterRange,
    ) -> Self {
        let mut v = self.continuous_parameter_ranges.unwrap_or_default();
        v.push(input);
        self.continuous_parameter_ranges = Some(v);
        self
    }
    /// <p>The array of <code>ContinuousParameterRange</code> objects that specify ranges of continuous hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn set_continuous_parameter_ranges(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ContinuousParameterRange>>,
    ) -> Self {
        self.continuous_parameter_ranges = input;
        self
    }
    /// Appends an item to `categorical_parameter_ranges`.
    ///
    /// To override the contents of this collection use [`set_categorical_parameter_ranges`](Self::set_categorical_parameter_ranges).
    ///
    /// <p>The array of <code>CategoricalParameterRange</code> objects that specify ranges of categorical hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn categorical_parameter_ranges(
        mut self,
        input: crate::types::CategoricalParameterRange,
    ) -> Self {
        let mut v = self.categorical_parameter_ranges.unwrap_or_default();
        v.push(input);
        self.categorical_parameter_ranges = Some(v);
        self
    }
    /// <p>The array of <code>CategoricalParameterRange</code> objects that specify ranges of categorical hyperparameters that a hyperparameter tuning job searches.</p>
    pub fn set_categorical_parameter_ranges(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CategoricalParameterRange>>,
    ) -> Self {
        self.categorical_parameter_ranges = input;
        self
    }
    /// Consumes the builder and constructs a [`ParameterRanges`](crate::types::ParameterRanges).
    pub fn build(self) -> crate::types::ParameterRanges {
        crate::types::ParameterRanges {
            integer_parameter_ranges: self.integer_parameter_ranges,
            continuous_parameter_ranges: self.continuous_parameter_ranges,
            categorical_parameter_ranges: self.categorical_parameter_ranges,
        }
    }
}
