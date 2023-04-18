extern crate image;
extern crate clap;

use clap::Arg;
use std::default::Default;
use std::io::{Read, BufWriter};
use std::fs::File;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod style;
pub mod painting;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
                            .version(concat!("v", env!("CARGO_PKG_VERSION")))
                            .about(env!("CARGO_PKG_DESCRIPTION"))
                            .arg(Arg::with_name("HTML")
                                     .help("Html file")
                                     .index(1)
                                     .required(true))
                            .arg(Arg::with_name("CSS")
                                     .help("Additional CSS file")
                                     .index(2)
                                     .required(true))
                            .arg(Arg::with_name("OUTPUT")
                                     .help("Output file name")
                                     .short("o")
                                     .default_value("output.png"))
                            .get_matches();

    let html = read_source(String::from(matches.value_of("HTML").unwrap()));
    let css = read_source(String::from(matches.value_of("CSS").unwrap()));
    let output = String::from(matches.value_of("OUTPUT").unwrap());

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width  = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let canvas = painting::paint(&layout_root, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
        let color = canvas.pixels[(y * w + x) as usize];
        image::Pixel::from_channels(color.r, color.g, color.b, color.a)
    });

    let mut file = BufWriter::new(File::create(&output).unwrap());
    image::ImageRgba8(img).save(&mut file, image::PNG).unwrap();
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
