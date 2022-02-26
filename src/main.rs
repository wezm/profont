#[cfg(feature = "exe")]
fn main() -> Result<(), anyhow::Error> {
    exe::main()
}

#[cfg(not(feature = "exe"))]
fn main() {
    panic!("profont must be compiled with the exe feature to enable the binary.")
}

#[cfg(feature = "exe")]
mod exe {
    use std::char;
    use std::env;

    use allsorts::binary::read::ReadScope;
    use allsorts::bitmap::cbdt::CBLCTable;
    use allsorts::bitmap::{BitDepth, Bitmap, BitmapGlyph, BitmapMetrics, Metrics};
    use allsorts::font::{GlyphTableFlags, MatchingPresentation};
    use allsorts::tables::FontTableProvider;
    use allsorts::tables::OpenTypeFont;
    use allsorts::{tag, Font};
    use anyhow::{anyhow, bail};
    use image::{GenericImage, GrayImage};

    const CHARS_PER_ROW: i32 = 32;

    pub fn main() -> Result<(), anyhow::Error> {
        // Read font
        let font_data = include_bytes!("../data/profontn.otb");
        let strike_size: u8 = env::args()
            .nth(1)
            .ok_or(anyhow!(
                "strike size argument must be supplied on command line"
            ))?
            .parse()
            .map_err(|_| anyhow!("invalid font size"))?;
        let font_size: u8 = env::args()
            .nth(2)
            .ok_or(anyhow!(
                "font size argument must be supplied on command line"
            ))?
            .parse()
            .map_err(|_| anyhow!("invalid font size"))?;

        let scope = ReadScope::new(font_data);
        let font_file = scope.read::<OpenTypeFont>()?;
        let table_provider = font_file.table_provider(0)?;

        // Read the tables we need
        let table = table_provider
            .table_data(tag::EBLC)?
            .ok_or(anyhow!("font does not have EBLC table"))?;
        let scope = ReadScope::new(&table);
        let eblc = scope.read::<CBLCTable<'_>>()?;

        let bitmap_infos = eblc
            .bitmap_sizes
            .iter()
            .map(|bitmap_size| bitmap_size.inner.clone())
            .collect::<Vec<_>>();

        // println!("Available strikes:");
        // for info in &bitmap_infos {
        //     println!("- {:?}", info)
        // }

        let mut font = Font::new(table_provider)?
            .ok_or_else(|| anyhow!("Unable to find suitable cmap table for character mapping"))?;
        font.set_embedded_image_filter(GlyphTableFlags::EBDT);

        // Print latin 1 characters
        let basic = (' ' as u32..='~' as u32)
            .into_iter()
            .map(|c| char::from_u32(c).unwrap());
        let extended = ('\u{00A0}' as u32..='ÿ' as u32)
            .into_iter()
            .map(|c| char::from_u32(c).unwrap());
        let all_chars = basic.chain(extended).collect::<Vec<_>>(); // TODO: Do we really need to collect these?

        let strike = bitmap_infos
            .iter()
            .find(|strike| strike.ppem_x == strike_size)
            .ok_or_else(|| {
                let sizes: String = bitmap_infos
                    .iter()
                    .map(|info| info.ppem_x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                anyhow!(
                    "Unable to find strike with size {}. Available: {}",
                    strike_size,
                    sizes
                )
            })?;

        // Determine the glyph metrics by looking up the space character
        let (space_index, _vs) =
            font.lookup_glyph_index(' ', MatchingPresentation::NotRequired, None);
        let space_image =
            match font.lookup_glyph_image(space_index, u16::from(strike.ppem_x), BitDepth::One)? {
                Some(image) => image,
                None => {
                    bail!("No image for the space character found");
                }
            };

        let mut metrics = *match space_image.metrics {
            Metrics::Embedded(ref metrics) => metrics.hori(),
            Metrics::HmtxVmtx(_) => {
                bail!("expected embedded metrics, got hmtx/vmtx metrics")
            }
        }
        .ok_or_else(|| anyhow!("missing horizontal metrics"))?;
        if strike_size == 16 {
            // The 16px font has an incorrect advance of 14, which spreads the characters out
            // too much
            metrics.advance = 10;
        }
        // dbg!(metrics);

        // Render the glyphs
        let img_width = (i32::from(metrics.advance) * CHARS_PER_ROW) as u32;
        let img_height = ((metrics.ascender - metrics.descender) as f64 * all_chars.len() as f64
            / CHARS_PER_ROW as f64)
            .ceil() as u32;
        let mut imgbuf = GrayImage::new(img_width, img_height);
        for (i, &ch) in all_chars.iter().enumerate() {
            let (glyph_index, _vs) =
                font.lookup_glyph_index(ch, MatchingPresentation::NotRequired, None);
            if glyph_index == 0 {
                bail!("No glyph for '{}'", ch);
            }

            let glyph_image = match font.lookup_glyph_image(
                glyph_index,
                u16::from(strike.ppem_x),
                BitDepth::One,
            )? {
                Some(image) => image,
                None => {
                    bail!("No image for '{}' ({})", ch, ch as u32);
                }
            };

            // eprintln!("Got bitmap for '{}': {:?}", ch, glyph_image.metrics);
            add_to_sprite_sheet(i, ch, &metrics, &glyph_image, &mut imgbuf)?;
        }

        // Write out the PNG
        let filename = format!("ProFont{}Point.png", font_size);
        imgbuf.save(&filename)?;
        println!(
            "Wrote {} with character size of {}x{}",
            filename,
            metrics.advance,
            metrics.ascender - metrics.descender
        );

        Ok(())
    }

    fn add_to_sprite_sheet(
        char_index: usize,
        ch: char,
        base_metrics: &BitmapMetrics,
        glyph_image: &BitmapGlyph,
        imgbuf: &mut GrayImage,
    ) -> Result<(), anyhow::Error> {
        // Load the glyph image data into an imgbuf
        let bitmap = match glyph_image.bitmap {
            Bitmap::Embedded(ref bitmap) if bitmap.format == BitDepth::One => bitmap,
            Bitmap::Embedded(_) => {
                bail!("got embedded image but it was not 1-bit")
            }
            Bitmap::Encapsulated(_) => {
                bail!("got encapsulated image but was expecting embedded image")
            }
        };

        let width = u32::from(bitmap.width);
        let height = u32::from(bitmap.height);
        // println!("bitmap is {}x{}", width, height);
        if width == 0 || height == 0 {
            return Ok(());
        }

        let expanded = expand_bits(u32::from(width), &bitmap.data);
        let glyph_buffer = GrayImage::from_raw(width, height, expanded).unwrap();
        // println!("loaded image for '{}'", ch);

        // Copy the image buffer to the sprite sheet
        let col = char_index as i32 % CHARS_PER_ROW;
        let row = char_index as i32 / CHARS_PER_ROW;
        // TODO: Ensure width and height are the same as all the other characters
        let metrics = match glyph_image.metrics {
            Metrics::Embedded(ref metrics) => metrics.hori(),
            Metrics::HmtxVmtx(_) => {
                bail!("expected embedded metrics, got hmtx/vmtx metrics")
            }
        }
        .ok_or_else(|| anyhow!("missing horizontal metrics"))?;
        let img_x = col * i32::from(base_metrics.advance) + i32::from(metrics.origin_offset_x);
        let row_height = i32::from(base_metrics.ascender) - i32::from(base_metrics.descender);
        let img_y = row * row_height + i32::from(base_metrics.ascender)
            - i32::from(bitmap.height)
            - i32::from(metrics.origin_offset_y);

        imgbuf.copy_from(&glyph_buffer, img_x as u32, img_y as u32)?;

        Ok(())
    }

    // https://github.com/image-rs/image/blob/183e74ea7540da9ce11e613256f8f74640ea70c5/src/utils/mod.rs#L39
    fn expand_bits(row_size: u32, buf: &[u8]) -> Vec<u8> {
        let bit_depth = 1u32;

        // Note: this conversion assumes that the scanlines begin on byte boundaries
        let mask = (1u8 << bit_depth as usize) - 1;
        let scaling_factor = 255 / ((1 << bit_depth as usize) - 1);
        let bit_width = row_size * u32::from(bit_depth);
        let skip = if bit_width % 8 == 0 {
            0
        } else {
            (8 - bit_width % 8) / u32::from(bit_depth)
        };
        let row_len = row_size + skip;
        let mut p = Vec::new();
        let mut i = 0;
        for v in buf {
            for shift in
                num_iter::range_step_inclusive(8i8 - (bit_depth as i8), 0, -(bit_depth as i8))
            {
                // skip the pixels that can be neglected because scanlines should
                // start at byte boundaries
                if i % (row_len as usize) < (row_size as usize) {
                    let pixel = (v & mask << shift as usize) >> shift as usize;
                    p.push(pixel * scaling_factor);
                }
                i += 1;
            }
        }
        p
    }
}
