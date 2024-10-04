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

struct ImageConfig<'a> {
  pixel_size: u32,
  logo_path: &'a str,
  width: u32,
  height: u32,
  rgb: Rgb<u8>
}

impl<'a> Default for ImageConfig<'a> {
  fn default() -> Self {
    Self {
      pixel_size: 20,
      logo_path: "imgs/rust_logo.png",
      width: 100,
      height: 100,
      rgb: Rgb([0, 0, 0])
    }
  }
}

pub struct QrWatermarkConfig<'a> {
  qr_code_config: QrCodeConfig,
  image_config: ImageConfig<'a>
}

pub struct QrWatermark<'a> {
  qr_code: QrCode,
  qr_code_image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
  config: QrWatermarkConfig<'a>
}

impl<'a> QrWatermark<'a> {

  pub fn new(text: &str, config: QrWatermarkConfig<'a>) -> Self {
    let qr_code = QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).unwrap();

    Self {
      qr_code,
      qr_code_image: None,
      config
    }
  }

  fn generate_image(&mut self) {
    let image_config = &self.config.image_config;
    let image_size = self.qr_code.size() as u32
      * self.config.image_config.pixel_size;

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

    let logo = image::open(image_config.logo_path).unwrap();
    let logo_thumbnail = logo.thumbnail(image_config.width, image_config.height);

    let qr_center_x = (image_size - logo_thumbnail.width()) / 2;
    let qr_center_y = (image_size - logo_thumbnail.height()) / 2;

    for x in 0..logo_thumbnail.width() {
      for y in 0..logo_thumbnail.height() {
        let pixel = logo_thumbnail.get_pixel(x, y).to_rgb();
        image.put_pixel(qr_center_x + x, qr_center_y + y, pixel);
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

impl<'a> Default for QrWatermark<'a> {
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
