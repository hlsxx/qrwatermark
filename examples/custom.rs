use qrwatermark::configs::logo_config::LogoConfigBuilder;
use qrwatermark::QrWatermark;
use qrwatermark::configs::image_config::ImageConfigBuilder;
use qrwatermark::traits::builder::Builder;

fn main() {
  // Custom image config
  let image_config = ImageConfigBuilder::new()
    .color_gradient(([206, 66, 43], [23, 23, 23])) // Custom gradient colors
    .color([112, 81, 24])
    // .is_auto_gradient_enabled() // Auto gradient
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
