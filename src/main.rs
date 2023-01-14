use crate::api::Api;

mod api;
mod domain;

#[tokio::main]
async fn main() {
    let api = Api::new();
    let user = api.login().await;
    let contracts = api.contracts(user).await;
}
