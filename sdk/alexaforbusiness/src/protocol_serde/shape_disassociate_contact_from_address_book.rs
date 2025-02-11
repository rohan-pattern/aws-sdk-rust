// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_contact_from_address_book_input(
    input: &crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_disassociate_contact_from_address_book_input::ser_disassociate_contact_from_address_book_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_contact_from_address_book_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookOutput, crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_contact_from_address_book_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookOutput, crate::operation::disassociate_contact_from_address_book::DisassociateContactFromAddressBookError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disassociate_contact_from_address_book::builders::DisassociateContactFromAddressBookOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
