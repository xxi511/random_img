use clap::Parser;
use futures::{future, FutureExt};
use rand::{distributions::Alphanumeric, Rng};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = 300)]
    width: i32,

    #[arg(short, default_value_t = 300, short = 'x')]
    height: i32,

    #[arg(short, default_value_t = 10)]
    count: i32,
}

// How to use   
// $ cargo run -- -w 100 -x 100 -c 10
// $ cargo run

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Download {:?} images, width: {:?}, height: {:?}", args.count, args.width, args.height);

    let mut start = 0;
    let mut end = 100;
    loop {
         if end > args.count {
            end = args.count;
        }
        let mut downloader = vec![download_image(&args).boxed()];
        for _ in start..end {
            downloader.push(download_image(&args).boxed());
        }
        let _ = future::join_all(downloader).await;
        start = end;
        if end == args.count {
            break;
        }
        end += 100;
    }
    
    Ok(())
}

async fn download_image(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://source.unsplash.com/random/{:?}x{:?}",
        args.width, args.height
    );
    let res = reqwest::get(url).await.unwrap();

    let image_url = res.url().as_str();
    let img_bytes = reqwest::get(image_url).await?.bytes().await?;
    let random_image = image::load_from_memory(&img_bytes)?;
    let filename = random_name();
    _ = random_image.save(&Path::new(&filename));
    Ok(())
}

fn random_name() -> String {
    let name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let filename = format!("{name}.png");
    return filename;
}
