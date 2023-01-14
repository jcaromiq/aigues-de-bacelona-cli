use crate::domain::{ConsumptionResponse, ContractDetail, ContractResponse, Login, User};
use chrono::{Duration, Local};
use reqwest::header::HeaderValue;
use reqwest::{header, Client};

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Api {
        let mut request_headers = header::HeaderMap::new();
        request_headers.insert(
            "Ocp-Apim-Subscription-Key",
            HeaderValue::from_static("3cca6060fee14bffa3450b19941bd954"),
        );
        Api {
            client: reqwest::ClientBuilder::new()
                .default_headers(request_headers)
                .cookie_store(true)
                .build()
                .expect("Can not initialize HttpClient"),
        }
    }
    pub async fn login(&self) -> Result<User, reqwest::Error> {
        let response: serde_json::Value = self
            .client
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
            //.expect("http errpr")
            .json()
            .await?;
        // .expect("json error");
        Ok(User::new(
            Login::from_env().user_identification,
            response
                .get("access_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            response.get("scope").unwrap().as_str().unwrap().to_string(),
            response.get("expires_in").unwrap().as_i64().unwrap(),
        ))
    }
    pub async fn contracts(&self, user: &User) -> ContractResponse {
        self.client
            .get("https://api.aiguesdebarcelona.cat/ofex-contracts-api/contracts")
            .query(&[
                ("lang", "ca"),
                ("userId", &user.user),
                ("clientId", &user.user),
            ])
            .send()
            .await
            .expect("http errpr")
            .json()
            .await
            .expect("json error")
    }
    pub async fn consumptions(
        &self,
        user: &User,
        contract: &ContractDetail,
    ) -> ConsumptionResponse {
        self.client
            .get("https://api.aiguesdebarcelona.cat/ofex-water-consumptions-api/meter/consumptions")
            .query(&[
                ("consumptionFrequency", "HOURLY"),
                ("contractNumber", &contract.contract_number),
                ("lang", "ca"),
                ("clientId", &user.user),
                ("userId", &user.user),
                ("fromDate", &Local::now().format("%d-%m-%Y").to_string()),
                (
                    "toDate",
                    &(Local::now() + Duration::days(1))
                        .format("%d-%m-%Y")
                        .to_string(),
                ),
                ("showNegativeValues", "false"),
            ])
            .send()
            .await
            .expect("http errpr")
            .json()
            .await
            .expect("json error")
    }
}
