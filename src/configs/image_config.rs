use image::Rgb;

pub struct ImageConfig {
  pub pixel_size: u32,
  pub width: u32,
  pub height: u32,
  pub rgb: Rgb<u8>
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 20,
      width: 100,
      height: 100,
      rgb: Rgb([0, 0, 0])
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  width: Option<u32>,
  height: Option<u32>,
  rgb: Option<Rgb<u8>>
}

impl ImageConfigBuilder {
  pub fn new() -> Self {
    Self {
      pixel_size: None,
      width: None,
      height: None,
      rgb: None
    }
  }

  pub fn pixel_size(mut self, size: u32) -> Self {
    self.pixel_size = Some(size);
    self
  }

  pub fn width(mut self, width: u32) -> Self {
    self.width = Some(width);
    self
  }

  pub fn height(mut self, height: u32) -> Self {
    self.height = Some(height);
    self
  }

  pub fn rgb(mut self, rgb: Rgb<u8>) -> Self {
    self.rgb = Some(rgb);
    self
  }

  pub fn build(self) -> ImageConfig {
    let image_config_default = ImageConfig::default();

    ImageConfig {
      pixel_size: self.pixel_size.unwrap_or(image_config_default.pixel_size),
      width: self.width.unwrap_or(image_config_default.width),
      height: self.height.unwrap_or(image_config_default.height),
      rgb: self.rgb.unwrap_or(image_config_default.rgb)
    }
  }

}
