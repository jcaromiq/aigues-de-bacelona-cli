use crate::api::Api;

mod api;
mod domain;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api = Api::new()?;
    let user = api.login().await?;
    let contracts = api.contracts(&user).await?;
    let consumptions = api
        .consumptions(&user, contracts.first_contract_number())
        .await?;
    println!(
        "Today consumed liters {:?}",
        consumptions.get_total_liters()
    );
    Ok(())
}
