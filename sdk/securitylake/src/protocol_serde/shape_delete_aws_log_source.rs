// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_aws_log_source_input(
    input: &crate::operation::delete_aws_log_source::DeleteAwsLogSourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_aws_log_source_input::ser_delete_aws_log_source_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_aws_log_source_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_aws_log_source::DeleteAwsLogSourceOutput,
    crate::operation::delete_aws_log_source::DeleteAwsLogSourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccountNotFoundException" => crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::AccountNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccountNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_not_found_exception::de_account_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_aws_log_source_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_aws_log_source::DeleteAwsLogSourceOutput,
    crate::operation::delete_aws_log_source::DeleteAwsLogSourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_aws_log_source::de_delete_aws_log_source(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::delete_aws_log_source::DeleteAwsLogSourceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_delete_aws_log_source(
    value: &[u8],
    mut builder: crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceOutputBuilder,
) -> Result<
    crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "failed" => {
                        builder = builder.set_failed(
                            crate::protocol_serde::shape_account_list::de_account_list(tokens)?,
                        );
                    }
                    "processing" => {
                        builder = builder.set_processing(
                            crate::protocol_serde::shape_account_list::de_account_list(tokens)?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
