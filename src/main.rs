extern crate euclid;
extern crate font_kit;
extern crate image;

use euclid::{Point2D, Size2D};
use font_kit::font::Font;
use font_kit::source::SystemSource;
use font_kit::hinting::HintingOptions;
use font_kit::canvas::{Canvas, RasterizationOptions, Format};
use image::Luma;
use std::sync::Arc;
use std::char;

fn main() {
    // let font_data = include_bytes!("../data/ProFontIIx.ttf").to_vec();
    // let font = Font::from_bytes(Arc::new(font_data), 0).expect("error loading font");
    let font = SystemSource::new().select_by_postscript_name("ProFontIIx")
        .unwrap()
        .load()
        .unwrap();

    // Get the font metrics to work out how to size the canvas
    // let metrics = font.metrics();

    // Collect all the glyphs present
    // let glyphs: Vec<_> = (0..font.glyph_count()).filter(|&glyph_id| font.typographic_bounds(glyph_id).is_ok()).collect();
    // println!("glyphs = {:?}", glyphs);

    // let mut canvas = Canvas::new(&Size2D::new(12 * font.glyph_count(), 22[>metrics.x_height.ceil() as u32<]), Format::A8);

    // let offset = Vector2D::new(12., 0.);
    // let mut origin = Point2D::zero();
    // for glyph_id in 0..font.glyph_count() {
    //     font.rasterize_glyph(&mut canvas,
    //                          glyph_id,
    //                          18.0,
    //                          &origin,
    //                          HintingOptions::None,
    //                          RasterizationOptions::Bilevel)
    //         .expect("error rasterizing glyph");

    //     // origin += offset;
    // }
    let char_size = Size2D::new(12, 22);
    let img_size = Size2D::new(char_size.width * 26, char_size.height * 4);
    let mut imgbuf = image::GrayImage::new(img_size.width, img_size.height);

    let font_size = 18f32;
    for (i, c) in (' ' as u32 ..= '~' as u32).into_iter().enumerate() {
        let chr = char::from_u32(c).unwrap(); // unwrap should be safe since u32 came from a char
        if let Some(glyph_id) = font.glyph_for_char(chr) {
            let raster_rect = font.raster_bounds(glyph_id,
                                                 font_size,
                                                 &Point2D::zero(),
                                                 HintingOptions::None,
                                                 RasterizationOptions::Bilevel)
                .expect("unable to get raster bounds");
            println!("i={} c={} chr={} glyph_id={} raster_rect={}", i, c, chr, glyph_id, raster_rect);

            let mut canvas = Canvas::new(&raster_rect.size.to_u32(), Format::A8);

            // let origin = Point2D::new(-raster_rect.origin.x, -raster_rect.origin.y).to_f32();
            // font.rasterize_glyph(&mut canvas,
            //                      glyph_id,
            //                      font_size,
            //                      &raster_rect.origin.to_f32(),
            //                      HintingOptions::None,
            //                      RasterizationOptions::Bilevel)
            //     .expect("error rasterizing glyph");

            // let col = i as u32 % 25;
            // let row = i as u32 / 25;
            // let img_x = col * char_size.width;
            // let img_y = row * char_size.height;
            // println!("'{}' i={} row={} col={} x, y=({}, {}) raster_rect={}", chr, i, row, col, img_x, img_y, raster_rect);
            // Copy onto image
            // for y in 0..raster_rect.size.height {
            //     let (row_start, row_end) = (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
            //     let row = &canvas.pixels[row_start..row_end];
            //     for x in 0..raster_rect.size.width {
            //         let val = row[x as usize];
            //         if val != 0 {
            //             imgbuf.put_pixel(img_x, img_y, Luma([0xFFu8]));
            //         }
            //     }
            // }
        }
    }

    // println!("{}x{} {}", canvas.size.width, canvas.size.height, canvas.pixels.len());
    imgbuf.save("profont18.png").expect("error saving PNG");
}
