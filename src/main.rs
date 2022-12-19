extern crate image;
extern crate clap;

use clap::Parser;

const EMOJI: [Emoji; 5] = [
    Emoji{color: (255,255,255), value: '🌝'},
    Emoji{color: (0,0,0), value: '🖤'},
    Emoji{color: (255,0,0), value: '🍅'},
    Emoji{color: (0,255,0), value: '🤢'},
    Emoji{color: (0,0,255), value: '🌊'},
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
        let x : i32 = i32::pow(self.color.0 as i32 - pixel[0] as i32, 2);
        let y : i32 = i32::pow(self.color.1 as i32 - pixel[1] as i32, 2);
        let z : i32 = i32::pow(self.color.2 as i32 - pixel[2] as i32, 2);
        f64::sqrt((x+y+z) as f64)
    }
}

fn main() {
    let args = Args::parse();

    // open and resize the image
    let img = image::open(args.path).unwrap();
    let img: image::DynamicImage = img.thumbnail(args.size as u32, args.size as u32);
    let img: &image::RgbImage = img.as_rgb8().unwrap();
    let (width, height) = img.dimensions();

    //println!("Pixel at (20,20) is {:?}", img[(20, 20)]);

    //println!("{}", EMOJI[0].distance(img[(20, 20)]));


    for i in 0..height {
        for j in 0..width {
            let mut best_dist : f64 = f64::MAX; 
            let mut best : Emoji = Emoji{color: (0,0,0), value: ' '};
            for e in EMOJI {
                let cur_dist :f64 = e.distance(img[(j,i)]);
                if cur_dist < best_dist {
                    best_dist = cur_dist;
                    best = e;
                }
            }
            print!("{}", best.value);
        }
        print!("\n")
    }
}
