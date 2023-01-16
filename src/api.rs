use crate::domain::{ConsumptionResponse, ContractResponse, Login, User};
use chrono::{DateTime, Duration, Local};
use reqwest::header::HeaderValue;
use reqwest::{header, Client};

pub struct Api {
    http_client: Client,
    user: User,
}

impl Api {
    pub async fn new() -> Result<Api, reqwest::Error> {
        let credentials = Login::from_env();

        let http_client = Self::init_client()?;

        let access_token = Self::get_token(&http_client, &credentials).await;

        let contract_number =
            Self::get_contract_number(&credentials.user_identification, &http_client).await;

        let user = User::new(
            credentials.user_identification,
            access_token?,
            contract_number?,
        );
        Ok(Api { http_client, user })
    }

    async fn get_contract_number(user: &String, http_client: &Client) -> Result<String, reqwest::Error> {
        let contract_number: ContractResponse = http_client
            .get("https://api.aiguesdebarcelona.cat/ofex-contracts-api/contracts")
            .query(&[("lang", "ca"), ("userId", user), ("clientId", user)])
            .send()
            .await?
            .json()
            .await?;
        Ok(contract_number.first_contract_number())
    }

    async fn get_token(http_client: &Client, credentials: &Login) -> Result<String, reqwest::Error> {
        let response: serde_json::Value = http_client
            .post("https://api.aiguesdebarcelona.cat/ofex-login-api/auth/getToken")
            .query(&[("lang", "ca"), ("recaptchaClientResponse", "")])
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                "Ocp-Apim-Subscription-Key",
                "6a98b8b8c7b243cda682a43f09e6588b;product=portlet-login-ofex",
            )
            .json(&credentials)
            .send()
            .await?
            .json()
            .await?;
        Ok(response
            .get("access_token")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
    }

    fn init_client() -> Result<Client, reqwest::Error> {
        let mut request_headers = header::HeaderMap::new();
        request_headers.insert(
            "Ocp-Apim-Subscription-Key",
            HeaderValue::from_static("3cca6060fee14bffa3450b19941bd954"),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(request_headers)
            .cookie_store(true)
            .build()?;
        Ok(client)
    }

    async fn consumptions(
        &self,
        contract: String,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Result<ConsumptionResponse, reqwest::Error> {
        self.http_client
            .get("https://api.aiguesdebarcelona.cat/ofex-water-consumptions-api/meter/consumptions")
            .query(&[
                ("consumptionFrequency", "HOURLY"),
                ("contractNumber", &contract),
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
        let today_consumptions = self
            .consumptions(
                String::from(&self.user.contract_number),
                Local::now(),
                Local::now() + Duration::days(1),
            )
            .await?
            .get_total_liters();
        Ok(today_consumptions)
    }

    pub async fn get_yesterday_consumptions(&self) -> Result<f32, reqwest::Error> {
        let today_consumptions = self
            .consumptions(
                String::from(&self.user.contract_number),
                Local::now() - Duration::days(1),
                Local::now(),
            )
            .await?
            .get_total_liters();
        Ok(today_consumptions)
    }
}
