use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    place_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new().build();

    let (game_servers, next_cursor) = client.game_servers(args.place_id, None, None, None).await?;

    for game_server in game_servers {
        println!("Server \"{}\"", game_server.id);
        println!(
            "  Players: {}/{}",
            game_server.playing, game_server.max_players
        );
        println!("  Server's FPS: {}", game_server.fps);
        println!("  Server's Ping: {}ms", game_server.ping);
        println!()
    }

    Ok(())
}
