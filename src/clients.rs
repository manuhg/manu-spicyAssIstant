use std::env;
use spiceai::ClientBuilder;
use openai_api_rs::v1::api::OpenAIClient;

pub fn get_openai_client() -> OpenAIClient {
    OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string())
}

pub async fn get_spiceai_client() -> spiceai::Client {
    let api_key = env::var("SPICE_AI_API_KEY").unwrap().to_string();
    ClientBuilder::new()
        .api_key(api_key.as_str())
        .use_spiceai_cloud()
        .build()
        .await
        .unwrap()
}
