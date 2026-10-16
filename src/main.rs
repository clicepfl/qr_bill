use qrbill::{
    Address, Currency, Iban, Language, QRBill, QRBillOptions, Reference, StructuredAddress,
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes};

mod structs;
use structs::{CurrencyRequest, QrbillRequest};

#[get("/")]
fn get() -> (Status, String) {
    (
        Status::MethodNotAllowed,
        String::from("Please use a POST request"),
    )
}

#[catch(422)]
fn catch_malformed() -> (Status, String) {
    (
        Status::BadRequest,
        String::from("Could not parse form data"),
    )
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
                Status::PayloadTooLarge,
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
        .mount("/qrbill-generator", routes![post_qr, get])
        .register("/qrbill-generator", catchers![catch_malformed])
        .launch()
        .await
        .unwrap();
}
