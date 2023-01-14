use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractResponse {
    #[serde(rename = "data")]
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "contractDetail")]
    contract_detail: ContractDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractDetail {
    #[serde(rename = "contractId")]
    contract_id: String,
    #[serde(rename = "contractNumber")]
    contract_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) user: String,
    access_token: String,
    scope: String,
    expires_in: i64,
}

impl User {
    pub(crate) fn new(user: String, access_token: String, scope: String, expires_in: i64) -> Self {
        User {
            user,
            access_token,
            scope,
            expires_in,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    #[serde(rename = "userIdentification")]
    pub(crate) user_identification: String,
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
    pub fn from_env() -> Self {
        Self::with_credentials(env::var("USER").unwrap(), env::var("PASSWORD").unwrap())
    }
}
