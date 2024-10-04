pub mod configs;

use image::{ImageBuffer, Luma, Pixel, Rgb, Rgba};
use imageproc::drawing::{draw_cross_mut, Canvas};
use qrcodegen::{QrCode, QrCodeEcc};
use configs::image_config::{ImageConfigBuilder, ImageConfig};

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

  pub fn image_config(mut self, image_config: ImageConfig) -> Self {
    self.image_config = image_config;
    self
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
      } else {
        Rgb([255, 255, 255])
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
