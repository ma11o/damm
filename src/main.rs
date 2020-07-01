extern crate image;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inputx = String::from(&args[1]);
    let inputy = String::from(&args[2]);
    let imgx: u32 = inputx.parse().unwrap();
    let imgy: u32 = inputy.parse().unwrap();

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = 128 as u8;
        let g = 128 as u8;
        let b = 128 as u8;
        *pixel = Rgb([r, g, b]);
    }

    let font = Vec::from(include_bytes!("OpenSans-Regular.ttf") as &[u8]);
    let font = Font::from_bytes(font).unwrap();

    let height = 12.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    let x = imgx.to_string();
    let y = imgy.to_string();
    let caption: &str = &(x + " x " + &y);

    draw_text_mut(
        &mut imgbuf,
        Rgb([255u8, 255u8, 255u8]),
        imgx / 2,
        imgy / 2,
        scale,
        &font,
        caption,
    );
    
    imgbuf.save("output.png").unwrap();
}
