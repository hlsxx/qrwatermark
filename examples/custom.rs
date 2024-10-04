use qrwatermark::{LogoConfigBuilder, QrWatermark};

fn main() {
  let logo_config = LogoConfigBuilder::new()
    .build();

  let mut qrw = QrWatermark::new("Hello", "tss_id.jpg")
    .logo_config(logo_config);

  qrw.save_as_png("test.png").expect("Error while saving image");
}
