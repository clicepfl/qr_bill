use qrbill::{QRBill, QRBillOptions};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes};

mod structs;
use structs::QRBillRequest;

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
fn post_qr(data: Json<QRBillRequest>) -> (Status, String) {
    let opts: Result<QRBillOptions, ()> = data.into_inner().try_into();

    let opts = match opts {
        Ok(opts) => opts,
        Err(_) => return (Status::BadRequest, String::from("Could not parse IBAN")),
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
        .mount("/", routes![post_qr, get])
        .register("/", catchers![catch_malformed])
        .launch()
        .await
        .unwrap();
}
