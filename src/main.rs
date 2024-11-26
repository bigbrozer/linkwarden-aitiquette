use app::Application;

mod api;
mod app;
mod models;
mod prompts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app: Application = Application::new();
    app.run().await;

    Ok(())
}
