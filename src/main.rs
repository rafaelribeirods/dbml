use cli::Cli;

mod cli;
mod commander;
mod config;
mod db;
mod dbml;

#[tokio::main]
async fn main() {
    let cli: Cli = cli::parse();
    if let Err(err) = commander::execute(cli).await {
        println!("Error: {}", err)
    }
}
