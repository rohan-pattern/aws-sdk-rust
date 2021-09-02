// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_content_identification_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MedicalContentIdentificationType>,
    smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amzn-transcribe-content-identification-type")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_enable_channel_identification(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-enable-channel-identification")
        .iter();
    let var_1 = smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_1.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_language_code(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::LanguageCode>,
    smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amzn-transcribe-language-code").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_media_encoding(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MediaEncoding>,
    smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amzn-transcribe-media-encoding")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_media_sample_rate_hertz(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amzn-transcribe-sample-rate").iter();
    let var_2 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_2.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_number_of_channels(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-number-of-channels")
        .iter();
    let var_3 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_3.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_3.len()
        )))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_request_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("x-amzn-request-id").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("x-amzn-transcribe-session-id").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_show_speaker_label(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-show-speaker-label")
        .iter();
    let var_4 = smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_4.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_4.len()
        )))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_specialty(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::Specialty>,
    smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amzn-transcribe-specialty").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_start_medical_stream_transcription_start_medical_stream_transcription_output_transcript_result_stream(
    body: &mut smithy_http::body::SdkBody,
) -> std::result::Result<
    smithy_http::event_stream::Receiver<
        crate::model::MedicalTranscriptResultStream,
        crate::error::StartMedicalStreamTranscriptionError,
    >,
    crate::error::StartMedicalStreamTranscriptionError,
> {
    let unmarshaller = crate::event_stream_serde::MedicalTranscriptResultStreamUnmarshaller::new();
    let body = std::mem::replace(body, smithy_http::body::SdkBody::taken());
    Ok(smithy_http::event_stream::Receiver::new(unmarshaller, body))
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<crate::model::Type>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amzn-transcribe-type").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_vocabulary_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map
        .get_all("x-amzn-transcribe-vocabulary-name")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_enable_channel_identification(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-enable-channel-identification")
        .iter();
    let var_5 = smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_5.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_5.len()
        )))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_enable_partial_results_stabilization(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-enable-partial-results-stabilization")
        .iter();
    let var_6 = smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_6.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_6.len()
        )))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_language_code(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::LanguageCode>,
    smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amzn-transcribe-language-code").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_media_encoding(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MediaEncoding>,
    smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amzn-transcribe-media-encoding")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_media_sample_rate_hertz(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amzn-transcribe-sample-rate").iter();
    let var_7 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_7.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_7.len()
        )))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_number_of_channels(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-number-of-channels")
        .iter();
    let var_8 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_8.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_8.len()
        )))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_partial_results_stability(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::PartialResultsStability>,
    smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amzn-transcribe-partial-results-stability")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_request_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("x-amzn-request-id").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("x-amzn-transcribe-session-id").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_show_speaker_label(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amzn-transcribe-show-speaker-label")
        .iter();
    let var_9 = smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_9.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_9.len()
        )))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_payload_start_stream_transcription_start_stream_transcription_output_transcript_result_stream(
    body: &mut smithy_http::body::SdkBody,
) -> std::result::Result<
    smithy_http::event_stream::Receiver<
        crate::model::TranscriptResultStream,
        crate::error::StartStreamTranscriptionError,
    >,
    crate::error::StartStreamTranscriptionError,
> {
    let unmarshaller = crate::event_stream_serde::TranscriptResultStreamUnmarshaller::new();
    let body = std::mem::replace(body, smithy_http::body::SdkBody::taken());
    Ok(smithy_http::event_stream::Receiver::new(unmarshaller, body))
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_filter_method(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::VocabularyFilterMethod>,
    smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amzn-transcribe-vocabulary-filter-method")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_filter_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map
        .get_all("x-amzn-transcribe-vocabulary-filter-name")
        .iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map
        .get_all("x-amzn-transcribe-vocabulary-name")
        .iter();
    smithy_http::header::one_or_none(headers)
}
