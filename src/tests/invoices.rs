use crate::api::app;
use crate::api::invoices::{CreateInvoice, CreateInvoiceRow};
use crate::models::NewParty;

use axum::http::StatusCode;
use axum_test::multipart::MultipartForm;
use axum_test::TestServer;

#[tokio::test]
async fn create() {
    let app = app().with_state(crate::database::new().await);

    let body = CreateInvoice {
        counter_party: NewParty {
            name: String::from("Velkoja"),
            street: String::from("Otakaari"),
            city: String::from("Espoo"),
            zip: String::from("02jotain"),
            bank_account: String::from("ei ole"),
        },
        due_date: chrono::Local::now().date_naive(),
        rows: vec![
            CreateInvoiceRow {
                product: String::from("pleikkari"),
                quantity: 69,
                unit: String::from("kpl"),
                unit_price: 4200,
            },
            CreateInvoiceRow {
                product: String::from("xbox"),
                quantity: 1,
                unit: String::from("kpl"),
                unit_price: 4200,
            },
            CreateInvoiceRow {
                product: String::from("nintendo wii"),
                quantity: 2,
                unit: String::from("kpl"),
                unit_price: 4200,
            },
        ],
        attachments: vec![],
    };

    let body = MultipartForm::new().add_text("data", serde_json::to_string(&body).unwrap());
    let server = TestServer::new(app).unwrap();

    let response = server.post("/invoices").multipart(body).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
}
