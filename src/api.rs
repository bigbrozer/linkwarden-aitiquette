use log::{debug, info, trace};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::{
    chat_completion::{ChatCompletionRequest, ChatCompletionResponse},
    error::APIError as OpenAIError,
};
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::prompts;

pub struct Linkwarden {
    client: Client,
    openai_client: OpenAIClient,
    pub base_url: String,
    pub token: String,
}

impl Linkwarden {
    /// Constructor
    pub fn new(
        base_url: String,
        token: String,
        openai_endpoint: String,
        openai_key: String,
    ) -> Linkwarden {
        info!("Using Linkwarden instance: {}", base_url);
        info!("Using OpenAI endpoint: {}", openai_endpoint);
        Linkwarden {
            client: Client::new(),
            openai_client: OpenAIClient::new_with_endpoint(openai_endpoint, openai_key),
            base_url,
            token,
        }
    }

    /// Get links at a cursor
    pub async fn get_links_at_cursor(&self, cursor: i64) -> Result<Vec<JsonValue>, String> {
        let url: String = format!("{}/api/v1/links?cursor={}&sort=1", self.base_url, cursor);
        let request: RequestBuilder = self.client.get(&url).bearer_auth(&self.token);

        let response: Response = match request.send().await.unwrap().error_for_status() {
            Ok(resp) => resp,
            Err(error) => return Err(error.to_string()),
        };

        match response.json::<JsonValue>().await {
            Ok(json_output) => Ok(json_output["response"].as_array().unwrap().to_vec()),
            Err(error) => Err(format!("Could not parse JSON: {}", error)),
        }
    }

    /// Get all links from the instance
    pub async fn get_all_links(&self) -> Vec<Link> {
        let mut json_links: Vec<JsonValue> = Vec::new();
        let mut links: Vec<Link> = Vec::new();
        let mut cursor: i64 = 0;

        while json_links.last().is_some() || json_links.is_empty() {
            let next_page: Vec<JsonValue> = self.get_links_at_cursor(cursor).await.unwrap();
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
            links.push(Link {
                id: link["id"].as_i64().unwrap(),
                name: link["name"].to_string(),
                url: link["url"].to_string(),
                text_content: link["textContent"].to_string(),
                tags: link["tags"].as_array().unwrap().to_vec(),
            });
        }
        trace!("{:#?}", links);
        links
    }

    pub async fn summarize(&self, link: &Link) -> Result<ChatCompletionResponse, OpenAIError> {
        let req = ChatCompletionRequest::new(
            "llama3.2:3b".to_string(),
            vec![prompts::build_summary(), prompts::for_link(link)],
        );

        let result: ChatCompletionResponse = self.openai_client.chat_completion(req).await?;
        Ok(result)
    }

    pub async fn tag(&self, link: &Link) -> Result<ChatCompletionResponse, OpenAIError> {
        let req = ChatCompletionRequest::new(
            "llama3.2:3b".to_string(),
            vec![prompts::build_tagging(), prompts::for_link(link)],
        );

        let result: ChatCompletionResponse = self.openai_client.chat_completion(req).await?;
        Ok(result)
    }
}

#[derive(Serialize, Debug)]
pub struct Link {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "textContent")]
    pub text_content: String,
    pub tags: Vec<JsonValue>,
}
