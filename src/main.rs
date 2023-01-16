use crate::api::Api;

mod api;
mod domain;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api = Api::new().await?;
    let today = api.get_today_consumptions().await?;
    let yesterday = api.get_yesterday_consumptions().await?;

    println!(
        "Consumed liters\nYesterday\t\t\t: {}\nToday's provisional\t: {}",
        yesterday, today
    );
    Ok(())
}
