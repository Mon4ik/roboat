use clap::Parser;
use roboat::presence::PresenceType;
use roboat::{ClientBuilder, Limit};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    user_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = ClientBuilder::new().build();

    let details = client.user_details(args.user_id).await?;
    let presence_raw = client.get_presence(vec![args.user_id]).await?;

    let presence = presence_raw.first().expect("User not found.");

    println!(
        "Presence of {} (@{})",
        details.display_name, details.username
    );
    println!(" > Currently: {:?}", presence.presence_type);

    if presence.presence_type == PresenceType::InGame {
        println!(" > Playing: {:?} (game_id)", presence.game_id);
    }

    println!(" > Last Online: {}", presence.last_online);
    println!(" > Last Location: {}", presence.last_location);
    Ok(())
}
