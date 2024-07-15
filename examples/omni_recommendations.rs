use clap::Parser;
use roboat::ClientBuilder;
use roboat::discovery::TreatmentType;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let topics = client.omni_recommendations().await?;

    for topic in topics {
        if let Some(topic) = topic.topic {
            print!("{}", topic);
        } else {
            print!("");
        }

        if let Some(subtitle) = topic.subtitle {
            println!(" - {}:", subtitle);
        } else {
            println!(":");
        }

        if topic.treatment_type == TreatmentType::FriendCarousel {
            println!(" * there's should be carousel of friends *")
        } else {
            for recommendation in topic.recommendation_list {
                println!(" • {} – {} ppl.", recommendation.name, recommendation.player_count);
            }
        }
    }

    Ok(())
}
