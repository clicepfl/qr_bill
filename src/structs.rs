use isocountry::CountryCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddressRequest {
    pub name: String,
    pub street: String,
    #[serde(rename = "houseNo")]
    pub house_no: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub town: String,
    #[serde(rename = "countryCode")]
    pub country_code: CountryCode,
}

#[derive(Deserialize)]
pub enum CurrencyRequest {
    CHF,
    EUR,
}

#[derive(Deserialize)]
pub struct QrbillRequest {
    pub account: String,
    pub amount: Option<f64>,
    pub currency: CurrencyRequest,
    #[serde(rename = "message")]
    pub extra_infos: Option<String>,
    pub creditor: AddressRequest,
}
