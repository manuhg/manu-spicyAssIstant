use serde_json::Value;
use spiceai::StreamExt;
use crate::clients::get_necessary_clients;
use crate::llm_connect::talk_to_llm;
use crate::constants::MAX_SQL_QUERY_STR_LEN;
use crate::models::AssistantResponse;

pub async fn assist_user(user_query: String) -> Result<AssistantResponse, String> {
    let clients = get_necessary_clients().await;
    let mut spiceai_client = clients.spice_ai;

    let llm_resp = talk_to_llm(user_query, clients.open_ai).await
        .expect("Unable to fetch response from the LLM")
        .expect("No response from LLM!");
    if llm_resp.sql_query.is_empty() || llm_resp.sql_query.len() > MAX_SQL_QUERY_STR_LEN {
        return Err("SQL query obtained from LLM is unsafe or faulty!".to_string());
    }


    let mut results: Vec<Value> = Vec::new(); // Vector to hold the extracted data


    let mut flight_data_stream = spiceai_client.query(llm_resp.sql_query.as_str()).
        await.expect("Exception fetching data from spice client");

    while let Some(batch) = flight_data_stream.next().await {
        match batch {
            // Write the record batch out as a JSON array

            Ok(batch) => {
                /* process batch */
                let buf = Vec::new();
                let mut writer = arrow_json::LineDelimitedWriter::new(buf);
                writer.write_batches(&vec![&batch]).unwrap();
                writer.finish().unwrap();
                let buf = writer.into_inner();
                let parsed_json: Value = serde_json::from_str(String::from_utf8(buf).unwrap().as_str()).expect("Invalid JSON entry in data");
                results.push(parsed_json);
            }
            Err(e) => {
                println!("Error fetching flight stream data {}", e)
            }
        };
    }

    Ok(AssistantResponse { message: llm_resp.query_desc, data: results })
}

