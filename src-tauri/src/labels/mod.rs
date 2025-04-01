// File: src/labels/mod.rs
use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use barcoders::generators::image::Image;
use barcoders::sym::code128::*;
use chrono::format;
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::typst_renderer::{LabelData, LabelStyle};

/// Generate a series of barcode labels with the format "FaSPyN-2025-XXXX"
pub fn generate_labels(label_data: &LabelData) -> Result<String, Box<dyn std::error::Error>> {
    let mut images = Vec::new();
    fs::create_dir_all("src/assets/img/temp")?;

    let background_path = match label_data.style {
        LabelStyle::Type1 => "src/assets/templates/tag01.png",
        LabelStyle::Type2 => "src/assets/templates/tag02_v1.png",
        LabelStyle::CustomType(ref path) => path,
    };

    let mut start = label_data.start;
    for i in 0..label_data.amount {
        let text = format!("{}-2025-{:04}", label_data.dependence, start);
        let image_path = format!("src/assets/img/temp/temp_label_result_{}.png", i+1);
        
        generate_single_label(&text, background_path, &image_path)?;
        images.push(image_path);
        start += 1;
    }
    
    println!("generate labels, antes del template");
    let template = include_str!("../assets/templates/etiquetas.typ");

    // Ok(images)
    Ok(template.to_string())
}

/// Generate a single barcode label
pub fn generate_single_label(
    text: &str,
    background_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let barcode = Code128::new(format!("\u{0181}{}", text)).unwrap();
    let png = Image::png(60);
    let encoded = barcode.encode();

    let bytes = png.generate(&encoded[..]).unwrap();

    let mut temp_barcode_path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    temp_barcode_path.push("src/assets/img/temp/temp_barcode.png");

    let file = File::create(&temp_barcode_path).unwrap();
    {
        let file = File::create(&Path::new(&temp_barcode_path)).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&bytes[..]).unwrap();
        writer.flush().unwrap();
    }

    let mut background = image::open(background_path)?;
    let mut foreground = image::open(&temp_barcode_path)?;

    make_background_transparent(&mut foreground);
    let wide_foreground = generate_wide_barcode(&foreground, 2, 0)?;
    let (foreground, x) = add_text(text.to_string(), wide_foreground)?;
    
    overlay_images(&mut background, &foreground, x, 196)?;
    
    // Save the result
    background.save(output_path)?;

    std::fs::remove_file(temp_barcode_path).ok();

    Ok(())
}

fn generate_wide_barcode(original: &DynamicImage, bar_width_multiplier: u32, bar_spacing: u32) -> Result<DynamicImage, ImageError> {
    let (fg_width, fg_height) = original.dimensions();
    
    // Reduce multiplier and spacing
    let bar_width_multiplier = bar_width_multiplier.max(1);
    let bar_spacing = bar_spacing.min(1); // Limit spacing to 1 pixel maximum

    // Calculate new dimensions with tighter spacing
    let new_width = fg_width.saturating_mul(bar_width_multiplier).saturating_add(bar_spacing * fg_width);
    
    // Create a new image with the new width
    let mut wide_barcode = RgbaImage::new(new_width, fg_height);
    
    // Initialize with transparency
    for pixel in wide_barcode.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 0]);
    }
    
    // Draw wider bars with minimal spacing
    for y in 0..fg_height {
        for x in 0..fg_width {
            let pixel = original.get_pixel(x, y);
            
            // If the pixel is not white (it's part of a bar)
            if pixel[0] < 240 || pixel[1] < 240 || pixel[2] < 240 {
                // Draw wide bar with minimal spacing
                for bx in 0..bar_width_multiplier {
                    let new_x = (x * (bar_width_multiplier + bar_spacing)) + bx;
                    if new_x < new_width {
                        wide_barcode.put_pixel(new_x, y, pixel);
                    }
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageRgba8(wide_barcode))
}

fn add_text(text: String, img: DynamicImage) -> Result<(DynamicImage, u32), Box<dyn std::error::Error>> {
    let text_height = 25;
    let margin = 5;
    let (width, height) = img.dimensions();
   
    // Safe offset calculation
    let x: u32 = if width > 430 {
        // If width is greater than expected, use a different offset
        width / 2
    } else {
        215 - (width / 2)
    };
    let x = x + 160;
    
    // Create a new image with additional height for text
    let mut img_with_text = RgbaImage::new(
        width,
        height + text_height + margin
    );
    
    // Initialize with transparency
    for pixel in img_with_text.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 0]);
    }
    
    // Copy the barcode to the new image
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            img_with_text.put_pixel(x, y, pixel);
        }
    }
    
    // Load font
    let font_data = include_bytes!("../assets/fonts/MYRIADPRO-SEMIBOLD.OTF");
    let font = FontArc::try_from_slice(font_data).expect("Error loading font");
    
    // Set up scale and text color
    let scale = PxScale::from(25.0);
    let scaled_font = font.as_scaled(scale);
    let color = Rgba([0, 0, 0, 255]);
    
    // Calculate total width of rendered text
    let text_width = text.chars()
        .map(|c| scaled_font.h_advance(scaled_font.glyph_id(c)))
        .sum::<f32>();
    
    // Calculate X position to center text
    let text_x = (width as i32 - text_width as i32) / 2;
    let text_y = height + margin;
    
    // Draw text
    draw_text_mut(
        &mut img_with_text,
        color,
        text_x,
        text_y as i32,
        scale,
        &font,
        &text
    );
    
    // Convert image with text to DynamicImage
    let barcode_with_text = DynamicImage::ImageRgba8(img_with_text);
    
    Ok((barcode_with_text, x))
}

fn make_background_transparent(img: &mut DynamicImage) {
    // Convert to RGBA if not already
    if img.color().has_alpha() == false {
        *img = img.to_rgba8().into();
    }
    
    let (width, height) = img.dimensions();
    
    // Convert white or near-white pixels to transparent
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            
            // If pixel is white or near-white
            if pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240 {
                // Make pixel transparent
                img.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], 0]));
            }
        }
    }
}

fn overlay_images(
    background: &mut DynamicImage,
    foreground: &DynamicImage,
    x_offset: u32,
    y_offset: u32,
) -> Result<(), ImageError> {
    let (fg_width, fg_height) = foreground.dimensions();
    let (bg_width, bg_height) = background.dimensions();
    
    // Adjust offsets if they would go out of bounds
    let safe_x_offset = x_offset.min(bg_width.saturating_sub(fg_width));
    let safe_y_offset = y_offset.min(bg_height.saturating_sub(fg_height));
    
    // Copy pixels from foreground image to background image
    for y in 0..fg_height {
        for x in 0..fg_width {
            let pixel = foreground.get_pixel(x, y);
            
            // Only copy if pixel is not completely transparent
            if pixel[3] > 0 {
                // Use safe coordinates
                let bg_x = x + safe_x_offset;
                let bg_y = y + safe_y_offset;
                
                if bg_x < bg_width && bg_y < bg_height {
                    background.put_pixel(bg_x, bg_y, pixel);
                }
            }
        }
    }
    
    Ok(())
}