use cli::Cli;

mod cli;
mod commander;
mod config;
mod db;

fn main() {
    let cli: Cli = cli::parse();
    if let Err(err) = commander::execute(cli) {
        println!("Error: {}", err)
    }
}
