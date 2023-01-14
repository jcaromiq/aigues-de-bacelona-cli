use reqwest::header::HeaderValue;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;


#[tokio::main]
async fn main() {
    let mut request_headers = header::HeaderMap::new();
    request_headers.insert(
        "Ocp-Apim-Subscription-Key",
        HeaderValue::from_static("3cca6060fee14bffa3450b19941bd954"),
    );
    let client: Client = reqwest::ClientBuilder::new()
        .default_headers(request_headers)
        .cookie_store(true)
        .build()
        .unwrap();
    let token = login(&client).await;
    let contracts = contracts(&client, token.user).await;
}

async fn contracts(client: &Client, user:String) -> ContractResponse {
    client
        .get("https://api.aiguesdebarcelona.cat/ofex-contracts-api/contracts")
        .query(&[("lang", "ca"), ("userId", &user), ("clientId", &user)])
        .send()
        .await
        .expect("http errpr")
        .json()
        .await
        .expect("json error")
}

async fn login(client: &Client) -> LoginResponse {
    let response : serde_json::Value = client
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
    LoginResponse::new(Login::from_env().user_identification,
    response.get("access_token").unwrap().as_str().unwrap().to_string(),
    response.get("scope").unwrap().as_str().unwrap().to_string(),
    response.get("expires_in").unwrap().as_i64().unwrap(),
    )
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
struct LoginResponse {
    user: String,
    access_token: String,
    scope: String,
    expires_in: i64,
}

impl LoginResponse {
    fn new(user:String, access_token:String, scope:String, expires_in:i64) -> Self {
        LoginResponse{
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
