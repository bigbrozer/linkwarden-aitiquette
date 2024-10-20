mod api;
mod prompts;

use clap::Parser;
use log::{debug, info};

use crate::api::{Link, Linkwarden};

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

    info!("Starting linkwarden-aitiquette 🚀");

    let args = Args::parse();

    let lw: Linkwarden = Linkwarden::new(
        String::from("https://link.vinzworld.fr"),
        args.linkwarden_token,
        args.openai_endpoint,
        args.openai_key,
    );

    info!("Fetching all links from the instance... please wait.");
    let all_links: Vec<Link> = lw.get_all_links().await;
    info!("This instance has {} links.", all_links.len());
    debug!("Content: {:?}", lw.summarize(&all_links[5]).await.unwrap());
    debug!("Content: {:?}", lw.tag(&all_links[5]).await.unwrap());

    Ok(())
}
