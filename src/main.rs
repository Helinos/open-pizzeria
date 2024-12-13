use core::str;
use std::{env, fs, io::Read};

use flate2::read::ZlibDecoder;
use image::ImageBuffer;
use thiserror::Error;

struct Reader<'a> {
    data: &'a [u8],
    head: u32,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { 
            data: data,
            head: 0 
        }
    }

    fn seek(&mut self, position: u32) -> u32 {
        let head = self.head;
        self.head = position;
        head
    }

    /// Move the head by "amount" and return the head's poition before the move.
    fn skip(&mut self, amount: u32) -> u32 {
        let head = self.head;
        self.head += amount;
        head
    }

    /// Read + move the head by "amount" bytes and return the bytes as a slice
    fn read_bytes(&mut self, amount: u32) -> &[u8] {
        let head = self.skip(amount) as usize;
        &self.data[head .. head + amount as usize]
    }

    fn read_byte(&mut self) -> &u8 {
        let head = self.skip(1) as usize;
        &self.data[head]
    }

    fn readu16(&mut self) -> u16 {
        let bytes = self.read_bytes(2);
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn readu32(&mut self) -> u32 {
        let bytes = self.read_bytes(4);
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_string(&mut self) -> String {
        let mut buf = String::new();

        loop {
            let char = self.read_bytes(1)[0];
            if char == 0 {
                break;
            }
            buf.push(char::from(char));
        }

        buf
    }

    fn read_color(&mut self) -> (u8, u8, u8) {
        let r = *self.read_byte();
        let g = *self.read_byte();
        let b = *self.read_byte();
        self.skip(1);
        return (r, g, b);
    }
}

#[derive(Error, Debug)]
pub enum FileParseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::error::ImageError),
    #[error("File {0} contained an invalid executable signature.")]
    InvaildExecutableSignature(String),
    #[error("File {0} contained an invalid PE signature.")]
    InvalidPESignature(String),
    #[error("File {0} is not Five Night at Freddy's")] // TODO: More robust checking for invalid files
    NotTheGame(String),
    #[error("There was an issue trying to decompress {0} data")]
    CompressionSizeMismatch(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_none() {
        let path = &args[0];
        println!("Usage: {} path/to/a-fnaf-exe.exe", path.split('/').last().unwrap_or(path));
        return;
    }

    let path = &args[1];

    read(path).unwrap();
}

