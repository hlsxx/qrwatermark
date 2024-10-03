use qrwatermark::QRWatermark;

fn main() {
  let mut qrw = QRWatermark::new("Hello");
  // qrw.print_into_console();
  qrw.save_as_png("test.png").expect("Error while saving image");
}
