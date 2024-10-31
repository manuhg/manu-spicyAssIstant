use std::env;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;

use crate::{constants};
use crate::models::LLMResponse;

fn get_openai_client() -> OpenAIClient {
    dotenv::dotenv().ok().expect("Failed to load .env file");
    OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string())
}
fn parse_json(json_text: &str) -> Option<LLMResponse> {
    println!("JSON data: {:?}", json_text);
    let start = json_text.find('{')?;
    let end = json_text.rfind('}')?;
    let json_text = json_text[start..=end].to_string();
    serde_json::from_str::<Option<LLMResponse>>(json_text.as_str()).expect("Invalid JSON: ")
    // Convert Result to Option, returning None if there's an error
}
pub async fn talk_to_llm(user_input: String) -> Result<Option<LLMResponse>, Box<dyn std::error::Error>> {
    let client = get_openai_client();
    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(constants::LLM_PRE_PROMPT.to_string())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }, chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(user_input)),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    let result = client.chat_completion(req).await?;
    let json_data = match result.choices[0].message.content.clone() {
        Some(content) => content,
        None => return Err("Cannot find valid JSON from LLM response".into()),
    };
    match parse_json(&json_data) {
        Some(parsed) => Ok(Some(parsed)),
        None => Err("Failed to parse JSON from LLM response".into()),
    }
}
