use clap::Parser;
use roboat::thumbnails::{ThumbnailSize, ThumbnailType};
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = ThumbnailSize::S150x150;
    let thumbnail_type = ThumbnailType::AvatarHeadshot;

    let args = Args::parse();
    let client = ClientBuilder::new().build();

    // Either of these methods work, both are here just to show the two different ways to do it.
    let url = client
        .token_thumbnail_url(args.token, size, thumbnail_type)
        .await?;

    println!("Avatar headshot thumbnail url: {}", url);

    Ok(())
}
