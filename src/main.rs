use isocountry::CountryCode;
use qrbill::{
    Address, Currency, Iban, Language, QRBill, QRBillOptions, Reference, StructuredAddress,
};
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
fn post_qr(data: Json<QrbillRequest>) -> String {
    let opts = QRBillOptions {
        account: (&data).account.parse::<Iban>().unwrap(),
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

    let qr = QRBill::new(opts);
    qr.unwrap().create_svg(false).unwrap()
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/qrbill-generator", routes![post_qr])
        .launch()
        .await
        .unwrap();
}