fn read(path: &str) -> Result<(), FileParseError> {
    let file_name = path.split('/').last().unwrap_or(path).to_string();
    let data = fs::read(path)?;

    let mut reader = Reader::new(&data);

    if reader.read_bytes(2) != "MZ".as_bytes() {
        return Err(FileParseError::InvaildExecutableSignature(file_name));
    }

    reader.seek(60);

    let header_offset: u32 = reader.readu32();

    reader.seek(header_offset);

    if reader.read_bytes(4) != "PE\x00\x00".as_bytes() {
        return Err(FileParseError::InvalidPESignature(file_name));
    }

    reader.skip(2);
    let number_of_sections = reader.readu16();
    reader.skip(16);

    let optional_header = 28 + 68;
    let data_dir = 16 * 8;

    reader.skip(optional_header + data_dir);

    let mut position: Option<u32> = None;

    for index in 0..number_of_sections {
        let head = reader.head;
        let name = reader.read_string();
        if name == ".extra" {
            reader.seek(head + 16 + 4);
            position = Some(reader.readu32());
            break;
        } else if index >= number_of_sections - 1 {
            reader.seek(head + 16);
            let size = reader.readu32();
            let address = reader.readu32();
            position = Some(size + address);
            break;
        }
        reader.seek(head + 40);
    }

    reader.seek(position.unwrap());

    if reader.read_bytes(8) != &[0x77, 0x77, 0x77, 0x77, 0x49, 0x87, 0x47, 0x12] {
        return Err(FileParseError::NotTheGame(file_name))
    }

    reader.skip(8);
    let _format_version = reader.readu32();
    reader.skip(8);

    let count = reader.readu32();

    for _ in 0..count {
        let filename_size = reader.readu16() as u32;
        let _file_name = reader.read_bytes(filename_size * 2);
        let _bingo = reader.readu32();
        let data_size = reader.readu32();
        reader.skip(data_size); //let data = reader.read_bytes(data_size);
    }

    let _header = reader.read_bytes(4);
    let _runtime_version = reader.readu16();
    let _runtime_subversion = reader.readu16();
    let _product_version = reader.readu32();
    let _product_build = reader.readu32();

    loop {
        let chunk_id = reader.readu16();

        if chunk_id == 0x7f7f {
            break;
        }

        let chunk_mode = reader.readu16();
        let chunk_size = reader.readu32();

        match chunk_mode {
            0 => {
                if chunk_id == 0x6666 {
                    let image_bank_data = reader.read_bytes(chunk_size);
                    let mut image_bank_reader = Reader::new(image_bank_data);

                    let number_of_items = image_bank_reader.readu32();
                    
                    for index in 0..number_of_items {
                        let _item_id = image_bank_reader.readu32();
                        
                        let decompressed_size = image_bank_reader.readu32();
                        let compressed_size = image_bank_reader.readu32();

                        let compressed_image_item_data = image_bank_reader.read_bytes(compressed_size);
                        let mut decompressed_image_item_data = Vec::new();
                        let bytes_read = ZlibDecoder::new(compressed_image_item_data).read_to_end(&mut decompressed_image_item_data)?;
                        
                        if bytes_read != decompressed_size as usize {
                            return Err(FileParseError::CompressionSizeMismatch(String::from("Image")));
                        }

                        let mut image_item_reader = Reader::new(&decompressed_image_item_data);

                        let _checksum = image_item_reader.readu32();
                        let _references = image_item_reader.readu32();
                        let size = image_item_reader.readu32();

                        let mut width = image_item_reader.readu16();
                        let height = image_item_reader.readu16();

                        let _graphic_mode = image_item_reader.read_byte();
                        let flags = image_item_reader.read_byte();
                        let has_transparency = *flags == 0x10;

                        image_item_reader.skip(2);

                        let _x_hotspot = image_item_reader.readu16();
                        let _y_hotspot = image_item_reader.readu16();

                        let _action_x = image_item_reader.readu16();
                        let _action_y = image_item_reader.readu16();

                        let _transparent_color = image_item_reader.read_color();

                        // Images are always stored with an even width
                        // Images with odd widths have a row of transparent pixels on the right edge
                        if (width % 2) == 1 {
                            width += 1;
                        }

                        let color_size = width as u32 * height as u32 * 3;

                        let color_data = image_item_reader.read_bytes(size);
                        let mut color_reader = Reader::new(color_data);

                        // Some image's alpha channels are 2 pixels wider than the image for whatever reason.
                        let weird = size - color_size != color_size / 3;

                        let mut image_buffer = ImageBuffer::new(width as u32, height as u32);
                        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                            let b = *color_reader.read_byte();
                            let g = *color_reader.read_byte();
                            let r = *color_reader.read_byte();
                            let a: u8;

                            if has_transparency {
                                let mut i = x + y * width as u32;

                                if weird && i > width as u32 {
                                    i += 2 * y;
                                }

                                a = color_data[(i + color_size) as usize]
                            } else {
                                a = u8::MAX;
                            }

                            *pixel = image::Rgba([r, g, b, a]);
                        }

                        image_buffer.save(format!("test/out/{}.png", index))?;
                    }
                } else {
                    reader.skip(chunk_size);
                }
            }
            1 => {
                reader.skip(4); // let decompressed_size = reader.readu32();
                let compressed_size = reader.readu32();
                reader.skip(compressed_size); //let compressed_data = reader.read_bytes(compressed_size);
            }
            _ => {
                reader.skip(chunk_size); //let data = reader.read_bytes(chunk_size);
            }
        }
    }

    Ok(())
}