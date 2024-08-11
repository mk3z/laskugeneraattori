use super::MailgunClient;
use crate::api::invoices::PopulatedInvoice;
use crate::error::Error;

impl MailgunClient {
    #[allow(dead_code)]
    pub async fn send_mail(self, invoice: &PopulatedInvoice) -> Result<(), Error> {
        let invoice_recipient = format!("{} <{}>", invoice.recipient_name, invoice.recipient_email);
        let form = reqwest::multipart::Form::new()
            .text("from", self.from)
            .text("to", self.default_to)
            .text("cc", invoice_recipient)
            .text("subject", format!("Uusi lasku #{}", invoice.id))
            .text("html", format!("Uusi lasku #{}", invoice.id));

        let form = invoice
            .attachments
            .iter()
            .try_fold(form, |form, attachment| {
                let path = std::env::var("ATTACHMENT_PATH").unwrap_or(String::from("."));
                let path = std::path::Path::new(&path).join(&attachment.hash);
                let bytes = std::fs::read(path)?;
                Ok::<reqwest::multipart::Form, Error>(form.part(
                    "attachment",
                    reqwest::multipart::Part::bytes(bytes).file_name(attachment.filename.clone()),
                ))
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
