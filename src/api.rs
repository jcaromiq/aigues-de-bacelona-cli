use crate::domain::{ContractResponse, Login, User};
use reqwest::header::HeaderValue;
use reqwest::{header, Client};

pub struct Api {
    client: Client,
}

impl Api {
    pub(crate) fn new() -> Api {
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
    pub(crate) async fn login(&self) -> User {
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
    pub(crate) async fn contracts(&self, user: User) -> ContractResponse {
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
