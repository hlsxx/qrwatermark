use image::{ImageBuffer, Luma};
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

struct QrImageConfig {
  pixel_size: u8
}

impl Default for QrImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 10
    }
  }
}

pub struct QrWatermarkConfig {
  qr_code_config: QrCodeConfig,
  qr_image_config: QrImageConfig
}

pub struct QrWatermark {
  qr_code: QrCode,
  qr_code_image: Option<ImageBuffer<Luma<u8>, Vec<u8>>>,
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
    let module_size_in_pixel = 20;
    let image_size = self.qr_code.size() * module_size_in_pixel;

    self.qr_code_image = Some(ImageBuffer::from_fn(image_size as u32, image_size as u32, |x, y| {
      let module_x = (x / module_size_in_pixel as u32) as i32;
      let module_y = (y / module_size_in_pixel as u32) as i32;

      if self.qr_code.get_module(module_x, module_y) {
        Luma([0u8])
      } else {
        Luma([255u8])
      }
    }));
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
      qr_image_config: QrImageConfig::default()
    };

    Self {
      qr_code,
      qr_code_image: None,
      config
    }
  }
}
