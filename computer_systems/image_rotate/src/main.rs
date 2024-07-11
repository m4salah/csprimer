// rotate bit map image challenge
//
use std::{
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
};

fn rotate_image() -> Result<(), Box<dyn Error>> {
    let bmp = fs::read("image-rotate/teapot.bmp")?;

    // Bitmap file header: 14 bytes
    // 2 bytes is the header field
    println!("Header Field: {}", String::from_utf8_lossy(&bmp[0..2]));

    // 4 bytes indicate the size of the bmp file
    let mut size = [0u8; 4];
    size.copy_from_slice(&bmp[2..6]);
    let size = u32::from_le_bytes(size);
    println!("File Size: {}", size);

    // 4 bytes reserverd value 2 bytes each
    println!("Reserved 1: {:?}", &bmp[6..8]);
    println!("Reserved 2: {:?}", &bmp[8..10]);

    // 00000000 00000100
    // 00000100 00000000
    // 4 bytes indicate the byte where the bitmap image data (pixel array) can be found.
    let mut pixel_array_offset = [0u8; 4];
    pixel_array_offset.copy_from_slice(&bmp[10..14]);
    let pixel_array_offset = u32::from_le_bytes(pixel_array_offset);
    println!("Pixel array offset: {}", pixel_array_offset);
    // End of the header file 14 bytes total

    // DIB header (bitmap information header)
    //
    // 4 bytes for dib header size
    let mut header_size = [0u8; 4];
    header_size.copy_from_slice(&bmp[14..18]);
    let header_size = u32::from_le_bytes(header_size);
    println!("DIB Header Size: {}", header_size);

    // 4 bytes for width in pixel
    let mut width = [0u8; 4];
    width.copy_from_slice(&bmp[18..22]);
    let width = i32::from_le_bytes(width);
    println!("Width: {}", width);

    // 4 bytes for height in pixel
    let mut height = [0u8; 4];
    height.copy_from_slice(&bmp[22..26]);
    let height = i32::from_le_bytes(height);
    println!("Height: {}", height);

    // 2 bytes for the color planes
    let mut color_planes = [0u8; 2];
    color_planes.copy_from_slice(&bmp[26..28]);
    let color_planes = u16::from_le_bytes(color_planes);
    println!("Color Planes: {}", color_planes);

    // 2 bytes for bits per pixel
    let mut bits_per_pixel = [0u8; 2];
    bits_per_pixel.copy_from_slice(&bmp[28..30]);
    let bit_per_pixel = u16::from_le_bytes(bits_per_pixel);
    println!("Bits Per Pixel: {}", bit_per_pixel);

    // 4 bytes for compression method
    let mut compression_method = [0u8; 4];
    compression_method.copy_from_slice(&bmp[30..34]);
    let compression_method = u32::from_le_bytes(compression_method);
    println!("compression_method: {}", compression_method);

    // 4 bytes for the image size, This is the size of the raw bitmap data
    let mut image_size = [0u8; 4];
    image_size.copy_from_slice(&bmp[34..38]);
    let image_size = i32::from_le_bytes(image_size);
    println!("Image Size: {}", image_size);

    // 4 bytes the horizontal resolution of the image. (pixel per metre, signed integer)
    let mut horizontal_resolution = [0u8; 4];
    horizontal_resolution.copy_from_slice(&bmp[38..42]);
    let horizontal_resolution = u32::from_le_bytes(horizontal_resolution);
    println!("horizontal_resolution: {}", horizontal_resolution);

    // 4 bytes the vertical resolution of the image. (pixel per metre, signed integer)
    let mut vertical_resolution = [0u8; 4];
    vertical_resolution.copy_from_slice(&bmp[42..46]);
    let vertical_resolution = u32::from_le_bytes(vertical_resolution);
    println!("vertical_resolution: {}", vertical_resolution);

    // 4 bytes, the number of colors in the color palette, or 0 to default to 2^n
    let mut color_palette = [0u8; 4];
    color_palette.copy_from_slice(&bmp[46..50]);
    let color_palette = u32::from_le_bytes(color_palette);
    println!("color_palette: {}", color_palette);

    // 4 bytes, the number of important colors used,
    // or 0 when every color is important; generally ignored
    let mut important_colors = [0u8; 4];
    important_colors.copy_from_slice(&bmp[50..54]);
    let important_colors = u32::from_le_bytes(important_colors);
    println!("important_colors: {}", important_colors);
    // End of DIB Header

    // The Row Size = (BitsPerPixe * ImageWidth / 32) * 4
    let row_size = ((bit_per_pixel as i32 * width) / 32) * 4;
    println!("Row Size: {}", row_size);

    // The PixelArraySize = Image Size = RowSize * |image_height|
    let pixel_array_size = row_size * height;
    println!("PixeArraySize = Image Size: {}", pixel_array_size);

    let mut result = vec![0u8; (width * height * 3) as usize];

    // Extract the header file + the DIB header.
    let mut header = bmp[..pixel_array_offset as usize].to_vec();

    // Extract the pixel array.
    let pixel_array = bmp[pixel_array_offset as usize..].to_vec();
    for y in 0..height {
        for x in 0..width {
            let src_offset = ((y * width + x) * 3) as usize;
            let dest_offset = (((width - x - 1) * height + y) * 3) as usize;

            result[dest_offset] = pixel_array[src_offset];
            result[dest_offset + 1] = pixel_array[src_offset + 1];
            result[dest_offset + 2] = pixel_array[src_offset + 2];
        }
    }

    let file = File::create("test_rotated_image.bmp")?;
    let mut writer = BufWriter::new(file);
    // write the header without anychange

    header[18..22].copy_from_slice(&height.to_le_bytes());
    header[22..26].copy_from_slice(&width.to_le_bytes());
    // println!("{:?}", &pixel_array[((403 * 3) + 4)..(403 * 3 + 3) * 2]);
    writer.write_all(&header)?;
    // write the pixel modified pixel array
    writer.write(&result)?;
    writer.flush()?;
    // println!("{:?}", result);
    Ok(())
}

/// End of the rotate bit map image challenge
fn main() -> Result<(), Box<dyn Error>> {
    rotate_image()
}
