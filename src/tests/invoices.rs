use crate::api::app;
use crate::api::invoices::{CreateInvoice, CreateInvoiceRow, PopulatedInvoice};
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

#[tokio::test]
async fn list_all() {
    let app = app().with_state(crate::database::new().await);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/invoices").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn create_list_all() {
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

    let server = TestServer::new(app).unwrap();
    let body = MultipartForm::new().add_text("data", serde_json::to_string(&body).unwrap());

    let create_response = server.post("/invoices").multipart(body).await;
    assert_eq!(create_response.status_code(), StatusCode::CREATED);

    let list_response = server.get("/invoices").await;
    assert_eq!(list_response.status_code(), StatusCode::OK);

    let result = std::panic::catch_unwind(|| list_response.json::<Vec<PopulatedInvoice>>());
    assert!(result.is_ok_and(|invoices| !invoices.is_empty()));
}
