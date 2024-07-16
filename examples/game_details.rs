use clap::Parser;

use roboat::ClientBuilder;
use roboat::presence::PresenceType;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    universe_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .build();

    let game_details = client
        .game_details(args.universe_id)
        .await?;

    println!("{}", game_details.source_name);

    for desc_line in game_details.source_description.split("\n") {
        println!(" {}", desc_line);
    }

    println!();

    println!("{} ppl. playing", game_details.playing);

    Ok(())
}
