use clap::Parser;
use futures::future::{self, ok, ready, BoxFuture};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 300)]
    width: i32,

    #[arg(short, long, default_value_t = 300, short = 'x')]
    height: i32,

    #[arg(short, long, default_value_t = 10)]
    count: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("width: {:?}, height: {:?}", args.width, args.height);
    _ = download_image(args).await;

    Ok(())
}

async fn download_image(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://source.unsplash.com/random/{:?}x{:?}",
        args.width, args.height
    );
    let res = reqwest::get(url).await.unwrap();

    let image_url = res.url().as_str();
    let img_bytes = reqwest::get(image_url).await?.bytes().await?;
    let random_image = image::load_from_memory(&img_bytes)?;
    _ = random_image.save(&Path::new("image.png"));
    Ok(())
}