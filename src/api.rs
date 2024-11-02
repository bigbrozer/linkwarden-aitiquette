use crate::models::Link;

use anyhow::{anyhow, Result};
use log::{debug, trace};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use reqwest::{Client, RequestBuilder, Response};
use serde_json::Value as JsonValue;

use crate::prompts;

pub struct Linkwarden {
    client: Client,
    openai_client: OpenAIClient,
    base_url: String,
    token: String,
}

impl Linkwarden {
    pub fn new(
        base_url: String,
        token: String,
        openai_endpoint: String,
        openai_key: String,
    ) -> Self {
        Self {
            client: Client::new(),
            openai_client: OpenAIClient::new_with_endpoint(openai_endpoint, openai_key),
            base_url,
            token,
        }
    }

    pub async fn get_links_at_cursor(&self, cursor: i64) -> Result<Vec<JsonValue>> {
        let url: String = format!("{}/api/v1/links?cursor={}&sort=1", self.base_url, cursor);
        let request: RequestBuilder = self.client.get(url).bearer_auth(&self.token);

        let response: Response = match request.send().await?.error_for_status() {
            Ok(resp) => {
                trace!("Got response from Linkwarden: {:?}", resp);
                resp
            }
            Err(err) => {
                return Err(anyhow!(format!(
                    "Error getting links at cursor {}: {}",
                    cursor, err
                )));
            }
        };

        let json_response: JsonValue = response.json::<JsonValue>().await?;
        let json_output: Vec<JsonValue> = match json_response["response"].as_array() {
            Some(links) => links.to_vec(),
            None => return Err(anyhow!("No links found at cursor {}", cursor)),
        };

        Ok(json_output)
    }

    pub async fn get_all_links(&self) -> Result<Vec<Link>> {
        let mut json_links: Vec<JsonValue> = Vec::new();
        let mut links: Vec<Link> = Vec::new();
        let mut cursor: i64 = 0;

        while json_links.last().is_some() || json_links.is_empty() {
            let next_page: Vec<JsonValue> = self.get_links_at_cursor(cursor).await?;
            trace!("{:#?}", next_page);
            debug!("Loaded {} new links.", next_page.len());
            cursor = match next_page.last() {
                Some(link) => link["id"].as_i64().unwrap(),
                None => break,
            };
            debug!("Next cursor set to: {}", cursor);
            json_links.extend(next_page);
        }

        for link in json_links {
            trace!("Raw link: {:#?}", link);
            let convert_to_link: Link = serde_json::from_value(link)?;
            trace!("Serialized link: {:#?}", convert_to_link);
            links.push(convert_to_link);
        }

        Ok(links)
    }

    pub async fn summarize(&self, link: &Link) -> Result<String> {
        let req = ChatCompletionRequest::new(
            "llama3.2:3b".to_string(),
            vec![prompts::build_summary(), prompts::for_link(link)],
        )
        .temperature(0.0);

        let result: ChatCompletionResponse = match self.openai_client.chat_completion(req).await {
            Ok(result) => result,
            Err(error) => return Err(anyhow!("Error from OpenAI: {}", error)),
        };
        trace!("Summary for link id: {}. Results: {:?}", link.id, result);

        let response: String = result.choices[0]
            .message
            .content
            .clone()
            .expect("Unexpected error: empty response from OpenAI !");

        Ok(response)
    }

    pub async fn tag(&self, link: &Link, summary: String) -> Result<Vec<String>> {
        let req = ChatCompletionRequest::new(
            "llama3.2:3b".to_string(),
            vec![
                prompts::build_tagging(),
                prompts::for_link_with_summary(link, summary),
            ],
        )
        .temperature(0.0);

        let result: ChatCompletionResponse = match self.openai_client.chat_completion(req).await {
            Ok(result) => result,
            Err(error) => return Err(anyhow!("Error from OpenAI: {}", error)),
        };
        trace!("Tags for link id: {}. Results: {:?}", link.id, result);

        let response: String = result.choices[0]
            .message
            .content
            .clone()
            .expect("Unexpected error: empty response from OpenAI !");
        let tags: Vec<String> =
            serde_json::from_str(&response).expect("Unexpected error: invalid JSON from OpenAI !");

        Ok(tags)
    }
}
