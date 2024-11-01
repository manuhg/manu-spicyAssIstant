use manu_spicy_assistant;
use manu_spicy_assistant::clients::get_openai_client;
use manu_spicy_assistant::llm_connect::talk_to_llm;

#[tokio::test]
async fn test_llm_response() {
    let user_input = "Hi, can you please tell me how many BMW evs are present in washington state in 2024?";
    let response = talk_to_llm(user_input.to_string(),  get_openai_client()).await.unwrap();

    // Assert that the response is not None
    assert!(response.is_some(), "Response should not be None!");

    // Unwrap the response for further checks
    let response = response.unwrap();

    // Assert that the `message` field is not empty
    assert!(!response.sql_query.is_empty(), "SQL query should not be empty!");
    assert!(!response.query_desc.is_empty(), "Query description should not be empty!");
}
