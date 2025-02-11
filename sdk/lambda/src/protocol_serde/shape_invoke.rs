// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_invoke_headers(
    input: &crate::operation::invoke::InvokeInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.invocation_type {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "invocation_type",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Invocation-Type", header_value);
        }
    }
    if let Some(inner_3) = &input.log_type {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "log_type",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Log-Type", header_value);
        }
    }
    if let Some(inner_5) = &input.client_context {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_context",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Context", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_invoke_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::invoke::InvokeOutput,
    crate::operation::invoke::InvokeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::invoke::InvokeError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::invoke::InvokeError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => {
            crate::operation::invoke::InvokeError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSNotFoundException" => crate::operation::invoke::InvokeError::KmsNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::KmsNotFoundExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_kms_not_found_exception::de_kms_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapStartException" => crate::operation::invoke::InvokeError::SnapStartException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::SnapStartExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_snap_start_exception::de_snap_start_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSubnetIDException" => {
            crate::operation::invoke::InvokeError::InvalidSubnetIdException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidSubnetIdExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_subnet_id_exception::de_invalid_subnet_id_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceException" => {
            crate::operation::invoke::InvokeError::ServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSDisabledException" => crate::operation::invoke::InvokeError::KmsDisabledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::KmsDisabledExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_kms_disabled_exception::de_kms_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EFSIOException" => crate::operation::invoke::InvokeError::EfsioException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EfsioExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_efsio_exception::de_efsio_exception_json_err(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::operation::invoke::InvokeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EFSMountConnectivityException" => {
            crate::operation::invoke::InvokeError::EfsMountConnectivityException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EfsMountConnectivityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_efs_mount_connectivity_exception::de_efs_mount_connectivity_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EFSMountFailureException" => {
            crate::operation::invoke::InvokeError::EfsMountFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EfsMountFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_efs_mount_failure_exception::de_efs_mount_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::invoke::InvokeError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::invoke::InvokeError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotReadyException" => {
            crate::operation::invoke::InvokeError::ResourceNotReadyException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotReadyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_ready_exception::de_resource_not_ready_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidZipFileException" => {
            crate::operation::invoke::InvokeError::InvalidZipFileException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidZipFileExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_zip_file_exception::de_invalid_zip_file_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValueException" => {
            crate::operation::invoke::InvokeError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestContentException" => {
            crate::operation::invoke::InvokeError::InvalidRequestContentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestContentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_content_exception::de_invalid_request_content_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SnapStartTimeoutException" => {
            crate::operation::invoke::InvokeError::SnapStartTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::SnapStartTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snap_start_timeout_exception::de_snap_start_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2ThrottledException" => crate::operation::invoke::InvokeError::Ec2ThrottledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::Ec2ThrottledExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_ec2_throttled_exception::de_ec2_throttled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SubnetIPAddressLimitReachedException" => {
            crate::operation::invoke::InvokeError::SubnetIpAddressLimitReachedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SubnetIpAddressLimitReachedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subnet_ip_address_limit_reached_exception::de_subnet_ip_address_limit_reached_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidSecurityGroupIDException" => {
            crate::operation::invoke::InvokeError::InvalidSecurityGroupIdException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSecurityGroupIdExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_security_group_id_exception::de_invalid_security_group_id_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RequestTooLargeException" => {
            crate::operation::invoke::InvokeError::RequestTooLargeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::RequestTooLargeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_too_large_exception::de_request_too_large_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedMediaTypeException" => {
            crate::operation::invoke::InvokeError::UnsupportedMediaTypeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedMediaTypeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_media_type_exception::de_unsupported_media_type_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EFSMountTimeoutException" => {
            crate::operation::invoke::InvokeError::EfsMountTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EfsMountTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_efs_mount_timeout_exception::de_efs_mount_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ENILimitReachedException" => {
            crate::operation::invoke::InvokeError::EniLimitReachedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EniLimitReachedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_eni_limit_reached_exception::de_eni_limit_reached_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SnapStartNotReadyException" => {
            crate::operation::invoke::InvokeError::SnapStartNotReadyException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::SnapStartNotReadyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snap_start_not_ready_exception::de_snap_start_not_ready_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2UnexpectedException" => {
            crate::operation::invoke::InvokeError::Ec2UnexpectedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::Ec2UnexpectedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_ec2_unexpected_exception::de_ec2_unexpected_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2AccessDeniedException" => {
            crate::operation::invoke::InvokeError::Ec2AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::Ec2AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_ec2_access_denied_exception::de_ec2_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRuntimeException" => {
            crate::operation::invoke::InvokeError::InvalidRuntimeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidRuntimeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_runtime_exception::de_invalid_runtime_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSAccessDeniedException" => {
            crate::operation::invoke::InvokeError::KmsAccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsAccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_access_denied_exception::de_kms_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInvalidStateException" => {
            crate::operation::invoke::InvokeError::KmsInvalidStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceConflictException" => {
            crate::operation::invoke::InvokeError::ResourceConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::invoke::InvokeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::invoke::InvokeError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_invoke_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::invoke::InvokeOutput,
    crate::operation::invoke::InvokeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::invoke::builders::InvokeOutputBuilder::default();
        let _ = response;
        output = output.set_executed_version(
            crate::protocol_serde::shape_invoke_output::de_executed_version_header(
                response.headers(),
            )
            .map_err(|_| {
                crate::operation::invoke::InvokeError::unhandled(
                    "Failed to parse ExecutedVersion from header `X-Amz-Executed-Version",
                )
            })?,
        );
        output = output.set_function_error(
            crate::protocol_serde::shape_invoke_output::de_function_error_header(
                response.headers(),
            )
            .map_err(|_| {
                crate::operation::invoke::InvokeError::unhandled(
                    "Failed to parse FunctionError from header `X-Amz-Function-Error",
                )
            })?,
        );
        output = output.set_log_result(
            crate::protocol_serde::shape_invoke_output::de_log_result_header(response.headers())
                .map_err(|_| {
                    crate::operation::invoke::InvokeError::unhandled(
                        "Failed to parse LogResult from header `X-Amz-Log-Result",
                    )
                })?,
        );
        output = output.set_payload(
            crate::protocol_serde::shape_invoke_output::de_payload_payload(
                response.body().as_ref(),
            )?,
        );
        output = output.set_status_code(Some(response.status().as_u16() as _));
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
