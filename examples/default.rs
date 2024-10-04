use qrwatermark::QrWatermark;

fn main() {
  let mut qrw = QrWatermark::default();
  qrw.save_as_png("test.png").expect("Error while saving image");
}
