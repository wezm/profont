extern crate euclid;
extern crate font_kit;
extern crate image;

use euclid::{Point2D, Rect, Size2D};
use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use image::Luma;
use std::char;
use std::env;
use std::sync::Arc;

struct Glyph {
    id: u32,
    raster_rect: Rect<i32>,
}

fn main() {
    let font_data = include_bytes!("../data/ProFontIIx.ttf").to_vec();
    let font = Font::from_bytes(Arc::new(font_data), 0).expect("error loading font");

    let font_size: f32 = std::env::args()
        .nth(1)
        .expect("font size argument must be supplied on command line")
        .parse()
        .expect("invalid font size");

    // Print latin 1 characters
    let basic = (' ' as u32..='~' as u32)
        .into_iter()
        .map(|c| char::from_u32(c).unwrap());
    let extended = ('\u{00A0}' as u32..='ÿ' as u32)
        .into_iter()
        .map(|c| char::from_u32(c).unwrap());
    let all_chars: Vec<_> = basic.chain(extended).collect();

    // Get the raster bounds of all chars
    let glyphs: Vec<_> = all_chars
        .iter()
        .map(|&chr| {
            font.glyph_for_char(chr).map(|glyph_id| {
                let raster_rect = font
                    .raster_bounds(
                        glyph_id,
                        font_size,
                        &Point2D::zero(),
                        HintingOptions::None,
                        RasterizationOptions::Bilevel,
                    ).expect("unable to get raster bounds");
                Glyph {
                    id: glyph_id,
                    raster_rect,
                }
            })
        }).collect();

    // Work out how big the glyphs are
    let char_size = Size2D::new(
        // 12, 22
        glyphs
            .iter()
            .map(|glyph| {
                glyph
                    .as_ref()
                    .map(|glyph| glyph.raster_rect.size.width)
                    .unwrap_or(0)
            }).max()
            .unwrap(),
        glyphs
            .iter()
            .map(|glyph| {
                glyph
                    .as_ref()
                    .map(|glyph| glyph.raster_rect.size.height)
                    .unwrap_or(0)
            }).max()
            .unwrap(),
    ).to_u32();

    // Render the glyphs
    let row_size = 32;
    let img_size = Size2D::new(
        char_size.width * row_size,
        (char_size.height as f64 * glyphs.len() as f64 / row_size as f64).ceil() as u32 +10,
    );
    println!("img_size={}", img_size);
    // let img_size = Size2D::new(800, 480);
    let mut imgbuf = image::GrayImage::new(img_size.width, img_size.height);

    for (i, (chr, glyph)) in all_chars.iter().zip(glyphs.iter()).enumerate() {
        if let Some(glyph) = glyph {
            let mut canvas = Canvas::new(&glyph.raster_rect.size.to_u32(), Format::A8);

            font.rasterize_glyph(
                &mut canvas,
                glyph.id,
                font_size,
                &glyph.raster_rect.origin.to_f32(),
                HintingOptions::None,
                RasterizationOptions::Bilevel,
            ).expect("error rasterizing glyph");

            let col = i as u32 % row_size;
            let row = i as u32 / row_size;
            let img_x = col * char_size.width;
            let img_y = row * char_size.height + char_size.height;
            println!(
                "'{}' i={} row={} col={} x,y=({}, {}) raster_rect={}",
                chr, i, row, col, img_x, img_y, glyph.raster_rect
            );
            // Copy onto image
            for y in (0u32..glyph.raster_rect.size.height as u32)
                .into_iter()
                .rev()
            {
                let mut line = String::new();

                let (row_start, row_end) =
                    (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
                let row = &canvas.pixels[row_start..row_end];
                for x in 0u32..glyph.raster_rect.size.width as u32 {
                    let val = row[x as usize];
                    line.push(shade(val));
                    line.push(shade(val));
                    if val != 0 {
                        let pixel_x = img_x as i32 + x as i32 + glyph.raster_rect.origin.x;
                        let pixel_y = img_y as i32 - glyph.raster_rect.size.height + y as i32
                            - glyph.raster_rect.origin.y
                            - 4;
                        if pixel_x >= 0 && pixel_y >= 0 {
                            imgbuf.put_pixel(pixel_x as u32, pixel_y as u32, Luma([0xFFu8]));
                        }
                    }
                }

                // println!("{}", line);
            }
        }
    }

    let filename = format!("data/ProFont{}Point.png", font_size);
    imgbuf.save(&filename).expect("error saving PNG");
    println!("Wrote {} with character size of {}", filename, char_size);
}

fn shade(value: u8) -> char {
    match value {
        0 => ' ',
        1...84 => '░',
        85...169 => '▒',
        170...254 => '▓',
        _ => '█',
    }
}
