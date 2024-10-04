use image::{ImageBuffer, Luma, Pixel, Rgb, Rgba};
use imageproc::drawing::{draw_cross_mut, Canvas};
use qrcodegen::{QrCode, QrCodeEcc};

struct QrCodeConfig {
  ecc: QrCodeEcc,
}

impl Default for QrCodeConfig {
  fn default() -> Self {
    Self {
      ecc: QrCodeEcc::Quartile
    }
  }
}

pub struct ImageConfig {
  pixel_size: u32,
  width: u32,
  height: u32,
  rgb: Rgb<u8>
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 20,
      width: 100,
      height: 100,
      rgb: Rgb([0, 0, 0])
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  width: Option<u32>,
  height: Option<u32>,
  rgb: Option<Rgb<u8>>
}

impl ImageConfigBuilder {
  pub fn new() -> Self {
    Self {
      pixel_size: None,
      width: None,
      height: None,
      rgb: None
    }
  }

  pub fn pixel_size(mut self, size: u32) -> Self {
    self.pixel_size = Some(size);
    self
  }

  pub fn width(mut self, width: u32) -> Self {
    self.width = Some(width);
    self
  }

  pub fn height(mut self, height: u32) -> Self {
    self.height = Some(height);
    self
  }

  pub fn rgb(mut self, rgb: Rgb<u8>) -> Self {
    self.rgb = Some(rgb);
    self
  }

  pub fn build(self) -> ImageConfig {
    let image_config_default = ImageConfig::default();

    ImageConfig {
      pixel_size: self.pixel_size.unwrap_or(image_config_default.pixel_size),
      width: self.width.unwrap_or(image_config_default.width),
      height: self.height.unwrap_or(image_config_default.height),
      rgb: self.rgb.unwrap_or(image_config_default.rgb)
    }
  }

}

pub struct QrWatermark<'a> {
  qr_code: QrCode,
  logo_path: &'a str,
  qr_code_config: QrCodeConfig,
  image_config: ImageConfig
}

impl<'a> QrWatermark<'a> {

  pub fn new(text: &'a str, logo_path: &'a str) -> Self {
    let qr_code = QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).unwrap();

    Self {
      qr_code,
      logo_path,
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build()
    }
  }

  fn generate_image(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_config = &self.image_config;
    let image_size = self.qr_code.size() as u32
      * image_config.pixel_size;

    let mut image = ImageBuffer::from_fn(image_size, image_size, |x, y| {
      let module_x = (x / image_config.pixel_size) as i32;
      let module_y = (y / image_config.pixel_size) as i32;

      if self.qr_code.get_module(module_x, module_y) {
        image_config.rgb
        // Rgba([1, 85, 155, 255])
        // Luma([0u8])
      } else {
        Rgb([255, 255, 255])
        // Luma([255u8])
      }
    });

    let logo = image::open(self.logo_path).unwrap();
    let logo_thumbnail = logo.thumbnail(image_config.width, image_config.height);

    let qr_center_x = (image_size - logo_thumbnail.width()) / 2;
    let qr_center_y = (image_size - logo_thumbnail.height()) / 2;

    for x in 0..logo_thumbnail.width() {
      for y in 0..logo_thumbnail.height() {
        let pixel = logo_thumbnail.get_pixel(x, y).to_rgb();
        image.put_pixel(qr_center_x + x, qr_center_y + y, pixel);
      }
    }

    image
  }

  pub fn print_into_console(&self) {
    let n = self.qr_code.size();

    for i in 0..n {
      for j in 0..n {
        let c = if self.qr_code.get_module(i, j) { '█' } else { ' ' };
        print!("{0}{0}", c);
      }

      println!();
    }
  }

  pub fn save_as_png(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image = self.generate_image();

    image.save(path)?;

    Ok(())
  }

}

impl<'a> Default for QrWatermark<'a> {
  fn default() -> Self {
    let qr_code = QrCode::encode_text("Hello this is QrWatermark", qrcodegen::QrCodeEcc::Medium).unwrap();

    Self {
      qr_code,
      logo_path: "imgs/rust_logo.png",
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build()
    }
  }
}
