// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_forbidden_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::forbidden_exception::Builder,
) -> Result<crate::error::forbidden_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_gone_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::gone_exception::Builder,
) -> Result<crate::error::gone_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::limit_exceeded_exception::Builder,
) -> Result<crate::error::limit_exceeded_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_get_connection(
    input: &[u8],
    mut builder: crate::output::get_connection_output::Builder,
) -> Result<crate::output::get_connection_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "connectedAt" => {
                        builder = builder.set_connected_at(
                            smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                smithy_types::instant::Format::DateTime,
                            )?,
                        );
                    }
                    "identity" => {
                        builder = builder
                            .set_identity(crate::json_deser::deser_structure_identity(tokens)?);
                    }
                    "lastActiveAt" => {
                        builder = builder.set_last_active_at(
                            smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                smithy_types::instant::Format::DateTime,
                            )?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_payload_too_large_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::payload_too_large_exception::Builder,
) -> Result<crate::error::payload_too_large_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub fn deser_structure_identity<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::Identity>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::Identity::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "sourceIp" => {
                                builder = builder.set_source_ip(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "userAgent" => {
                                builder = builder.set_user_agent(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}
