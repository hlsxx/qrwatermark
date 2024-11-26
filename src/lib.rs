pub mod configs;
pub mod traits;

use std::{error, io};
use std::path::PathBuf;

use traits::{builder::Builder, rgb::ToRgb};

use image::{DynamicImage, ImageBuffer, Pixel, Rgb};
use imageproc::drawing::Canvas;
use qrcodegen::{QrCode, QrCodeEcc};
use configs::image_config::{ImageConfig, ImageConfigBuilder, ImagePixelType};
use configs::logo_config::{LogoConfigBuilder, LogoConfig};

impl ToRgb for Vec<u8> {
  fn to_rgb(&self) -> Result<Rgb<u8>, &'static str> {
    if self.len() != 3 {
      return Err("Vector must have size of 3");
    }

    Ok(Rgb([self[0], self[1], self[2]]))
  }
}

#[allow(unused)]
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

#[allow(unused)]
pub struct QrWatermark<'a> {
  text: &'a str,
  logo_path: Option<PathBuf>,
  qr_code_config: QrCodeConfig,
  image_config: ImageConfig,
  logo_config: LogoConfig
}

impl<'a> QrWatermark<'a> {

  pub fn new(text: &'a str) -> Self {
    Self {
      text,
      logo_path: None,
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build(),
      logo_config: LogoConfigBuilder::new().build()
    }
  }

  fn open_logo_path(&self, logo_path: &PathBuf) -> Result<DynamicImage, Box<dyn error::Error>> {
    match image::open(&logo_path) {
      Ok(image) => Ok(image),
      Err(_) => {
        let message = format!("Unable to open image '{}'", logo_path.to_string_lossy());
        Err(Box::new(io::Error::new(std::io::ErrorKind::Other, message)))
      }
    }
  }

  fn generate_qr_code(&self) -> Result<QrCode, Box<dyn error::Error>> {
    let qr_code = QrCode::encode_text(self.text, qrcodegen::QrCodeEcc::Medium)?;
    Ok(qr_code)
  }

  fn set_auto_gradient_color(&mut self) {
    let new_color = [
      self.image_config.color[0].wrapping_add(1),
      self.image_config.color[1].wrapping_add(2),
      self.image_config.color[2].wrapping_add(3),
    ];

    self.image_config.color.copy_from_slice(&new_color);
  }

  fn generate_image(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn error::Error>> {
    let qr_code = self.generate_qr_code()?;

    let image_size = qr_code.size() as u32
      * self.image_config.pixel_size
      + (self.image_config.margin_size * self.image_config.pixel_size) * 2;

    let mut last_y = 0;
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
      };

      if last_y != module_y {
        if let Some((start_color, end_color)) = self.image_config.color_gradient {
          let t = y as f32 / (image_size - 1) as f32;
          let r = (start_color[0] as f32 * (1.0 - t) + end_color[0] as f32 * t) as u8;
          let g = (start_color[1] as f32 * (1.0 - t) + end_color[1] as f32 * t) as u8;
          let b = (start_color[2] as f32 * (1.0 - t) + end_color[2] as f32 * t) as u8;

          self.image_config.color.copy_from_slice(&[r, g, b]);
        } else if self.image_config.is_auto_gradient_enabled {
          last_y = module_y;
          self.set_auto_gradient_color();
        }
      }

      if qr_code.get_module(module_x, module_y) {
        Rgb::from(self.image_config.color)
      } else {
        Rgb::from(self.image_config.background_color)
      }
    });

    // Generate logo
    if let Some(logo_path) = &self.logo_path {
      let logo = self.open_logo_path(&logo_path)?;

      let logo_width = self.logo_config.width;
      let logo_height = self.logo_config.height;

      let logo_thumbnail = logo.thumbnail(logo_width, logo_height);

      let qr_center_x = (image_size - logo_thumbnail.width()) / 2;
      let qr_center_y = (image_size - logo_thumbnail.height()) / 2;

      for x in 0..logo_thumbnail.width() {
        for y in 0..logo_thumbnail.height() {
          let pixel = logo_thumbnail.get_pixel(x, y).to_rgb();
          image.put_pixel(qr_center_x + x, qr_center_y + y, pixel);
        }
      }
    }

    Ok(image)
  }

  pub fn logo(mut self, logo_path: &'a str) -> Self {
    self.logo_path = Some(PathBuf::from(logo_path));
    self
  }

  pub fn image_config(mut self, config: ImageConfig) -> Self {
    self.image_config = config;
    self
  }

  pub fn logo_config(mut self, config: LogoConfig) -> Self {
    self.logo_config = config;
    self
  }

  pub fn print_into_console(&self) -> Result<(), Box<dyn error::Error>> {
    let qr_code = self.generate_qr_code()?;
    let n = qr_code.size();

    for i in 0..n {
      for j in 0..n {
        let c = if qr_code.get_module(i, j) { '█' } else { ' ' };
        print!("{0}{0}", c);
      }

      println!();
    }

    Ok(())
  }

  fn convert_into_type(&self, pixel_type: ImagePixelType) {

  }

  pub fn save_as_image(&mut self, path: &str) -> Result<(), Box<dyn error::Error>> {
    let mut image = self.generate_image()?;

    // image = match self.image_config.pixel_type {
    //   ImagePixelType::Dot => self.convert_into_type(ImagePixelType::Dot),
    //   ImagePixelType::Square => image
    // };

    image.save(path)?;

    Ok(())
  }

}

impl<'a> Default for QrWatermark<'a> {
  fn default() -> Self {
    let mut logo_buf_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    logo_buf_path.push("imgs/rust_logo.png");

    Self {
      text: "Hello this is QrWatermark",
      logo_path: Some(logo_buf_path),
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfigBuilder::new().build(),
      logo_config: LogoConfigBuilder::new().build()
    }
  }
}
