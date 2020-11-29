#[cfg(feature = "exe")]
fn main() {
    exe::main();
}

#[cfg(not(feature = "exe"))]
fn main() {
    println!("Must be compiled with the exe feature to enable the binary.");
}

#[cfg(feature = "exe")]
mod exe {
    extern crate clap;
    extern crate euclid;
    extern crate font_kit;
    extern crate image;
    extern crate pathfinder_geometry;

    use std::fs::File;

    use self::clap::{App, Arg, ArgMatches};
    use self::euclid::Size2D;
    use self::font_kit::canvas::{Canvas, Format, RasterizationOptions};
    use self::font_kit::font::Font;
    use self::font_kit::hinting::HintingOptions;
    use self::image::Luma;
    use self::pathfinder_geometry::{rect::RectI, transform2d::Transform2F};
    use std::char;

    struct Glyph {
        id: u32,
        raster_rect: RectI,
        c: char,
    }

    fn get_args() -> ArgMatches<'static> {
        let dumpglyph_arg = Arg::with_name("dumpglyph")
            .help("Dump glyph in terminal")
            .short("d")
            .long("dumpglyph");

        let ttf_filename_arg = Arg::with_name("TTF")
            .help("TTF file name")
            .required(true)
            .index(1);
        let png_prefix_arg = Arg::with_name("PNG-PREFIX")
            .help("Prefix for PNG")
            .required(true)
            .index(2);
        let size_arg = Arg::with_name("SIZE")
            .help("Font size in blocks")
            .default_value("32")
            .index(3);

        App::new("create-font")
            .version("0.1")
            .about("Simple tool to create image from TTF for creating font for embedded-graphics`")
            .arg(ttf_filename_arg)
            .arg(dumpglyph_arg)
            .arg(png_prefix_arg)
            .arg(size_arg)
            .get_matches()
    }

    pub fn main() {
        let matches = get_args();

        let ttf_filename = matches.value_of("TTF").unwrap();
        let png_prefix = matches.value_of("PNG-PREFIX").unwrap();
        let font_size: f32 = matches.value_of("SIZE").unwrap().parse().unwrap();
        let dump_glyph_enable  = matches.is_present("dumpglyph");

        let mut file = File::open(ttf_filename).unwrap();

        let font = Font::from_file(&mut file, 0).expect("error loading font");

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
                            Transform2F::default(),
                            HintingOptions::None,
                            RasterizationOptions::Bilevel,
                        )
                        .expect("unable to get raster bounds");
                    Glyph {
                        id: glyph_id,
                        raster_rect,
                        c: chr,
                    }
                })
            })
            .collect();

        // Work out how big the glyphs are
        let char_size = Size2D::new(
            glyphs
                .iter()
                .map(|glyph| {
                    glyph
                        .as_ref()
                        .map(|glyph| glyph.raster_rect.width())
                        .unwrap_or(0)
                })
                .max()
                .unwrap(),
            glyphs
                .iter()
                .map(|glyph| {
                    glyph
                        .as_ref()
                        .map(|glyph| glyph.raster_rect.height())
                        .unwrap_or(0)
                })
                .max()
                .unwrap(),
        )
        .to_u32();
        println!("Size {:?}", char_size);
        // Render the glyphs
        let row_size = 32;
        let img_size = Size2D::new(
            char_size.width * row_size,
            char_size.height * ((glyphs.len() as u32 + row_size - 1) / row_size),
        );
        let mut imgbuf = image::GrayImage::new(img_size.width, img_size.height);

        for (i, (_chr, glyph)) in all_chars.iter().zip(glyphs.iter()).enumerate() {
            if let Some(glyph) = glyph {
                let mut canvas = Canvas::new(glyph.raster_rect.size(), Format::A8);

                font.rasterize_glyph(
                    &mut canvas,
                    glyph.id,
                    font_size,
                    Transform2F::from_translation(-glyph.raster_rect.origin().to_f32()),
                    HintingOptions::None,
                    RasterizationOptions::Bilevel,
                )
                .expect("error rasterizing glyph");


                if dump_glyph_enable {
                    dump_glyph(glyph, &canvas);
                }

                let col = i as u32 % row_size;
                let row = i as u32 / row_size;
                let img_x = col * char_size.width;
                let img_y = row * char_size.height + char_size.height;

                // Copy onto image
                for y in (0..glyph.raster_rect.height()).into_iter().rev() {
                    let (row_start, row_end) =
                        (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
                    let row = &canvas.pixels[row_start..row_end];
                    for x in 0..glyph.raster_rect.width() {
                        let val = row[x as usize];

                        if val != 0 {
                            let pixel_x = img_x as i32 + x as i32 + glyph.raster_rect.origin().x();
                            let pixel_y = img_y as i32 - glyph.raster_rect.height() + y as i32;

                            if pixel_x >= 0 && pixel_y >= 0 {
                                imgbuf.put_pixel(pixel_x as u32, pixel_y as u32, Luma([0xFFu8]));
                            }
                        }
                    }
                }
            }
        }

        let filename = format!("{}{}Point.png", png_prefix, font_size);
        imgbuf.save(&filename).expect("error saving PNG");
        println!("Wrote {} with character size of {}", filename, char_size);
    }

    fn shade(value: u8) -> char {
        match value {
            0 => ' ',
            1..=84 => '░',
            85..=169 => '▒',
            170..=254 => '▓',
            _ => '█',
        }
    }

    fn dump_glyph(glyph: &Glyph, canvas: &Canvas) {
        println!("char '{}'", glyph.c);

        for y in 0..glyph.raster_rect.height() {
            let mut line = String::new();
            let (row_start, row_end) =
                (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
            let row = &canvas.pixels[row_start..row_end];
            for x in 0..glyph.raster_rect.width() {
                let shade = shade(row[x as usize]);
                line.push(shade);
                line.push(shade);
            }
            println!("{}", line);
        }
    }
}
