// Define all the models

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LLMResponse {
    pub sql_query: String,
    pub query_desc: String,
}
