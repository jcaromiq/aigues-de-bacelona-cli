use crate::api::Api;

mod api;
mod domain;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api = Api::new().await?;
    println!(
        "Today Consumed liters {:?}",
        api.get_today_consumptions().await?
    );

    println!(
        "Yesterday Consumed liters {:?}",
        api.get_yesterday_consumptions().await?
    );
    Ok(())
}
