pub mod configs;
pub mod traits;

use std::{env::current_dir, path::{absolute, Path, PathBuf}};

use traits::builder::Builder;

use image::{ImageBuffer, Pixel, Rgb};
use imageproc::drawing::Canvas;
use qrcodegen::{QrCode, QrCodeEcc};
use configs::{image_config::{ImageConfig, ImageConfigBuilder}, logo_config::{LogoConfig, LogoConfigBuilder}};

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

pub struct QrWatermark {
  qr_code: QrCode,
  logo_path: PathBuf,
  qr_code_config: QrCodeConfig,
  image_config: ImageConfig,
  logo_config: LogoConfig
}

impl<'a> QrWatermark {

  pub fn new(text: &'a str, logo_path: &'a str) -> Self {
    let qr_code = QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).unwrap();
    let logo_path_buf = PathBuf::from(logo_path);

    Self {
      qr_code,
      logo_path: logo_path_buf,
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build(),
      logo_config: LogoConfigBuilder::new().build()
    }
  }

  #[allow(unused)]
  fn generate_color_gradient(&mut self) {
    !unimplemented!()
  }

  fn generate_image(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_size = self.qr_code.size() as u32
      * self.image_config.pixel_size
      + (self.image_config.margin_size * self.image_config.pixel_size) * 2;

    let mut image = ImageBuffer::from_fn(image_size, image_size, |x, y| {
      let x_with_margin = x as i32 - (self.image_config.margin_size * self.image_config.pixel_size) as i32;
      let y_with_margin = y as i32 - (self.image_config.margin_size * self.image_config.pixel_size) as i32;

      let mut module_x = x_with_margin;
      let mut module_y = y_with_margin;

      if x_with_margin > 0 {
        module_x = x_with_margin / self.image_config.pixel_size as i32;
      }

      if y_with_margin > 0 {
        module_y = y_with_margin / self.image_config.pixel_size as i32;
      }

      if self.qr_code.get_module(module_x, module_y) {
        self.image_config.color
      } else {
        self.image_config.background_color
      }
    });

    let logo = image::open(&self.logo_path).unwrap();

    let logo_thumbnail = logo.thumbnail(
      self.logo_config.width,
      self.logo_config.height);

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

  pub fn image_config(mut self, image_config: ImageConfig) -> Self {
    self.image_config = image_config;
    self
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

impl<'a> Default for QrWatermark {
  fn default() -> Self {
    let qr_code = QrCode::encode_text("Hello this is QrWatermark", qrcodegen::QrCodeEcc::Medium).unwrap();

    let mut logo_buf_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    logo_buf_path.push("imgs/rust_logo.png");

    Self {
      qr_code,
      logo_path: logo_buf_path,
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build(),
      logo_config: LogoConfigBuilder::new().build()
    }
  }
}
