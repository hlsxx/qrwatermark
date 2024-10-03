use qrwatermark::QrWatermark;

fn main() {
  let mut qrw = QrWatermark::default();
  qrw.save_as_png("test.png").expect("Error while saving image");

  // let mut qrw = QrWatermark::new("Hello");
  // // qrw.print_into_console();
  // qrw.save_as_png("test.png").expect("Error while saving image");
}
