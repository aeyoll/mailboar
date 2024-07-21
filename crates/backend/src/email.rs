use crate::repository::{Message, MessagePart};
use chrono::Utc;
use mailparse::{parse_mail, DispositionType, MailHeaderMap, MailParseError, ParsedMail};

/// Parse a raw email into a `Message` struct
///
/// Example:
/// ```rust
/// use mailboar_backend::email::parse_message;
/// let message = parse_message(
///     &Some("sender@example.com".to_string()),
///     &["recipient@example.com".to_string()],
///     b"Subject: This is a test email\nContent-Type: text/plain\n\nHello world!\n"
/// );
/// ```
pub fn parse_message(
    sender: &Option<String>,
    recipients: &[String],
    raw_email: &[u8],
) -> Result<Message, MailParseError> {
    let parsed_email = parse_mail(raw_email)?;

    let sender = sender.clone();
    let headers = parsed_email.get_headers();

    let subject = headers.get_first_value("Subject");

    let typ = parsed_email.ctype.mimetype.clone();

    let mut parsed_parts = vec![];
    flatten_parts_into(&mut parsed_parts, &parsed_email);

    let mut parts = vec![];
    let mut part_id = 1;
    for parsed_part in parsed_parts {
        let body = parsed_part.get_body_raw().unwrap().clone();
        parts.push(MessagePart {
            cid: format!("{}.mail", part_id),
            typ: parsed_part.ctype.mimetype.clone(),
            filename: parsed_part
                .ctype
                .params
                .get("name")
                .cloned()
                .unwrap_or_else(|| format!("part{}", part_id)),
            size: body.len(),
            charset: parsed_part.ctype.charset.clone(),
            body,
            is_attachment: parsed_part.get_content_disposition().disposition
                == DispositionType::Attachment,
        });
        part_id += 1;
    }

    Ok(Message {
        id: None,
        size: raw_email.len(),
        charset: parsed_email.ctype.charset.clone(),
        subject,
        sender,
        recipients: recipients.to_owned(),
        created_at: Utc::now(),
        typ,
        parts,
        source: raw_email.to_vec(),
    })
}

fn flatten_parts_into<'e>(vec: &mut Vec<&'e ParsedMail<'e>>, parsed_email: &'e ParsedMail<'e>) {
    for part in &parsed_email.subparts {
        vec.push(part);
        flatten_parts_into(vec, part);
    }
}

#[cfg(test)]
mod tests {
    use crate::email::parse_message;

    #[tokio::test]
    async fn test_parse_message() {
        let sender = Some("sender@example.com".to_string());
        let recipients = vec!["recipient@example.com".to_string()];
        let raw_email = concat!(
            "Subject: This is a test email\n",
            "Content-Type: multipart/ alternative; boundary=foobar\n",
            "Date: Sun, 02 Oct 2016 07:06:22 -0700 (PDT)\n",
            "\n",
            "--foobar\n",
            "Content-Type: text/plain; charset=utf-8\n",
            "Content-Transfer-Encoding: quoted-printable\n",
            "\n",
            "Hello world!\r\n"
        )
        .as_bytes();

        let message = parse_message(&sender, &recipients, raw_email);
        assert!(message.is_ok());
        let message = message.unwrap();
        assert_eq!(message.subject, Some("This is a test email".to_string()));
        assert_eq!(message.sender, sender);
        assert_eq!(message.recipients, recipients);
        assert_eq!(message.parts.len(), 1);
        assert_eq!(message.parts[0].typ, "text/plain");
        assert_eq!(message.parts[0].body, b"Hello world!\r\n".to_vec());
    }
}
