use lettre::message::{Mailbox, Message, MessageBuilder, MultiPart, SinglePart};
use lettre::Address;
use mail_parser::{MessageParser, MessagePart, MimeHeaders};
use std::str::FromStr;

/// Parse an email from raw bytes and build a Lettre Message
pub fn parse_and_build_message(
    message_source: &[u8],
    to_address: &str,
) -> Result<Message, Box<dyn std::error::Error>> {
    // Parse the raw email
    let parsed_email = parse_email(message_source)?;
    // Build the 'to' address
    let to = build_to_address(to_address)?;
    let mut builder = Message::builder();

    // Set the 'from' address, subject, and 'to' address
    builder = set_from_address(builder, &parsed_email)?;
    builder = set_subject(builder, &parsed_email)?;
    builder = builder.to(to);

    // Build the email body
    let body = build_body(&parsed_email)?;
    let lettre_message = builder.multipart(body)?;

    Ok(lettre_message)
}

/// Parse raw email bytes into a mail_parser::Message
fn parse_email(message_source: &[u8]) -> Result<mail_parser::Message, Box<dyn std::error::Error>> {
    MessageParser::default()
        .parse(message_source)
        .ok_or_else(|| "Failed to parse the email".into())
}

/// Build a Mailbox from a string address
fn build_to_address(to_address: &str) -> Result<Mailbox, Box<dyn std::error::Error>> {
    let address = Address::from_str(to_address)?;
    Ok(Mailbox::new(None, address))
}

/// Set the 'from' address in the MessageBuilder
fn set_from_address(
    builder: MessageBuilder,
    parsed_email: &mail_parser::Message,
) -> Result<MessageBuilder, Box<dyn std::error::Error>> {
    let from = parsed_email
        .from()
        .and_then(|f| f.first())
        .and_then(|f| f.address())
        .ok_or("From address not found")?;
    let from_mailbox = Mailbox::new(None, from.parse()?);
    Ok(builder.from(from_mailbox))
}

/// Set the subject in the MessageBuilder
fn set_subject(
    builder: MessageBuilder,
    parsed_email: &mail_parser::Message,
) -> Result<MessageBuilder, Box<dyn std::error::Error>> {
    if let Some(subject) = parsed_email.subject() {
        Ok(builder.subject(subject))
    } else {
        Ok(builder)
    }
}

/// Build the email body, including attachments
fn build_body(
    parsed_email: &mail_parser::Message,
) -> Result<MultiPart, Box<dyn std::error::Error>> {
    let mut multipart = MultiPart::mixed().build();

    // Add the main body parts
    if parsed_email.parts.len() > 1 {
        multipart = multipart.multipart(build_multipart_body(parsed_email)?);
    } else {
        multipart = multipart.singlepart(build_singlepart_body(parsed_email)?);
    }

    // Add attachments
    for attachment in parsed_email.attachments() {
        multipart = multipart.singlepart(build_attachment(attachment)?);
    }

    Ok(multipart)
}

/// Build a multipart body for emails with multiple parts
fn build_multipart_body(
    parsed_email: &mail_parser::Message,
) -> Result<MultiPart, Box<dyn std::error::Error>> {
    let mut multipart = MultiPart::alternative().build();

    for part in &parsed_email.parts {
        let content_type = part.content_type().ok_or("Content type not found")?;
        let content = part.contents();

        // Create the content type string
        let format = format!(
            "{}/{}",
            content_type.c_type,
            content_type.c_subtype.as_deref().unwrap_or("")
        );
        let lettre_content_type = lettre::message::header::ContentType::parse(&format)?;

        // Add each part to the multipart body
        multipart = multipart.singlepart(
            SinglePart::builder()
                .header(lettre_content_type)
                .body(content.to_vec()),
        );
    }

    Ok(multipart)
}

/// Build a single part body for emails with only one part
fn build_singlepart_body(
    parsed_email: &mail_parser::Message,
) -> Result<SinglePart, Box<dyn std::error::Error>> {
    let content_type = parsed_email
        .content_type()
        .ok_or("Content type not found")?;
    let content = parsed_email.body_text(0).ok_or("Body text not found")?;

    // Create the content type string
    let format = format!(
        "{}/{}",
        content_type.c_type,
        content_type.c_subtype.as_deref().unwrap_or("")
    );
    let lettre_content_type = lettre::message::header::ContentType::parse(&format)?;

    Ok(SinglePart::builder()
        .header(lettre_content_type)
        .body(content.as_bytes().to_vec()))
}

/// Build an attachment part
fn build_attachment(attachment: &MessagePart) -> Result<SinglePart, Box<dyn std::error::Error>> {
    let content_type = attachment
        .content_type()
        .ok_or("Attachment content type not found")?;

    // Create the content type string
    let format = format!(
        "{}/{}",
        content_type.c_type,
        content_type.c_subtype.as_deref().unwrap_or("")
    );
    let lettre_content_type = lettre::message::header::ContentType::parse(&format)?;

    // Set the filename and content disposition
    let filename = attachment.attachment_name().unwrap_or("attachment");
    let content_disposition = lettre::message::header::ContentDisposition::attachment(filename);

    Ok(SinglePart::builder()
        .header(lettre_content_type)
        .header(content_disposition)
        .body(attachment.contents().to_vec()))
}
