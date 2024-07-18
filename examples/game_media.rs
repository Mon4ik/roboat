use clap::Parser;

use roboat::{games::GameMediaType, ClientBuilder};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    universe_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new().build();

    let game_media = client.game_media(args.universe_id).await?;

    println!("Found {} media.", game_media.len());

    for media in game_media {
        if media.asset_type == GameMediaType::Image {
            println!(" - Image: rbxassetid://{}", media.image_id.unwrap());
        } else if media.asset_type == GameMediaType::YouTubeVideo {
            println!(
                " - Youtube Video: https://youtu.be/{}",
                media.video_hash.unwrap()
            );
        }
    }

    Ok(())
}
