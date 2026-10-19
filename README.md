# QRBill generator API  

This is a rewrite of the old API, written in java <https://github.com/clicepfl/qrbill-generator>

## Routes

The API is interacted through its root directory, using POST requests, located at `/qrbill-generator`

Any other method will yield an error

### POST

Posting a request needs the corresponding fields

```json
{
  "account": ""
  "amount": 10,
  "currency": "CHF",
  "message": "Custom message",
  "creditor": {
    "name": "CLIC",
    "street": "Station 14, EPFL",
    "houseNo": "",
    "postalCode": "1015",
    "town": "Lausanne",
    "countryCode": "CH"
  }
}
```

#### Example using cURL

```bash
curl http://localhost:8000/qrbill-generator -d '{"account":"CH2304835177498341002","amount":10,"currency":"CHF", "message": "Souper de Faculté - Lucie Mermod","creditor":{"name":"CLIC","street":"Station 14, EPFL","houseNo":"","postalCode":"1015","town":"Lausanne", "countryCode": "CH"}}'
```

