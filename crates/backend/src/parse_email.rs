use lettre::message::{Mailbox, Message, MessageBuilder, MultiPart, SinglePart};
use lettre::Address;
use mail_parser::{MessageParser, MessagePart, MimeHeaders};
use std::str::FromStr;

pub fn parse_and_build_message(
    message_source: &[u8],
    to_address: &str,
) -> Result<Message, Box<dyn std::error::Error>> {
    let parsed_email = parse_email(message_source)?;
    let to = build_to_address(to_address)?;
    let mut builder = Message::builder();

    builder = set_from_address(builder, &parsed_email)?;
    builder = set_subject(builder, &parsed_email)?;
    builder = builder.to(to);

    let body = build_body(&parsed_email)?;
    let lettre_message = builder.multipart(body)?;

    Ok(lettre_message)
}

fn parse_email(message_source: &[u8]) -> Result<mail_parser::Message, Box<dyn std::error::Error>> {
    MessageParser::default()
        .parse(message_source)
        .ok_or_else(|| "Failed to parse the email".into())
}

fn build_to_address(to_address: &str) -> Result<Mailbox, Box<dyn std::error::Error>> {
    let address = Address::from_str(to_address)?;
    Ok(Mailbox::new(None, address))
}

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

fn build_multipart_body(
    parsed_email: &mail_parser::Message,
) -> Result<MultiPart, Box<dyn std::error::Error>> {
    let mut multipart = MultiPart::alternative().build();

    for part in &parsed_email.parts {
        let content_type = part.content_type().ok_or("Content type not found")?;
        let content = part.contents();

        let format = format!(
            "{}/{}",
            content_type.c_type,
            content_type.c_subtype.as_deref().unwrap_or("")
        );
        let lettre_content_type = lettre::message::header::ContentType::parse(&format)?;

        multipart = multipart.singlepart(
            SinglePart::builder()
                .header(lettre_content_type)
                .body(content.to_vec()),
        );
    }

    Ok(multipart)
}

fn build_singlepart_body(
    parsed_email: &mail_parser::Message,
) -> Result<SinglePart, Box<dyn std::error::Error>> {
    let content_type = parsed_email
        .content_type()
        .ok_or("Content type not found")?;
    let content = parsed_email.body_text(0).ok_or("Body text not found")?;

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

fn build_attachment(attachment: &MessagePart) -> Result<SinglePart, Box<dyn std::error::Error>> {
    let content_type = attachment
        .content_type()
        .ok_or("Attachment content type not found")?;
    let format = format!(
        "{}/{}",
        content_type.c_type,
        content_type.c_subtype.as_deref().unwrap_or("")
    );
    let lettre_content_type = lettre::message::header::ContentType::parse(&format)?;

    let filename = attachment.attachment_name().unwrap_or("attachment");
    let content_disposition = lettre::message::header::ContentDisposition::attachment(filename);

    Ok(SinglePart::builder()
        .header(lettre_content_type)
        .header(content_disposition)
        .body(attachment.contents().to_vec()))
}
