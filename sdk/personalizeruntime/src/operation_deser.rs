// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_personalized_ranking_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetPersonalizedRankingOutput,
    crate::error::GetPersonalizedRankingError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetPersonalizedRankingError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::GetPersonalizedRankingError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInputException" => crate::error::GetPersonalizedRankingError {
            meta: generic,
            kind: crate::error::GetPersonalizedRankingErrorKind::InvalidInputException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_input_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetPersonalizedRankingError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::GetPersonalizedRankingError {
                meta: generic,
                kind: crate::error::GetPersonalizedRankingErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetPersonalizedRankingError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::GetPersonalizedRankingError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_personalized_ranking_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetPersonalizedRankingOutput,
    crate::error::GetPersonalizedRankingError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_personalized_ranking_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_get_personalized_ranking(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetPersonalizedRankingError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_recommendations_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRecommendationsOutput,
    crate::error::GetRecommendationsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetRecommendationsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetRecommendationsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInputException" => crate::error::GetRecommendationsError {
            meta: generic,
            kind: crate::error::GetRecommendationsErrorKind::InvalidInputException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_input_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetRecommendationsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::GetRecommendationsError {
                meta: generic,
                kind: crate::error::GetRecommendationsErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetRecommendationsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::GetRecommendationsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_recommendations_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRecommendationsOutput,
    crate::error::GetRecommendationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_recommendations_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_get_recommendations(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetRecommendationsError::unhandled)?;
        output.build()
    })
}
