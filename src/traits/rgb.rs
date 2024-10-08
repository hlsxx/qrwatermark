use image::Rgb;

pub trait ToRgb {
  fn to_rgb(&self) -> Result<Rgb<u8>, &'static str>;
}
