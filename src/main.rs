extern crate image;
extern crate clap;

use clap::Parser;

const EMOJI: [Emoji; 2] = [
    Emoji{color: (255,255,255), value: '🌝'},
    Emoji{color: (0,0,0), value: '🌚'},
];

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

struct Emoji {
    color : (u8, u8, u8),
    value : char,
}

impl Emoji {
    /// return euclidian distance from emoji's color
    /// to the supplied rgb value
    fn distance(&self, pixel: image::Rgb<u8>) -> f64 {
       0.1 
    }
}

fn main() {
    let args = Args::parse();

    // open and resize the image
    let img = image::open(args.path).unwrap();
    let img: image::DynamicImage = img.thumbnail(args.size as u32, args.size as u32);
    let img: &image::RgbImage = img.as_rgb8().unwrap();

    println!("Pixel at (20,20) is {:?}", img[(20, 20)]);

    println!("{}", EMOJI[0].distance(img[(20, 20)]));
}
