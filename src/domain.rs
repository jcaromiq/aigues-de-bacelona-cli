use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionResponse {
    #[serde(rename = "data")]
    pub consumptions: Vec<Consumption>,
}

impl ConsumptionResponse {
    pub fn get_total_liters(&self) -> f32 {
        self.consumptions
            .iter()
            .map(|x| x.to_litters())
            .reduce(|acc, e| acc + e)
            .unwrap_or(0.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(rename = "deltaConsumption")]
    pub delta_consumption: f32,
    #[serde(rename = "datetime")]
    pub date_time: String,
}

impl Consumption {
    pub fn to_litters(&self) -> f32 {
        self.delta_consumption * 1000.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractResponse {
    #[serde(rename = "data")]
    pub contracts: Vec<Contract>,
}

impl ContractResponse {
    pub fn first_contract_number(&self) -> &ContractDetail {
        &self.contracts.first().expect("pa").detail
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "contractDetail")]
    pub detail: ContractDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractDetail {
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "contractNumber")]
    pub contract_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user: String,
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
