extern crate dotenv;

use dotenv::dotenv;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use std::env;

const INITIATE_TRANSACTION_URL: &'static str = "https://api.paystack.co/transaction/initialize";

pub enum TransactionChannel {
    Card,
    Bank,
    Ussd,
    Qr,
    MobileMoney,
    BankTransfer,
}

pub enum Currency {
    Ngn,
    Ghs,
    Zar,
}

pub enum ChargeBearer {
    ACCOUNT,
    SUBACCOUNT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTransaction {
    pub amount: i32, //convert to string
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_charge: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl NewTransaction {
    pub fn new(amount: i32, email: String) -> NewTransaction {
        NewTransaction {
            amount: amount,
            email: email,
            currency: None,
            reference: None,
            callback_url: None,
            channels: None,
            split_code: None,
            subaccount: None,
            plan: None,
            invoice_limit: None,
            transaction_charge: None,
            bearer: None,
            metadata: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

pub async fn initiate_transaction(transaction: &NewTransaction) -> Result<reqwest::Response> {
    dotenv().ok();

    let secret_key = env::var("SECRET_KEY").unwrap();

    let request_json = serde_json::to_string(&transaction).unwrap();

    println!("request = {}", request_json);

    let client = reqwest::Client::new();
    let res = client
        .post(INITIATE_TRANSACTION_URL)
        .bearer_auth(secret_key)
        .body(request_json)
        .send()
        .await?;

    Ok(res)
}
