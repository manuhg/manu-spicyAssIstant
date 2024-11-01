use std::env;
use spiceai::ClientBuilder;
use openai_api_rs::v1::api::OpenAIClient;

pub struct NecessaryClients {
    pub open_ai: OpenAIClient,
    pub spice_ai: spiceai::Client,
}
// impl NecessaryClients {
//     pub fn get_openai_client(&self)->&OpenAIClient{
//         &self.open_ai
//     }
//     pub fn get_spiceai_client(&mut self)->&mut spiceai::Client{
//         &mut self.spice_ai
//     }
// }

pub fn get_openai_client() -> OpenAIClient {
    OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string())
}

async fn get_spiceai_client() -> spiceai::Client {
    let api_key = env::var("SPICE_AI_API_KEY").unwrap().to_string();
    ClientBuilder::new()
        .api_key(api_key.as_str())
        // .use_spiceai_cloud()
        .build()
        .await
        .unwrap()
}

pub async fn get_necessary_clients() -> NecessaryClients {

    NecessaryClients {
        open_ai: get_openai_client(),
        spice_ai: get_spiceai_client().await
    }
}
