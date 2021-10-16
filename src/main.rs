use reqwest::Result;
use rust_paystack::transaction;

#[tokio::main]
async fn main() -> Result<(())> {
    let transaction =
        transaction::NewTransaction::new(100, String::from("mayowaaj69@hotmail.co.uk"));

    let response = transaction::initiate_transaction(&transaction).await?;

    println!("{}", response.text().await?);

    Ok(())
}
