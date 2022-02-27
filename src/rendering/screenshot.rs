//! Data structure representing a screenshot
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
#[derive(Debug, Clone, PartialEq)]
/// A single screencap.
pub struct Screenshot {
    /// Raw bytes that represent the screenshot; encoded as png
    payload: Arc<Vec<u8>>,
    /// Width of the image in pixels
    width: usize,
    /// Height of the image in pixels
    height: usize,
    color_encoding: ColorType,
    source_encoding: ByteSource,
}
#[derive(Debug, Clone, Copy, PartialEq)]
///Decribes pixel encoding. Equivalent to [`png::ColorType`], maybe should be removed
pub enum ColorType {
    /// Screenshot has RBGA format
    Rgba,
    /// Screenshot has RBG format
    Rgb,
}

///Describes the source of the payload bytes
///
///This controls the alignment of the payload bytes in the [`Screenshot`]
#[derive(Debug, Clone, PartialEq)]
pub enum ByteSource {
    /// payload bytes come from a headless WGPU compositor
    WGPU,
    /// payload bytes come from a png file or encoded Screenshot object
    Png,
}

impl Into<png::ColorType> for ColorType {
    fn into(self) -> png::ColorType {
        match self {
            Self::Rgb => png::ColorType::Rgb,
            Self::Rgba => png::ColorType::Rgba,
        }
    }
}

impl From<png::ColorType> for ColorType {
    fn from(color: png::ColorType) -> ColorType {
        match color {
            png::ColorType::Rgb => ColorType::Rgb,
            png::ColorType::Rgba => ColorType::Rgba,
            _ => panic!("Unsupported color"),
        }
    }
}

impl Screenshot {
    /// Create a new [`Screenshot`] object
    pub fn new(payload: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            payload: Arc::new(payload),
            width,
            height,
            color_encoding: ColorType::Rgba,
            source_encoding: ByteSource::WGPU,
        }
    }

    /// Sets the encoding field for a [`Screenshot`] object
    pub fn color_encoding(mut self, color_type: ColorType) -> Self {
        self.color_encoding = color_type;

        self
    }

    /// Creates a [`Screenshot`] object from png
    pub fn from_png<S: AsRef<std::path::Path>>(path: S) -> Result<Self, Box<dyn Error>> {
        let decoder = png::Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut payload = vec![0; reader.output_buffer_size()];
        let out = reader.next_frame(&mut payload)?;

        Ok(Self {
            payload: Arc::new(payload),
            width: out.width as usize,
            height: out.height as usize,
            color_encoding: reader.info().color_type.into(),
            source_encoding: ByteSource::Png,
        })
    }

    /// Saves the [`Screenshot`] to the input path
    pub fn save_image_to_png<S: AsRef<std::path::Path>>(&self, path: S) {
        let file = std::fs::File::create(path).unwrap();
        self.encode_png(file);
    }

    fn encode_png<W: Write>(&self, buffer: W) {
        let mut png_encoder = png::Encoder::new(buffer, self.width as u32, self.height as u32);
        png_encoder.set_depth(png::BitDepth::Eight);
        png_encoder.set_color(self.color_encoding.into());

        let bytes_per_pixel = match self.color_encoding {
            ColorType::Rgba => std::mem::size_of::<u32>(),
            ColorType::Rgb => std::mem::size_of::<u8>() * 3,
        };

        let unpadded_bytes_per_row = self.width * bytes_per_pixel;
        let align = match self.source_encoding {
            ByteSource::WGPU => wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize,
            ByteSource::Png => 0,
        };
        let padded_bytes_per_row_padding = if align != 0 {
            (align - unpadded_bytes_per_row % align) % align
        } else {
            0
        };
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;

        let mut png_writer_z = png_encoder.write_header().unwrap();
        let mut png_writer = png_writer_z
            .stream_writer_with_size(unpadded_bytes_per_row as usize)
            .unwrap();

        // from the padded_buffer we write just the unpadded bytes into the image
        for chunk in self.payload.chunks(padded_bytes_per_row) {
            png_writer
                .write_all(&chunk[..unpadded_bytes_per_row])
                .unwrap();
        }

        png_writer.finish().expect("Png writer finish failed");
    }

    /// This does a round-trip from raw data-> png data -> back to "raw frame data;
    /// The motivation for this is that the raw pixel data of a screenshot won't be equivalent to what the data in a png frame will be
    /// due to padding or other encoding limitations
    pub fn encode_png_frame(self) -> Self {
        let mut out_vec = vec![];
        self.encode_png(&mut out_vec);

        let decoder = png::Decoder::new(out_vec.as_slice());
        let mut reader = decoder.read_info().unwrap();
        let mut payload = vec![0; reader.output_buffer_size()];
        let out = reader.next_frame(&mut payload).unwrap();
        Self {
            payload: Arc::new(payload),
            width: out.width as usize,
            height: out.height as usize,
            color_encoding: reader.info().color_type.into(),
            source_encoding: ByteSource::Png,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn round_trip() {
        let payload = vec![0xfe; 4 * 512 * 512];
        let ss = Screenshot::new(payload, 512, 512).encode_png_frame();
        let temp_png = tempfile::NamedTempFile::new().expect("tempfile creation failed");
        ss.save_image_to_png(temp_png.path());
        let ss_from_file = Screenshot::from_png(temp_png.path()).expect("Decoder fail");
        assert_eq!(ss, ss_from_file);
    }

    #[test]
    fn round_trip_rgb() {
        let payload = vec![0xfe; 3 * 512 * 512];
        let ss = Screenshot::new(payload, 512, 512)
            .color_encoding(ColorType::Rgb)
            .encode_png_frame();
        let temp_png = tempfile::NamedTempFile::new().expect("tempfile creation failed");
        ss.save_image_to_png(temp_png.path());
        let ss_from_file = Screenshot::from_png(temp_png.path()).expect("Decoder fail");
        //let temp_png2 = tempfile::NamedTempFile::new().expect("tempfile creation failed");

        //ss_from_file.save_image_to_png(temp_png2.path());
        //let ss_from_file2 = Screenshot::from_png(temp_png.path()).expect("Decoder fail");
        //assert_eq!(ss_from_file2, ss_from_file);

        assert_eq!(ss, ss_from_file);
    }
}
