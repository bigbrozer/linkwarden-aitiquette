mod api;
mod models;
mod prompts;

use crate::api::Linkwarden;
use crate::models::Link;
use clap::Parser;
use log::{debug, info};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Parser, Debug)]
struct Args {
    /// The base URL of the Linkwarden instance
    #[arg(short = 'b', long, env)]
    linkwarden_base_url: String,

    /// The token to use for authenticating on Linkwarden instance
    #[arg(short = 't', long, env)]
    linkwarden_token: String,

    /// The URL to the OpenAI compatible endpoint
    #[arg(long, default_value_t = String::from("http://localhost:11434"), env)]
    openai_endpoint: String,

    /// The API key to use on the OpenAI compatible endpoint
    #[arg(long, default_value_t = String::from("ollama"), env)]
    openai_key: String,

    /// The name of the model to use for tagging
    #[arg(short = 'm', long, default_value_t = String::from("llama3.2:3b"), env)]
    openai_model_name: String,

    /// The language to use for tags
    #[arg(short = 'l', long, default_value_t = String::from("english"), env)]
    language: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    info!("Starting linkwarden-aitiquette 🚀");
    info!("Linkwarden instance: {}", args.linkwarden_base_url);
    info!("OpenAI endpoint: {}", args.openai_endpoint);
    info!("OpenAI model: {}", args.openai_model_name);

    let lw: Arc<Linkwarden> = Arc::new(Linkwarden::new(
        args.linkwarden_base_url,
        args.linkwarden_token,
        args.openai_endpoint,
        args.openai_key,
        args.openai_model_name,
        args.language,
    ));

    let permits: Arc<Semaphore> = Arc::new(Semaphore::new(3));

    info!("Fetching all links from the instance... please wait.");
    let mut all_links: Vec<Link> = lw.get_all_links().await.unwrap();
    info!("This instance has {} links.", all_links.len());
    all_links.truncate(5);

    let mut jhs = Vec::new();
    for link in all_links {
        let permits = permits.clone();
        let lw = lw.clone();

        let jh = tokio::spawn(async move {
            let _permit = permits.acquire().await.unwrap();
            info!("Preparing summary for link: {}", link.url);
            let summary: String = lw.summarize(&link).await.unwrap();
            let tags: Vec<String> = lw.tag(&link, &summary).await.unwrap();
            (summary, tags)
        });
        jhs.push(jh);
    }

    for jh in jhs {
        let results: (String, Vec<String>) = jh.await.unwrap();
        debug!("Summary: {:?}\nTags: {:?}", results.0, results.1);
    }

    Ok(())
}
