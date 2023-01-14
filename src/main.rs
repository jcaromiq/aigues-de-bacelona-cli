use reqwest::header::HeaderValue;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;

struct Api {
    client: Client,
}

impl Api {
    fn new() -> Api {
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
                .unwrap(),
        }
    }
    async fn login(&self) -> User {
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
            .await
            .expect("http errpr")
            .json()
            .await
            .expect("json error");
        User::new(
            Login::from_env().user_identification,
            response
                .get("access_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            response.get("scope").unwrap().as_str().unwrap().to_string(),
            response.get("expires_in").unwrap().as_i64().unwrap(),
        )
    }
    async fn contracts(&self, user: User) -> ContractResponse {
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
}

#[tokio::main]
async fn main() {
    let api = Api::new();
    let user = api.login().await;
    let contracts = api.contracts(user).await;
}

#[derive(Debug, Serialize, Deserialize)]
struct ContractResponse {
    #[serde(rename = "data")]
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    #[serde(rename = "contractDetail")]
    contract_detail: ContractDetail,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContractDetail {
    #[serde(rename = "contractId")]
    contract_id: String,
    #[serde(rename = "contractNumber")]
    contract_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    user: String,
    access_token: String,
    scope: String,
    expires_in: i64,
}

impl User {
    fn new(user: String, access_token: String, scope: String, expires_in: i64) -> Self {
        User {
            user,
            access_token,
            scope,
            expires_in,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Login {
    #[serde(rename = "userIdentification")]
    user_identification: String,
    password: String,
    scope: String,
    #[serde(rename = "companyIdentification")]
    company_identification: String,
}

impl Login {
    fn with_credentials(user: String, password: String) -> Self {
        Login {
            user_identification: user,
            password,
            scope: "ofex".to_string(),
            company_identification: "".to_string(),
        }
    }
    fn from_env() -> Self {
        Self::with_credentials(env::var("USER").unwrap(), env::var("PASSWORD").unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    name: String,
}
