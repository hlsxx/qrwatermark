use qrwatermark::QrWatermark;

fn main() {
  QrWatermark::default()
    .save_as_png("./imgs/example1.png")
    .expect("Unable to save image");
}
