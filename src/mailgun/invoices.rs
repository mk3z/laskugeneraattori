use super::MailgunClient;
use crate::api::invoices::Invoice;
use crate::error::Error;
use crate::merge::merge_pdf;
use typst::model::Document;

impl MailgunClient {
    pub async fn send_mail(self, invoice: Invoice) -> Result<(), Error> {
        let document: Document = invoice.to_owned().try_into()?;
        let pdf = typst_pdf::pdf(&document, typst::foundations::Smart::Auto, None);

        let mut pdfs = vec![pdf];
        pdfs.extend_from_slice(
            invoice
                .attachments
                .clone()
                .into_iter()
                .map(|a| a.bytes)
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let pdf = merge_pdf(pdfs)?;

        let invoice_recipient = format!("{} <{}>", invoice.recipient_name, invoice.recipient_email);
        let form = reqwest::multipart::Form::new()
            .text("from", self.from)
            .text("to", self.default_to)
            .text("cc", invoice_recipient)
            .text(
                "subject",
                format!("Uusi lasku, lähettäjä {}", invoice.recipient_name),
            )
            .text(
                "html",
                format!("Uusi lasku, lähettäjä {}", invoice.recipient_name),
            )
            .part(
                "attachment",
                reqwest::multipart::Part::bytes(pdf).file_name("invoice.pdf"),
            );

        let form = invoice
            .attachments
            .into_iter()
            .try_fold(form, |form, attachment| {
                Ok::<reqwest::multipart::Form, Error>(
                    form.part(
                        "attachment",
                        reqwest::multipart::Part::bytes(attachment.bytes)
                            .file_name(attachment.filename.clone()),
                    ),
                )
            })?;

        let response = self
            .client
            .post(self.url)
            .basic_auth(self.api_user, Some(self.api_key))
            .multipart(form)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ReqwestError(e)),
        }
    }
}
