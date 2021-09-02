// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_role_credentials_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRoleCredentialsOutput,
    crate::error::GetRoleCredentialsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetRoleCredentialsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetRoleCredentialsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidRequestException" => crate::error::GetRoleCredentialsError {
            meta: generic,
            kind: crate::error::GetRoleCredentialsErrorKind::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_request_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetRoleCredentialsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::GetRoleCredentialsError {
                meta: generic,
                kind: crate::error::GetRoleCredentialsErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetRoleCredentialsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "TooManyRequestsException" => crate::error::GetRoleCredentialsError {
            meta: generic,
            kind: crate::error::GetRoleCredentialsErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_too_many_requests_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::GetRoleCredentialsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnauthorizedException" => crate::error::GetRoleCredentialsError {
            meta: generic,
            kind: crate::error::GetRoleCredentialsErrorKind::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_unauthorized_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetRoleCredentialsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetRoleCredentialsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_role_credentials_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRoleCredentialsOutput,
    crate::error::GetRoleCredentialsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_role_credentials_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_get_role_credentials(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetRoleCredentialsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_account_roles_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListAccountRolesOutput, crate::error::ListAccountRolesError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListAccountRolesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListAccountRolesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidRequestException" => crate::error::ListAccountRolesError {
            meta: generic,
            kind: crate::error::ListAccountRolesErrorKind::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_request_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListAccountRolesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::ListAccountRolesError {
                meta: generic,
                kind: crate::error::ListAccountRolesErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListAccountRolesError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "TooManyRequestsException" => crate::error::ListAccountRolesError {
            meta: generic,
            kind: crate::error::ListAccountRolesErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_too_many_requests_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::ListAccountRolesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnauthorizedException" => crate::error::ListAccountRolesError {
            meta: generic,
            kind: crate::error::ListAccountRolesErrorKind::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_unauthorized_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListAccountRolesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListAccountRolesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_account_roles_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListAccountRolesOutput, crate::error::ListAccountRolesError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_account_roles_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_list_account_roles(response.body().as_ref(), output)
                .map_err(crate::error::ListAccountRolesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_accounts_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListAccountsOutput, crate::error::ListAccountsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListAccountsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListAccountsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidRequestException" => crate::error::ListAccountsError {
            meta: generic,
            kind: crate::error::ListAccountsErrorKind::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_request_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListAccountsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::ListAccountsError {
                meta: generic,
                kind: crate::error::ListAccountsErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListAccountsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "TooManyRequestsException" => crate::error::ListAccountsError {
            meta: generic,
            kind: crate::error::ListAccountsErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_too_many_requests_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::ListAccountsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnauthorizedException" => crate::error::ListAccountsError {
            meta: generic,
            kind: crate::error::ListAccountsErrorKind::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_unauthorized_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListAccountsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListAccountsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_accounts_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListAccountsOutput, crate::error::ListAccountsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_accounts_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_accounts(response.body().as_ref(), output)
            .map_err(crate::error::ListAccountsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_logout_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::LogoutOutput, crate::error::LogoutError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::LogoutError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::LogoutError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidRequestException" => crate::error::LogoutError {
            meta: generic,
            kind: crate::error::LogoutErrorKind::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_request_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::LogoutError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "TooManyRequestsException" => crate::error::LogoutError {
            meta: generic,
            kind: crate::error::LogoutErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_too_many_requests_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::LogoutError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnauthorizedException" => crate::error::LogoutError {
            meta: generic,
            kind: crate::error::LogoutErrorKind::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_unauthorized_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::LogoutError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::LogoutError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_logout_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::LogoutOutput, crate::error::LogoutError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::logout_output::Builder::default();
        let _ = response;
        output.build()
    })
}
