use qrwatermark::QrWatermark;

fn main() {
  QrWatermark::new("QrWatermark without logo")
    .save_as_image("./imgs/example3.png")
    .expect("Unable to save image");
}
