use super::DatabaseConnection;
use crate::api::invoices::{CreateInvoice, PopulatedInvoice};
use crate::error::Error;
use crate::models::*;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl DatabaseConnection {
    pub async fn create_party(&mut self, party: &NewParty) -> Result<Party, Error> {
        use crate::schema::parties::dsl::*;

        diesel::insert_into(parties)
            .values(party)
            .on_conflict(diesel::upsert::on_constraint("no_duplicates"))
            .do_nothing()
            .execute(&mut self.0)
            .await?;

        // NOTE: Diesel is dumb so we have to requery for the data
        // because on_conflict() doesn't support returning()
        Ok(parties
            .filter(
                name.eq(&party.name)
                    .and(street.eq(&party.street))
                    .and(city.eq(&party.city))
                    .and(zip.eq(&party.zip))
                    .and(bank_account.eq(&party.bank_account)),
            )
            .first::<Party>(&mut self.0)
            .await?)
    }

    pub async fn create_invoice(
        &mut self,
        invoice: CreateInvoice,
    ) -> Result<PopulatedInvoice, Error> {
        let party = self.create_party(&invoice.counter_party).await?;

        let inv = NewInvoice {
            status: InvoiceStatus::Open,
            counter_party_id: party.id,
            creation_time: chrono::Utc::now(),
            due_date: invoice.due_date,
        };

        let created = {
            use crate::schema::invoices::dsl::*;
            diesel::insert_into(invoices)
                .values(&inv)
                .returning(invoices::all_columns())
                .get_result::<Invoice>(&mut self.0)
                .await?
        };

        let rows = {
            use crate::schema::invoice_rows::dsl::*;
            diesel::insert_into(invoice_rows)
                .values(
                    &invoice
                        .rows
                        .into_iter()
                        .map(|r| NewInvoiceRow {
                            invoice_id: created.id,
                            product: r.product,
                            quantity: r.quantity,
                            unit: r.unit,
                            unit_price: r.unit_price,
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(invoice_rows::all_columns())
                .get_results::<InvoiceRow>(&mut self.0)
                .await?
        };

        let attachments = {
            use crate::schema::invoice_attachments::dsl::*;
            diesel::insert_into(invoice_attachments)
                .values(
                    &invoice
                        .attachments
                        .into_iter()
                        .map(|a| NewAttachment {
                            invoice_id: created.id,
                            hash: a.hash,
                            filename: a.filename,
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(invoice_attachments::all_columns())
                .get_results::<Attachment>(&mut self.0)
                .await?
        };

        Ok(PopulatedInvoice {
            id: created.id,
            status: created.status,
            creation_time: created.creation_time,
            counter_party: party,
            rows,
            due_date: created.due_date,
            attachments,
        })
    }
}
