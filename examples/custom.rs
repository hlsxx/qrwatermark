use qrwatermark::configs::logo_config::LogoConfigBuilder;
use qrwatermark::QrWatermark;
use qrwatermark::configs::image_config::ImageConfigBuilder;
use qrwatermark::traits::builder::Builder;

fn main() {
  // Custom image config
  let image_config = ImageConfigBuilder::new()
    .color(image::Rgb([14, 99, 88]))
    .build();

  // Custom logo config
  let logo_config =  LogoConfigBuilder::new()
    .width(70)
    .height(70)
    .build();

  let mut qrw = QrWatermark::new("Custom text")
    .logo("imgs/rust_logo.png")
    .logo_config(logo_config)
    .image_config(image_config);

  qrw.save_as_png("test.png").expect("Error while saving image");
}
