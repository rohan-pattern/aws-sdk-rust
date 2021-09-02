// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `MarketplaceCommerceAnalytics20150701`.
///
/// This client allows ergonomic access to a `MarketplaceCommerceAnalytics20150701`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn generate_data_set(&self) -> fluent_builders::GenerateDataSet<C, M, R> {
        fluent_builders::GenerateDataSet::new(self.handle.clone())
    }
    pub fn start_support_data_export(&self) -> fluent_builders::StartSupportDataExport<C, M, R> {
        fluent_builders::StartSupportDataExport::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct GenerateDataSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::generate_data_set_input::Builder,
    }
    impl<C, M, R> GenerateDataSet<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GenerateDataSetOutput,
            smithy_http::result::SdkError<crate::error::GenerateDataSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GenerateDataSetInputOperationOutputAlias,
                crate::output::GenerateDataSetOutput,
                crate::error::GenerateDataSetError,
                crate::input::GenerateDataSetInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The desired data set type.</p>
        /// <p>
        /// <ul>
        /// <li>
        /// <strong>customer_subscriber_hourly_monthly_subscriptions</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>customer_subscriber_annual_subscriptions</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_usage_by_instance_type</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_fees</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_free_trial_conversions</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_new_instances</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_new_product_subscribers</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>daily_business_canceled_product_subscribers</strong>
        /// <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>monthly_revenue_billing_and_revenue_data</strong>
        /// <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes metered transactions (e.g. hourly) from one month prior.</p>
        /// </li>
        /// <li>
        /// <strong>monthly_revenue_annual_subscriptions</strong>
        /// <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes up-front software charges (e.g. annual) from one month prior.</p>
        /// </li>
        /// <li>
        /// <strong>monthly_revenue_field_demonstration_usage</strong>
        /// <p>From 2018-03-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>monthly_revenue_flexible_payment_schedule</strong>
        /// <p>From 2018-11-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_product</strong>
        /// <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_instance_hours</strong>
        /// <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_customer_geo</strong>
        /// <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_age_of_uncollected_funds</strong>
        /// <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_age_of_disbursed_funds</strong>
        /// <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_age_of_past_due_funds</strong>
        /// <p>From 2018-04-07 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_uncollected_funds_breakdown</strong>
        /// <p>From 2019-10-04 to present: Available every 30 days by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>sales_compensation_billed_revenue</strong>
        /// <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes metered transactions (e.g. hourly) from one month prior, and up-front software charges (e.g. annual) from one month prior.</p>
        /// </li>
        /// <li>
        /// <strong>us_sales_and_use_tax_records</strong>
        /// <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p>
        /// </li>
        /// <li>
        /// <strong>disbursed_amount_by_product_with_uncollected_funds</strong>
        /// <p>This data set is deprecated. Download related reports from AMMP instead!</p>
        /// </li>
        /// <li>
        /// <strong>customer_profile_by_industry</strong>
        /// <p>This data set is deprecated. Download related reports from AMMP instead!</p>
        /// </li>
        /// <li>
        /// <strong>customer_profile_by_revenue</strong>
        /// <p>This data set is deprecated. Download related reports from AMMP instead!</p>
        /// </li>
        /// <li>
        /// <strong>customer_profile_by_geography</strong>
        /// <p>This data set is deprecated. Download related reports from AMMP instead!</p>
        /// </li>
        /// </ul>
        /// </p>
        pub fn data_set_type(mut self, inp: crate::model::DataSetType) -> Self {
            self.inner = self.inner.data_set_type(inp);
            self
        }
        pub fn set_data_set_type(
            mut self,
            input: std::option::Option<crate::model::DataSetType>,
        ) -> Self {
            self.inner = self.inner.set_data_set_type(input);
            self
        }
        /// The date a data set was published.
        /// For daily data sets, provide a date with day-level granularity for the desired day.
        /// For monthly data sets except those with prefix disbursed_amount, provide a date with month-level granularity for the desired month (the day value will be ignored).
        /// For data sets with prefix disbursed_amount, provide a date with day-level granularity for the desired day. For these data sets we will look backwards in time over the range of 31 days until the first data set is found (the latest one).
        pub fn data_set_publication_date(mut self, inp: smithy_types::Instant) -> Self {
            self.inner = self.inner.data_set_publication_date(inp);
            self
        }
        pub fn set_data_set_publication_date(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.inner = self.inner.set_data_set_publication_date(input);
            self
        }
        /// The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided
        /// AWS services.
        pub fn role_name_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.role_name_arn(inp);
            self
        }
        pub fn set_role_name_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_role_name_arn(input);
            self
        }
        /// The name (friendly name, not ARN) of the destination S3 bucket.
        pub fn destination_s3_bucket_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_s3_bucket_name(inp);
            self
        }
        pub fn set_destination_s3_bucket_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_bucket_name(input);
            self
        }
        /// (Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems.
        /// For example, if given the bucket name "mybucket" and the prefix "myprefix/mydatasets", the output file
        /// "outputfile" would be published to "s3://mybucket/myprefix/mydatasets/outputfile".
        /// If the prefix directory structure does not exist, it will be created.
        /// If no prefix is provided, the data set will be published to the S3 bucket root.
        pub fn destination_s3_prefix(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_s3_prefix(inp);
            self
        }
        pub fn set_destination_s3_prefix(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_prefix(input);
            self
        }
        /// Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an
        /// error has occurred.
        pub fn sns_topic_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sns_topic_arn(inp);
            self
        }
        pub fn set_sns_topic_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sns_topic_arn(input);
            self
        }
        /// Adds a key-value pair to `customerDefinedValues`.
        ///
        /// To override the contents of this collection use [`set_customer_defined_values`](Self::set_customer_defined_values).
        /// (Optional) Key-value pairs which will be returned, unmodified, in the
        /// Amazon SNS notification message and the data set metadata file. These
        /// key-value pairs can be used to correlated responses with tracking
        /// information from other systems.
        pub fn customer_defined_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.customer_defined_values(k, v);
            self
        }
        pub fn set_customer_defined_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_customer_defined_values(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartSupportDataExport<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_support_data_export_input::Builder,
    }
    impl<C, M, R> StartSupportDataExport<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartSupportDataExportOutput,
            smithy_http::result::SdkError<crate::error::StartSupportDataExportError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartSupportDataExportInputOperationOutputAlias,
                crate::output::StartSupportDataExportOutput,
                crate::error::StartSupportDataExportError,
                crate::input::StartSupportDataExportInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Specifies the data set type to be written to the output csv file. The data set types customer_support_contacts_data and
        /// test_customer_support_contacts_data both result in a csv file containing the following fields: Product Id, Product Code, Customer Guid,
        /// Subscription Guid, Subscription Start Date, Organization, AWS Account Id, Given Name, Surname, Telephone Number, Email, Title,
        /// Country Code, ZIP Code, Operation Type, and Operation Time.
        /// </p>
        /// <p>
        /// <ul>
        /// <li><i>customer_support_contacts_data</i> Customer support contact data. The data set will contain all changes (Creates, Updates, and Deletes) to customer support contact data from the date specified in the from_date parameter.</li>
        /// <li><i>test_customer_support_contacts_data</i> An example data set containing static test data in the same format as customer_support_contacts_data</li>
        /// </ul>
        /// </p>
        pub fn data_set_type(mut self, inp: crate::model::SupportDataSetType) -> Self {
            self.inner = self.inner.data_set_type(inp);
            self
        }
        pub fn set_data_set_type(
            mut self,
            input: std::option::Option<crate::model::SupportDataSetType>,
        ) -> Self {
            self.inner = self.inner.set_data_set_type(input);
            self
        }
        /// The start date from which to retrieve the data set in UTC.  This parameter only affects the customer_support_contacts_data data set type.
        pub fn from_date(mut self, inp: smithy_types::Instant) -> Self {
            self.inner = self.inner.from_date(inp);
            self
        }
        pub fn set_from_date(mut self, input: std::option::Option<smithy_types::Instant>) -> Self {
            self.inner = self.inner.set_from_date(input);
            self
        }
        /// The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided
        /// AWS services.
        pub fn role_name_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.role_name_arn(inp);
            self
        }
        pub fn set_role_name_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_role_name_arn(input);
            self
        }
        /// The name (friendly name, not ARN) of the destination S3 bucket.
        pub fn destination_s3_bucket_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_s3_bucket_name(inp);
            self
        }
        pub fn set_destination_s3_bucket_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_bucket_name(input);
            self
        }
        /// (Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems.
        /// For example, if given the bucket name "mybucket" and the prefix "myprefix/mydatasets", the output file
        /// "outputfile" would be published to "s3://mybucket/myprefix/mydatasets/outputfile".
        /// If the prefix directory structure does not exist, it will be created.
        /// If no prefix is provided, the data set will be published to the S3 bucket root.
        pub fn destination_s3_prefix(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_s3_prefix(inp);
            self
        }
        pub fn set_destination_s3_prefix(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_prefix(input);
            self
        }
        /// Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an
        /// error has occurred.
        pub fn sns_topic_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sns_topic_arn(inp);
            self
        }
        pub fn set_sns_topic_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sns_topic_arn(input);
            self
        }
        /// Adds a key-value pair to `customerDefinedValues`.
        ///
        /// To override the contents of this collection use [`set_customer_defined_values`](Self::set_customer_defined_values).
        /// (Optional) Key-value pairs which will be returned, unmodified, in the
        /// Amazon SNS notification message and the data set metadata file.
        pub fn customer_defined_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.customer_defined_values(k, v);
            self
        }
        pub fn set_customer_defined_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_customer_defined_values(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
