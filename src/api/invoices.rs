use crate::database::DatabaseConnection;
use crate::error::Error;
use axum::{async_trait, body::Bytes, http::StatusCode, Json};
use axum_typed_multipart::{
    FieldData, FieldMetadata, TryFromChunks, TryFromMultipart, TypedMultipart, TypedMultipartError,
};
use axum_valid::Garde;
use chrono::{DateTime, NaiveDate, Utc};
use futures::stream::Stream;
use futures::stream::{self, TryStreamExt};
use garde::Validate;
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

#[async_trait]
impl TryFromChunks for CreateInvoice {
    async fn try_from_chunks(
        chunks: impl Stream<Item = Result<Bytes, TypedMultipartError>> + Send + Sync + Unpin,
        metadata: FieldMetadata,
    ) -> Result<Self, TypedMultipartError> {
        let bytes = Bytes::try_from_chunks(chunks, metadata).await?;

        serde_json::from_slice(&bytes).map_err(|e| TypedMultipartError::Other { source: e.into() })
    }
}

/// Body for the request for creating new invoices
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateInvoice {
    /// The other party of the invoice
    #[garde(dive)]
    pub counter_party: crate::models::NewParty,
    /// The due date of the invoice.
    /// It cannot be in the past
    //TODO: #[garde(time(op = after_now))]
    #[garde(skip)]
    pub due_date: NaiveDate,
    /// The rows of the invoice
    #[garde(length(min = 1), dive)]
    pub rows: Vec<CreateInvoiceRow>,
    // NOTE: We get the attachments from the multipart form
    #[garde(skip)]
    #[serde(skip_deserializing)]
    pub attachments: Vec<CreateInvoiceAttachment>,
}

#[derive(TryFromMultipart, Validate)]
pub struct CreateInvoiceForm {
    #[garde(dive)]
    pub data: CreateInvoice,
    // FIXME: Maybe use NamedTempFile
    #[garde(skip)]
    #[form_data(limit = "unlimited")]
    pub attachments: Vec<FieldData<Bytes>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateInvoiceRow {
    /// The product can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub product: String,
    /// The quantity of the product, must be positive
    #[garde(range(min = 1))]
    pub quantity: i32,
    /// The unit can be at most 128 characters
    #[garde(byte_length(max = 128))]
    pub unit: String,
    /// Unit price is encoded as number of cents to avoid floating-point precision bugs
    /// must be positive
    #[garde(range(min = 1))]
    pub unit_price: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateInvoiceAttachment {
    pub filename: String,
    pub hash: String,
}

/// A populated invoice type that is returned to the user
#[derive(Clone, Debug, Serialize)]
pub struct PopulatedInvoice {
    pub id: i32,
    pub status: crate::models::InvoiceStatus,
    pub creation_time: DateTime<Utc>,
    pub due_date: NaiveDate,
    pub counter_party: crate::models::Party,
    pub rows: Vec<crate::models::InvoiceRow>,
    pub attachments: Vec<crate::models::Attachment>,
}

async fn try_handle_file(field: &FieldData<Bytes>) -> Result<CreateInvoiceAttachment, Error> {
    let filename = field
        .metadata
        .file_name
        .as_ref()
        .ok_or(Error::MissingFilename)?
        .to_string();

    let cont = field.contents.clone();
    // NOTE: Avoid blocking the entire tokio runtime
    let hash = tokio_rayon::spawn_fifo(move || hex::encode(Sha256::digest(&cont))).await;
    let file_path = format!(
        "{}/{hash}",
        std::env::var("ATTACHMENT_PATH").unwrap_or(String::from("."))
    );

    if tokio::fs::File::open(&file_path).await.is_err() {
        let mut file = tokio::fs::File::create(&file_path).await?;

        // FIXME: Properly handle write error
        // as failing here should remove the created file
        file.write_all(&field.contents).await?;
    } else {
        debug!("Skipping duplicate file: {hash}")
    }

    Ok(CreateInvoiceAttachment {
        filename: filename.to_string(),
        hash,
    })
}

pub async fn create(
    mut conn: DatabaseConnection,
    Garde(TypedMultipart(mut multipart)): Garde<TypedMultipart<CreateInvoiceForm>>,
) -> Result<(StatusCode, Json<PopulatedInvoice>), Error> {
    multipart.data.attachments = stream::iter(
        multipart
            .attachments
            .iter()
            .map(try_handle_file)
            .map(Ok)
            // NOTE: This collect might seem harmless but
            // I dare you to try removing it
            .collect::<Vec<_>>(),
    )
    // FIXME: Don't hardcode buffer size
    .try_buffer_unordered(50)
    .try_collect::<Vec<_>>()
    .await?;

    Ok((
        StatusCode::CREATED,
        axum::Json(conn.create_invoice(multipart.data.clone()).await?),
    ))
}

pub async fn list_all(
    mut conn: DatabaseConnection
) -> Result<(StatusCode, Json<Vec<PopulatedInvoice>>), Error> {
    Ok((
        StatusCode::OK,
        axum::Json(conn.list_invoices().await?),
    ))
}
