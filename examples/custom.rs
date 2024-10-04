use qrwatermark::QrWatermark;
use qrwatermark::configs::image_config::ImageConfigBuilder;

fn main() {
  let image_config = ImageConfigBuilder::new()
    .build();

  let mut qrw = QrWatermark::new("Hello", "tss_id.jpg")
    .image_config(image_config);

  qrw.save_as_png("test.png").expect("Error while saving image");
}
