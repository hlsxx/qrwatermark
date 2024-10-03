use image::{ImageBuffer, Luma};
use qrcodegen::QrCode;

pub struct QRWatermark {
  qr_code: QrCode,
  qr_code_image: Option<ImageBuffer<Luma<u8>, Vec<u8>>>
}

impl QRWatermark {

  pub fn new(text: &str) -> Self {
    let qr_code = QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).unwrap();

    Self {
      qr_code,
      qr_code_image: None
    }
  }

  fn generate_image(&mut self) {
    let module_size_in_pixel = 10;
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
