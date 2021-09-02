// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_group_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeGroupOutput, crate::error::DescribeGroupError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeGroupError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeGroupError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeGroupError {
            meta: generic,
            kind: crate::error::DescribeGroupErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeGroupError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DescribeGroupError {
            meta: generic,
            kind: crate::error::DescribeGroupErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeGroupError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::DescribeGroupError {
                meta: generic,
                kind: crate::error::DescribeGroupErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeGroupError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ThrottlingException" => crate::error::DescribeGroupError {
            meta: generic,
            kind: crate::error::DescribeGroupErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_throttling_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeGroupError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeGroupError {
            meta: generic,
            kind: crate::error::DescribeGroupErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeGroupError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_group_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeGroupOutput, crate::error::DescribeGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_group_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_describe_group(response.body().as_ref(), output)
                .map_err(crate::error::DescribeGroupError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_user_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeUserOutput, crate::error::DescribeUserError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeUserError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeUserError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeUserError {
            meta: generic,
            kind: crate::error::DescribeUserErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeUserError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DescribeUserError {
            meta: generic,
            kind: crate::error::DescribeUserErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeUserError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::DescribeUserError {
                meta: generic,
                kind: crate::error::DescribeUserErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeUserError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ThrottlingException" => crate::error::DescribeUserError {
            meta: generic,
            kind: crate::error::DescribeUserErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_throttling_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeUserError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeUserError {
            meta: generic,
            kind: crate::error::DescribeUserErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeUserError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeUserError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_user_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeUserOutput, crate::error::DescribeUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_user_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_describe_user(response.body().as_ref(), output)
            .map_err(crate::error::DescribeUserError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_groups_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListGroupsOutput, crate::error::ListGroupsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListGroupsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListGroupsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListGroupsError {
            meta: generic,
            kind: crate::error::ListGroupsErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListGroupsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::ListGroupsError {
            meta: generic,
            kind: crate::error::ListGroupsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListGroupsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::ListGroupsError {
                meta: generic,
                kind: crate::error::ListGroupsErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListGroupsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ThrottlingException" => crate::error::ListGroupsError {
            meta: generic,
            kind: crate::error::ListGroupsErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_throttling_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListGroupsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListGroupsError {
            meta: generic,
            kind: crate::error::ListGroupsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListGroupsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListGroupsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_groups_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListGroupsOutput, crate::error::ListGroupsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_groups_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_groups(response.body().as_ref(), output)
            .map_err(crate::error::ListGroupsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_users_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListUsersOutput, crate::error::ListUsersError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListUsersError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListUsersError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListUsersError {
            meta: generic,
            kind: crate::error::ListUsersErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListUsersError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::ListUsersError {
            meta: generic,
            kind: crate::error::ListUsersErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListUsersError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::ListUsersError {
                meta: generic,
                kind: crate::error::ListUsersErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListUsersError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ThrottlingException" => crate::error::ListUsersError {
            meta: generic,
            kind: crate::error::ListUsersErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_throttling_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListUsersError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListUsersError {
            meta: generic,
            kind: crate::error::ListUsersErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListUsersError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListUsersError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_users_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListUsersOutput, crate::error::ListUsersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_users_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_users(response.body().as_ref(), output)
            .map_err(crate::error::ListUsersError::unhandled)?;
        output.build()
    })
}
