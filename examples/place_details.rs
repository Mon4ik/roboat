use clap::Parser;

use roboat::ClientBuilder;
use roboat::presence::PresenceType;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,

    #[arg(long, short)]
    place_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let place_details = client
        .place_details(args.place_id)
        .await?;

    println!("{}", place_details.source_name);

    for desc_line in place_details.source_description.split("\n") {
        println!(" {}", desc_line);
    }

    println!();

    println!("URL: {}", place_details.url);

    Ok(())
}
