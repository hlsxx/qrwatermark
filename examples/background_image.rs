use std::path::PathBuf;

use qrwatermark::{
  configs::{
    logo_config::LogoConfigBuilder,
    image_config::ImageConfigBuilder
  },
  traits::builder::Builder,
  QrWatermark,
};

fn main() {
  // Custom image config
  let image_config = ImageConfigBuilder::new()
    .color([112, 81, 24])
    .background_image("./imgs/background.jpg")
    .build();

  let mut qrw = QrWatermark::new("QrWatermark example")
    .logo("imgs/rust_logo.png")
    .image_config(image_config);

  qrw.save_as_image("./imgs/example3.png").expect("Unable to save image");
}
