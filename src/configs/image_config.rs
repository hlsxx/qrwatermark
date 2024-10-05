use image::Rgb;

use crate::traits::builder::Builder;

pub struct ImageConfig {
  pub pixel_size: u32,
  pub margin_size: u32,
  pub rgb: Rgb<u8>,
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 10,
      margin_size: 1,
      rgb: Rgb([0, 0, 0]),
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  margin_size: Option<u32>,
  rgb: Option<Rgb<u8>>,
}

impl Builder<ImageConfig> for ImageConfigBuilder {
  fn new() -> Self {
    Self {
      pixel_size: None,
      margin_size: None,
      rgb: None,
    }
  }

  fn build(self) -> ImageConfig {
    let image_config_default = ImageConfig::default();

    ImageConfig {
      pixel_size: self.pixel_size.unwrap_or(image_config_default.pixel_size),
      rgb: self.rgb.unwrap_or(image_config_default.rgb),
      margin_size: self.margin_size.unwrap_or(image_config_default.margin_size)
    }
  }

}

impl ImageConfigBuilder {
  pub fn pixel_size(mut self, size: u32) -> Self {
    self.pixel_size = Some(size);
    self
  }

  pub fn margin_size(mut self, size: u32) -> Self {
    self.margin_size = Some(size);
    self
  }

  pub fn rgb(mut self, rgb: Rgb<u8>) -> Self {
    self.rgb = Some(rgb);
    self
  }

}
