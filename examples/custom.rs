use qrwatermark::QrWatermark;
use qrwatermark::configs::image_config::ImageConfigBuilder;
use qrwatermark::traits::builder::Builder;

fn main() {
  let image_config = ImageConfigBuilder::new()
    .build();

  let mut qrw = QrWatermark::new("e0d32992d96e5eda910fe675e20bbce0")
    .logo("imgs/rust_logo.png")
    .image_config(image_config);

  qrw.save_as_png("test.png").expect("Error while saving image");
}
