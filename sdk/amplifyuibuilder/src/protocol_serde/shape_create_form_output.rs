// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_entity_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::Form>,
    crate::operation::create_form::CreateFormError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_form::de_form_payload(body)
                .map_err(crate::operation::create_form::CreateFormError::unhandled)
        })
        .transpose()
}
