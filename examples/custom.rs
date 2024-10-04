use qrwatermark::{ImageConfigBuilder, QrWatermark};

fn main() {
  let image_config_builder = ImageConfigBuilder::new()
    .build();

  let mut qrw = QrWatermark::new("Hello", "tss_id.jpg");

  qrw.save_as_png("test.png").expect("Error while saving image");
}
