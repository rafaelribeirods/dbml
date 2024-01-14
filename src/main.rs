use cli::Cli;

mod cli;
mod commander;

fn main() {
    let cli: Cli = cli::parse();
    if let Err(err) = commander::execute(cli) {
        println!("Error: {}", err)
    }
}
