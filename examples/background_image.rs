use std::path::PathBuf;

use qrwatermark::{
  configs::{
    image_config::{ImageConfigBuilder, ImagePixelType}, logo_config::LogoConfigBuilder
  },
  traits::builder::Builder,
  QrWatermark,
};

fn main() {
  // Custom image config
  let image_config = ImageConfigBuilder::new()
    .pixel_size(15)
    .has_random_pixel_color()
    .background_image("./imgs/background.jpg")
    .build();

  let mut qrw = QrWatermark::new("QrWatermark example")
    .logo("imgs/rust_logo.png")
    .image_config(image_config);

  qrw.save_as_image("./imgs/example3.png").expect("Unable to save image");
}
