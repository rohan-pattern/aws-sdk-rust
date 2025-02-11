// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_admin_reset_user_password_input(
    input: &crate::operation::admin_reset_user_password::AdminResetUserPasswordInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_admin_reset_user_password_input::ser_admin_reset_user_password_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_admin_reset_user_password_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::admin_reset_user_password::AdminResetUserPasswordOutput,
    crate::operation::admin_reset_user_password::AdminResetUserPasswordError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalErrorException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEmailRoleAccessPolicyException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InvalidEmailRoleAccessPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEmailRoleAccessPolicyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_email_role_access_policy_exception::de_invalid_email_role_access_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidLambdaResponseException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InvalidLambdaResponseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidLambdaResponseExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_lambda_response_exception::de_invalid_lambda_response_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSmsRoleAccessPolicyException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InvalidSmsRoleAccessPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSmsRoleAccessPolicyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_sms_role_access_policy_exception::de_invalid_sms_role_access_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSmsRoleTrustRelationshipException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::InvalidSmsRoleTrustRelationshipException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSmsRoleTrustRelationshipExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_sms_role_trust_relationship_exception::de_invalid_sms_role_trust_relationship_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotAuthorizedException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotAuthorizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnexpectedLambdaException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::UnexpectedLambdaException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnexpectedLambdaExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unexpected_lambda_exception::de_unexpected_lambda_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserLambdaValidationException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::UserLambdaValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UserLambdaValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_lambda_validation_exception::de_user_lambda_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserNotFoundException" => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::UserNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UserNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_not_found_exception::de_user_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_reset_user_password::AdminResetUserPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::admin_reset_user_password::AdminResetUserPasswordError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_admin_reset_user_password_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::admin_reset_user_password::AdminResetUserPasswordOutput,
    crate::operation::admin_reset_user_password::AdminResetUserPasswordError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::admin_reset_user_password::builders::AdminResetUserPasswordOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
