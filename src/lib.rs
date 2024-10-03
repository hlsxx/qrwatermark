use image::{ImageBuffer, Luma, Rgb, Rgba};
use imageproc::drawing::{draw_cross_mut, Canvas};
use qrcodegen::{QrCode, QrCodeEcc};

struct QrCodeConfig {
  ecc: QrCodeEcc,
}

impl Default for QrCodeConfig {
  fn default() -> Self {
    Self {
      ecc: QrCodeEcc::Medium
    }
  }
}

struct ImageConfig {
  pixel_size: u32
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 20
    }
  }
}

pub struct QrWatermarkConfig {
  qr_code_config: QrCodeConfig,
  image_config: ImageConfig
}

pub struct QrWatermark {
  qr_code: QrCode,
  qr_code_image: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
  config: QrWatermarkConfig
}

impl QrWatermark {

  pub fn new(text: &str, config: QrWatermarkConfig) -> Self {
    let qr_code = QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).unwrap();

    Self {
      qr_code,
      qr_code_image: None,
      config
    }
  }

  fn generate_image(&mut self) {
    let image_size = self.qr_code.size() as u32 * self.config.image_config.pixel_size;

    let mut image = ImageBuffer::from_fn(image_size , image_size, |x, y| {
      let module_x = (x / self.config.image_config.pixel_size) as i32;
      let module_y = (y / self.config.image_config.pixel_size) as i32;

      let (r, g, b) = (100, 100, 100);

      if self.qr_code.get_module(module_x, module_y) {
        Rgba([r, g, b, 100])
        // Luma([0u8])
      } else {
        Rgba([255, 255, 255, 100])
        // Luma([255u8])
      }
    });

    let logo = image::open("logo.jpeg").unwrap();
    let logo_thumbnail = logo.thumbnail(40, 40);

    for x in 0..logo_thumbnail.width() {
      for y in 0..logo_thumbnail.height() {
        let pixel = logo_thumbnail.get_pixel(x, y);
        image.put_pixel(x, y, pixel);
      }
    }

    // image.put_pixel(10, 10, pixel);

    self.qr_code_image = Some(image);
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
    self.generate_image();

    if let Some(qr_code_image) = &self.qr_code_image {
      qr_code_image.save(path)?;
    }

    Ok(())
  }

}

impl Default for QrWatermark {
  fn default() -> Self {
    let qr_code = QrCode::encode_text("Hello this is QrWatermark", qrcodegen::QrCodeEcc::Medium).unwrap();
    let config = QrWatermarkConfig {
      qr_code_config: QrCodeConfig::default(),
      image_config: ImageConfig::default()
    };

    Self {
      qr_code,
      qr_code_image: None,
      config
    }
  }
}
