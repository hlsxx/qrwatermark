use qrwatermark::{
  configs::{
    logo_config::LogoConfigBuilder,
    image_config::{self, ImageConfigBuilder}
  },
  traits::builder::Builder,
  QrWatermark,
};

fn main() {
  // Custom image config
  let image_config = ImageConfigBuilder::new()
    .color([112, 81, 24])
    .pixel_size(15)
    .pixel_type(image_config::ImagePixelType::Dot)
    .build();

  // Custom logo config
  let logo_config =  LogoConfigBuilder::new()
    .width(70)
    .height(70)
    .build();

  let mut qrw = QrWatermark::new("QrWatermark example")
    .logo("imgs/rust_logo.png")
    .logo_config(logo_config)
    .image_config(image_config);

  qrw.save_as_image("./imgs/example2.png").expect("Unable to save image");
}
