extern crate image;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    width: u32,
    height: u32,
}

fn main() {
    let args = Cli::from_args();
    let width: u32 = args.width;
    let height: u32 = args.height;
    
    let mut imgbuf = ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = 128 as u8;
        let g = 128 as u8;
        let b = 128 as u8;
        *pixel = Rgb([r, g, b]);
    }

    let font = Vec::from(include_bytes!("OpenSans-Regular.ttf") as &[u8]);
    let font = Font::from_bytes(font).unwrap();

    let font_size = 20.0;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    let x = width.to_string();
    let y = height.to_string();
    let caption: &str = &(x + " x " + &y);

    draw_text_mut(
        &mut imgbuf,
        Rgb([255u8, 255u8, 255u8]),
        0,
        0,
        scale,
        &font,
        caption,
    );

    imgbuf.save("output.png").unwrap();
}
