extern crate image;

fn main() {
    
    // open and resize the image
    let img = image::open("/home/christoph/Downloads/frog.jpg").unwrap();
    let img: image::DynamicImage = img.thumbnail(100, 100);
    let img: &image::RgbImage = img.as_rgb8().unwrap();

    println!("Pixel at (20,20) is {:?}", img[(20, 20)]);
}
