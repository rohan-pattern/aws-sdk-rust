// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_media(
    op_response: &mut smithy_http::operation::Response,
) -> std::result::Result<crate::output::GetMediaOutput, crate::error::GetMediaError> {
    let response = op_response.http_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_media_output::Builder::default();
        let _ = response;
        output = output.set_content_type(
            crate::http_serde::deser_header_get_media_get_media_output_content_type(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetMediaError::unhandled(
                    "Failed to parse ContentType from header `Content-Type",
                )
            })?,
        );
        output = output.set_payload(Some(
            crate::http_serde::deser_payload_get_media_get_media_output_payload(
                response.body_mut(),
            )?,
        ));
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_media_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetMediaOutput, crate::error::GetMediaError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetMediaError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetMediaError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientLimitExceededException" => {
            crate::error::GetMediaError {
                meta: generic,
                kind: crate::error::GetMediaErrorKind::ClientLimitExceededException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::client_limit_exceeded_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_client_limit_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetMediaError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ConnectionLimitExceededException" => crate::error::GetMediaError {
            meta: generic,
            kind: crate::error::GetMediaErrorKind::ConnectionLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::connection_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_connection_limit_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetMediaError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidArgumentException" => crate::error::GetMediaError {
            meta: generic,
            kind: crate::error::GetMediaErrorKind::InvalidArgumentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_argument_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetMediaError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidEndpointException" => crate::error::GetMediaError {
            meta: generic,
            kind: crate::error::GetMediaErrorKind::InvalidEndpointException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_endpoint_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_endpoint_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetMediaError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "NotAuthorizedException" => crate::error::GetMediaError {
            meta: generic,
            kind: crate::error::GetMediaErrorKind::NotAuthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_authorized_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_not_authorized_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetMediaError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::GetMediaError {
                meta: generic,
                kind: crate::error::GetMediaErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetMediaError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::GetMediaError::generic(generic),
    })
}
