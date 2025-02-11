// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_configuration_set_event_destination_input(
    input: &crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_configuration_set_event_destination_input::ser_update_configuration_set_event_destination_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_configuration_set_event_destination_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationOutput, crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceErrorException" => crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_configuration_set_event_destination_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationOutput, crate::operation::update_configuration_set_event_destination::UpdateConfigurationSetEventDestinationError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_configuration_set_event_destination::builders::UpdateConfigurationSetEventDestinationOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
