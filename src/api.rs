use crate::domain::{ConsumptionResponse, ContractDetail, ContractResponse, Login, User};
use chrono::{DateTime, Duration, Local};
use reqwest::header::HeaderValue;
use reqwest::{header, Client};

pub struct Api {
    client: Client,
    user: User,
}

impl Api {
    pub async fn new() -> Result<Api, reqwest::Error> {
        let mut request_headers = header::HeaderMap::new();
        request_headers.insert(
            "Ocp-Apim-Subscription-Key",
            HeaderValue::from_static("3cca6060fee14bffa3450b19941bd954"),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(request_headers)
            .cookie_store(true)
            .build()?;

        let response: serde_json::Value = client
            .post("https://api.aiguesdebarcelona.cat/ofex-login-api/auth/getToken")
            .query(&[("lang", "ca"), ("recaptchaClientResponse", "")])
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                "Ocp-Apim-Subscription-Key",
                "6a98b8b8c7b243cda682a43f09e6588b;product=portlet-login-ofex",
            )
            .json(&Login::from_env())
            .send()
            .await?
            .json()
            .await?;
        let user = User::new(
            Login::from_env().user_identification,
            response
                .get("access_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            response.get("scope").unwrap().as_str().unwrap().to_string(),
            response.get("expires_in").unwrap().as_i64().unwrap(),
        );
        Ok(Api { client, user })
    }

    async fn contracts(&self, user: &User) -> Result<ContractResponse, reqwest::Error> {
        self.client
            .get("https://api.aiguesdebarcelona.cat/ofex-contracts-api/contracts")
            .query(&[
                ("lang", "ca"),
                ("userId", &user.user),
                ("clientId", &user.user),
            ])
            .send()
            .await?
            .json()
            .await
    }
    async fn consumptions(
        &self,
        contract: &ContractDetail,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Result<ConsumptionResponse, reqwest::Error> {
        self.client
            .get("https://api.aiguesdebarcelona.cat/ofex-water-consumptions-api/meter/consumptions")
            .query(&[
                ("consumptionFrequency", "HOURLY"),
                ("contractNumber", &contract.contract_number),
                ("lang", "ca"),
                ("clientId", &self.user.user),
                ("userId", &self.user.user),
                ("fromDate", &from.format("%d-%m-%Y").to_string()),
                ("toDate", &to.format("%d-%m-%Y").to_string()),
                ("showNegativeValues", "false"),
            ])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_today_consumptions(&self) -> Result<f32, reqwest::Error> {
        let contracts = self.contracts(&self.user).await?;
        let today_consumptions = self
            .consumptions(
                contracts.first_contract_number(),
                Local::now(),
                Local::now() + Duration::days(1),
            )
            .await?
            .get_total_liters();
        Ok(today_consumptions)
    }

    pub async fn get_yesterday_consumptions(&self) -> Result<f32, reqwest::Error> {
        let contracts = self.contracts(&self.user).await?;
        let today_consumptions = self
            .consumptions(
                contracts.first_contract_number(),
                Local::now() - Duration::days(1),
                Local::now(),
            )
            .await?
            .get_total_liters();
        Ok(today_consumptions)
    }
}
