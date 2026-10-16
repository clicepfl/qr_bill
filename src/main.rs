use isocountry::CountryCode;
use qrbill::{
    Address, Currency, Iban, Language, QRBill, QRBillOptions, Reference, StructuredAddress,
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, routes};

use chrono::{DateTime, Utc};

use serde::Deserialize;

#[derive(Deserialize)]
struct AddressRequest {
    name: String,
    street: String,
    #[serde(rename = "houseNo")]
    house_no: String,
    #[serde(rename = "postalCode")]
    postal_code: String,
    town: String,
    #[serde(rename = "countryCode")]
    country_code: CountryCode,
}

#[derive(Deserialize)]
enum CurrencyRequest {
    CHF,
    EUR,
}

#[derive(Deserialize)]
struct ReferenceRequest {}

#[derive(Deserialize)]
struct LanguageRequest {}

#[derive(Deserialize)]
struct QrbillRequest {
    account: String,
    amount: Option<f64>,
    currency: CurrencyRequest,
    #[serde(rename = "message")]
    extra_infos: Option<String>,
    creditor: AddressRequest,
    //due_date: Option<DateTime<Utc>>,
    //debtor: Option<AddressRequest>,
    //reference: ReferenceRequest,
    //alternative_processes: Vec<String>,
    //language: LanguageRequest,
    //top_line: bool,
    //payment_line: bool,
}

#[post("/", data = "<data>")]
fn post_qr(data: Json<QrbillRequest>) -> (Status, String) {
    let iban = match (&data).account.parse::<Iban>() {
        Ok(iban) => iban,
        Err(_) => return (Status::BadRequest, String::from("Could not parse iban")),
    };

    let opts = QRBillOptions {
        account: iban,
        creditor: Address::Structured(StructuredAddress {
            name: data.creditor.name.clone(),
            street: data.creditor.street.clone(),
            house_number: data.creditor.house_no.clone(),
            postal_code: data.creditor.postal_code.clone(),
            city: data.creditor.town.clone(),
            country: data.creditor.country_code.clone(),
        }),
        amount: data.amount,
        currency: match data.currency {
            CurrencyRequest::CHF => Currency::SwissFranc,
            CurrencyRequest::EUR => Currency::Euro,
        },
        due_date: None,
        debtor: None,
        reference: Reference::None,
        extra_infos: data.extra_infos.clone(),
        alternative_processes: vec![],
        language: Language::English,
        top_line: false,
        payment_line: false,
    };

    match QRBill::new(opts) {
        Ok(qr) => match qr.create_svg(false) {
            Ok(svg) => (Status::Ok, svg),
            Err(_) => (
                Status::InternalServerError,
                String::from("An error ocurred while rendering the QRBill SVG"),
            ),
        },
        Err(e) => match e {
            qrbill::Error::InvalidIban => (Status::BadRequest, String::from("Invalid iban")),
            qrbill::Error::ExtraInfos => (
                Status::BadRequest,
                String::from("Message field content is too long"),
            ),
            _ => (
                Status::InternalServerError,
                String::from("A server error ocurred"),
            ),
        },
    }
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/qrbill-generator", routes![post_qr])
        .launch()
        .await
        .unwrap();
}
