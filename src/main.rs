mod api;
mod models;
mod prompts;

use clap::Parser;
use log::{debug, info};

use crate::api::Linkwarden;
use crate::models::Link;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    info!("Starting linkwarden-aitiquette 🚀");
    info!("Linkwarden instance: {}", args.linkwarden_base_url);
    info!("OpenAI endpoint: {}", args.openai_endpoint);

    let lw: Linkwarden = Linkwarden::new(
        args.linkwarden_base_url,
        args.linkwarden_token,
        args.openai_endpoint,
        args.openai_key,
    );

    info!("Fetching all links from the instance... please wait.");
    let all_links: Vec<Link> = lw.get_all_links().await.unwrap();
    info!("This instance has {} links.", all_links.len());
    let link_summary: String = lw.summarize(&all_links[0]).await.unwrap();
    debug!("Summary:\n{}", link_summary);
    let tags: Vec<String> = lw.tag(&all_links[0], link_summary).await.unwrap();
    debug!("Tags: {:?}", tags);

    Ok(())
}
