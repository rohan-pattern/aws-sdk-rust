// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_journey_response_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::JourneyResponse>,
    crate::operation::delete_journey::DeleteJourneyError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_journey_response::de_journey_response_payload(body)
                .map_err(crate::operation::delete_journey::DeleteJourneyError::unhandled)
        })
        .transpose()
}
