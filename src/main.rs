extern crate image;
extern crate clap;

use clap::Parser;

/// Turn images in to their emoji representation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to image
    #[arg(short, long)]
    path: String,

    /// Size of image
    #[arg(short, long, default_value_t = 25)]
    size: u8,
}

fn main() {
    let args = Args::parse();

    // open and resize the image
    let img = image::open(args.path).unwrap();
    let img: image::DynamicImage = img.thumbnail(args.size as u32, args.size as u32);
    let img: &image::RgbImage = img.as_rgb8().unwrap();

    println!("Pixel at (20,20) is {:?}", img[(20, 20)]);
}
