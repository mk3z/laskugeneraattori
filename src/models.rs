use crate::schema::{invoice_attachments, invoice_rows, invoices, parties};
use chrono::{DateTime, NaiveDate, Utc};
use garde::Validate;

use serde_derive::{Deserialize, Serialize};

// NOTES:
// This is implemented based on https://github.com/Tietokilta/laskugeneraattori/blob/main/backend/src/procountor.rs#L293
// major changes are justified below:
// - I think PaymentInfo and Party can be joined into one struct/field
//  => due date is moved to the Invoice struct
// - I deem the inclusion of currencies or payment methods unnecessary for now
// - I don't think having a massive enum for product units is necessary, just have it as string :D
// - Is VAT really necessary to account for? I'm leaving it out for now
// - I'm also leaving InvoiceType out, at least for now

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::InvoiceStatus"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    Open,
    Accepted,
    Paid,
}

/// A party of the invoice
#[derive(Identifiable, Queryable, Clone, Debug, Serialize)]
#[diesel(table_name = parties)]
pub struct Party {
    pub id: i32,
    /// The name can be at most 128 characters
    pub name: String,
    /// The street can be at most 128 characters
    pub street: String,
    /// The city can be at most 128 characters
    pub city: String,
    /// The zipcode can be at most 128 characters (:D)
    pub zip: String,
    /// The bank_account can be at most 128 characters
    pub bank_account: String,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize, Validate)]
#[diesel(table_name = parties)]
pub struct NewParty {
    /// The name can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub name: String,
    /// The street can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub street: String,
    /// The city can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub city: String,
    /// The zipcode can be at most 128 characters (:D)
    #[garde(byte_length(max = 128))]
    pub zip: String,
    /// The bank_account can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub bank_account: String,
}

/// The invoice model as stored in the database
#[derive(Identifiable, Queryable, Clone, Debug)]
#[diesel(table_name = invoices)]
pub struct Invoice {
    pub id: i32,
    pub status: InvoiceStatus,
    pub creation_time: DateTime<Utc>,
    pub counter_party_id: i32,
    pub due_date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
    pub status: InvoiceStatus,
    pub creation_time: DateTime<Utc>,
    pub counter_party_id: i32,
    pub due_date: NaiveDate,
}

/// A single row of an invoice
#[derive(Identifiable, Queryable, Clone, Debug, Serialize)]
#[diesel(table_name = invoice_rows)]
pub struct InvoiceRow {
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    pub invoice_id: i32,
    /// The product can be at most 128 characters
    pub product: String,
    pub quantity: i32,
    /// The unit can be at most 128 characters
    pub unit: String,
    /// Unit price is encoded as number of cents to avoid floating-point precision bugs
    pub unit_price: i32,
}

#[derive(Insertable)]
#[diesel(table_name = invoice_rows)]
pub struct NewInvoiceRow {
    pub invoice_id: i32,
    pub product: String,
    pub quantity: i32,
    pub unit: String,
    pub unit_price: i32,
}

/// The metadata for an invoice attachment
/// The file itself can be requested using its hash and filename
/// => /somepath/{hash}/{filename}
#[derive(Identifiable, Queryable, Clone, Debug, Serialize)]
#[diesel(table_name = invoice_attachments)]
pub struct Attachment {
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    pub invoice_id: i32,
    /// The filename can be at most 128 characters
    pub filename: String,
    /// The SHA256 hash of the file contents as a hex string (64 characters)
    pub hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = invoice_attachments)]
pub struct NewAttachment {
    pub invoice_id: i32,
    pub filename: String,
    pub hash: String,
}
