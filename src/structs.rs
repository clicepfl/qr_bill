use isocountry::CountryCode;
use qrbill::{Address, Currency, Iban, Language, QRBillOptions, Reference, StructuredAddress};
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
pub struct QRBillRequest {
    pub account: String,
    pub amount: Option<f64>,
    pub currency: CurrencyRequest,
    #[serde(rename = "message")]
    pub extra_infos: Option<String>,
    pub creditor: AddressRequest,
}

impl TryInto<QRBillOptions> for QRBillRequest {
    type Error = ();

    fn try_into(self) -> Result<QRBillOptions, Self::Error> {
        let iban = match (&self).account.parse::<Iban>() {
            Ok(iban) => iban,
            Err(_) => return Err(()),
        };

        Ok(QRBillOptions {
            account: iban,
            creditor: Address::Structured(StructuredAddress {
                name: self.creditor.name.clone(),
                street: self.creditor.street.clone(),
                house_number: self.creditor.house_no.clone(),
                postal_code: self.creditor.postal_code.clone(),
                city: self.creditor.town.clone(),
                country: self.creditor.country_code.clone(),
            }),
            amount: self.amount,
            currency: match self.currency {
                CurrencyRequest::CHF => Currency::SwissFranc,
                CurrencyRequest::EUR => Currency::Euro,
            },
            due_date: None,
            debtor: None,
            reference: Reference::None,
            extra_infos: self.extra_infos.clone(),
            alternative_processes: vec![],
            language: Language::English,
            top_line: false,
            payment_line: false,
        })
    }
}
