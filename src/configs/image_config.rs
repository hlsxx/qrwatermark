use crate::traits::builder::Builder;

pub struct ImageConfig {
  pub pixel_size: u32,
  pub margin_size: u32,
  pub color: [u8; 3],
  pub background_color: [u8; 3],
  pub is_gradient_enabled: bool
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 10,
      margin_size: 1,
      color: [0, 0, 0],
      background_color: [255, 255, 255],
      is_gradient_enabled: false
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  margin_size: Option<u32>,
  color: Option<[u8; 3]>,
  background_color: Option<[u8; 3]>,
  is_gradient_enabled: bool
}

impl Builder<ImageConfig> for ImageConfigBuilder {
  fn new() -> Self {
    Self {
      pixel_size: None,
      margin_size: None,
      color: None,
      background_color: None,
      is_gradient_enabled: false
    }
  }

  fn build(self) -> ImageConfig {
    let image_config_default = ImageConfig::default();

    ImageConfig {
      pixel_size: self.pixel_size.unwrap_or(image_config_default.pixel_size),
      margin_size: self.margin_size.unwrap_or(image_config_default.margin_size),
      color: self.color.unwrap_or(image_config_default.color),
      background_color: self.background_color.unwrap_or(image_config_default.background_color),
      is_gradient_enabled: self.is_gradient_enabled
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

  pub fn color(mut self, rgb: [u8; 3]) -> Self {
    self.color = Some(rgb);
    self
  }

  pub fn background_color(mut self, rgb: [u8; 3]) -> Self {
    self.background_color = Some(rgb);
    self
  }

  pub fn is_gradient_enabled(mut self) -> Self {
    self.is_gradient_enabled = true;
    return self
  }

}
