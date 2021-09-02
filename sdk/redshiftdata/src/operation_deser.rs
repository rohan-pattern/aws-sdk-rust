// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_batch_execute_statement_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::BatchExecuteStatementOutput,
    crate::error::BatchExecuteStatementError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::BatchExecuteStatementError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::BatchExecuteStatementError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ActiveStatementsExceededException" => crate::error::BatchExecuteStatementError {
            meta: generic,
            kind: crate::error::BatchExecuteStatementErrorKind::ActiveStatementsExceededException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::active_statements_exceeded_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_active_statements_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::BatchExecuteStatementError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            ),
        },
        "BatchExecuteStatementException" => crate::error::BatchExecuteStatementError {
            meta: generic,
            kind: crate::error::BatchExecuteStatementErrorKind::BatchExecuteStatementException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::batch_execute_statement_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_batch_execute_statement_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::BatchExecuteStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::BatchExecuteStatementError {
            meta: generic,
            kind: crate::error::BatchExecuteStatementErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::BatchExecuteStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::BatchExecuteStatementError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_batch_execute_statement_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::BatchExecuteStatementOutput,
    crate::error::BatchExecuteStatementError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_execute_statement_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_batch_execute_statement(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::BatchExecuteStatementError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_cancel_statement_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CancelStatementOutput, crate::error::CancelStatementError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::CancelStatementError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CancelStatementError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::CancelStatementError {
            meta: generic,
            kind: crate::error::CancelStatementErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CancelStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::CancelStatementError {
                meta: generic,
                kind: crate::error::CancelStatementErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::CancelStatementError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::CancelStatementError {
            meta: generic,
            kind: crate::error::CancelStatementErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CancelStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::CancelStatementError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_cancel_statement_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CancelStatementOutput, crate::error::CancelStatementError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::cancel_statement_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_cancel_statement(response.body().as_ref(), output)
                .map_err(crate::error::CancelStatementError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_statement_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeStatementOutput, crate::error::DescribeStatementError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeStatementError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeStatementError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DescribeStatementError {
            meta: generic,
            kind: crate::error::DescribeStatementErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::DescribeStatementError {
                meta: generic,
                kind: crate::error::DescribeStatementErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeStatementError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::DescribeStatementError {
            meta: generic,
            kind: crate::error::DescribeStatementErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeStatementError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_statement_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeStatementOutput, crate::error::DescribeStatementError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_statement_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_describe_statement(response.body().as_ref(), output)
                .map_err(crate::error::DescribeStatementError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_table_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeTableOutput, crate::error::DescribeTableError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeTableError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeTableError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DescribeTableError {
            meta: generic,
            kind: crate::error::DescribeTableErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeTableError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeTableError {
            meta: generic,
            kind: crate::error::DescribeTableErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeTableError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeTableError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_table_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeTableOutput, crate::error::DescribeTableError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_table_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_describe_table(response.body().as_ref(), output)
                .map_err(crate::error::DescribeTableError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_execute_statement_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ExecuteStatementError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ExecuteStatementError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ActiveStatementsExceededException" => crate::error::ExecuteStatementError {
            meta: generic,
            kind: crate::error::ExecuteStatementErrorKind::ActiveStatementsExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::active_statements_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_active_statements_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ExecuteStatementException" => crate::error::ExecuteStatementError {
            meta: generic,
            kind: crate::error::ExecuteStatementErrorKind::ExecuteStatementException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::execute_statement_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_execute_statement_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::ExecuteStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ExecuteStatementError {
            meta: generic,
            kind: crate::error::ExecuteStatementErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ExecuteStatementError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ExecuteStatementError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_execute_statement_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::execute_statement_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_execute_statement(response.body().as_ref(), output)
                .map_err(crate::error::ExecuteStatementError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_statement_result_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetStatementResultOutput,
    crate::error::GetStatementResultError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetStatementResultError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetStatementResultError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::GetStatementResultError {
            meta: generic,
            kind: crate::error::GetStatementResultErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetStatementResultError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::GetStatementResultError {
                meta: generic,
                kind: crate::error::GetStatementResultErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetStatementResultError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::GetStatementResultError {
            meta: generic,
            kind: crate::error::GetStatementResultErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetStatementResultError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetStatementResultError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_statement_result_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetStatementResultOutput,
    crate::error::GetStatementResultError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_statement_result_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_get_statement_result(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetStatementResultError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_databases_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListDatabasesOutput, crate::error::ListDatabasesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListDatabasesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListDatabasesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListDatabasesError {
            meta: generic,
            kind: crate::error::ListDatabasesErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListDatabasesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListDatabasesError {
            meta: generic,
            kind: crate::error::ListDatabasesErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListDatabasesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListDatabasesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_databases_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListDatabasesOutput, crate::error::ListDatabasesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_databases_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_list_databases(response.body().as_ref(), output)
                .map_err(crate::error::ListDatabasesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_schemas_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListSchemasOutput, crate::error::ListSchemasError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListSchemasError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListSchemasError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListSchemasError {
            meta: generic,
            kind: crate::error::ListSchemasErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListSchemasError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListSchemasError {
            meta: generic,
            kind: crate::error::ListSchemasErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListSchemasError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListSchemasError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_schemas_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListSchemasOutput, crate::error::ListSchemasError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_schemas_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_schemas(response.body().as_ref(), output)
            .map_err(crate::error::ListSchemasError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_statements_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListStatementsOutput, crate::error::ListStatementsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListStatementsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListStatementsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListStatementsError {
            meta: generic,
            kind: crate::error::ListStatementsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListStatementsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListStatementsError {
            meta: generic,
            kind: crate::error::ListStatementsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListStatementsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListStatementsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_statements_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListStatementsOutput, crate::error::ListStatementsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_statements_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_list_statements(response.body().as_ref(), output)
                .map_err(crate::error::ListStatementsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_tables_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListTablesOutput, crate::error::ListTablesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListTablesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListTablesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListTablesError {
            meta: generic,
            kind: crate::error::ListTablesErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListTablesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListTablesError {
            meta: generic,
            kind: crate::error::ListTablesErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListTablesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListTablesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_tables_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListTablesOutput, crate::error::ListTablesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_tables_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_tables(response.body().as_ref(), output)
            .map_err(crate::error::ListTablesError::unhandled)?;
        output.build()
    })
}
