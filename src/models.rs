// Define all the models

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssistantResponse {
    pub message: String,
    pub data: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LLMResponse {
    pub sql_query: String,
    pub query_desc: String,
}
