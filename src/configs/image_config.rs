use crate::traits::builder::Builder;

pub struct ImageConfig {
  pub pixel_size: u32,
  pub margin_size: u32,
  pub color: [u8; 3],
  pub color_gradient: Option<([u8; 3], [u8; 3])>,
  pub background_color: [u8; 3],
  pub is_gradient_enabled: bool
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 10,
      margin_size: 1,
      color: [0, 0, 0],
      color_gradient: None,
      background_color: [255, 255, 255],
      is_gradient_enabled: false
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  margin_size: Option<u32>,
  color: Option<[u8; 3]>,
  color_gradient: Option<([u8; 3], [u8; 3])>,
  background_color: Option<[u8; 3]>,
  is_gradient_enabled: bool
}

impl Builder<ImageConfig> for ImageConfigBuilder {
  fn new() -> Self {
    Self {
      pixel_size: None,
      margin_size: None,
      color: None,
      color_gradient: None,
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
      color_gradient: self.color_gradient,
      background_color: self.background_color.unwrap_or(image_config_default.background_color),
      is_gradient_enabled: self.is_gradient_enabled
    }
  }
}

impl ImageConfigBuilder {
  /// Sets the default pixel size
  pub fn pixel_size(mut self, size: u32) -> Self {
    self.pixel_size = Some(size);
    self
  }

  /// Sets the frame margin
  pub fn margin_size(mut self, size: u32) -> Self {
    self.margin_size = Some(size);
    self
  }

  /// Sets the color of the pixel using an RGB value
  ///
  /// # Arguments
  /// * `rgb` - A 3-element array representing the RGB value of the pixel (e.g., `[200, 0, 0]` for red).
  ///
  /// # Example
  /// ```
  /// let image_config = ImageConfigBuilder::new()
  /// image_config.color([200, 0, 0]); // Sets the pixel to red
  /// ```
  pub fn color(mut self, rgb: [u8; 3]) -> Self {
    self.color = Some(rgb);
    self
  }

  /// Sets the color of the pixel calculated from the provided RGB values
  ///
  /// # Arguments
  /// * `rgbs` - A tuple containing two 3-element arrays representing the RGB values for the
  /// gradient start and gradient end
  /// ///
  /// # Example
  /// ```
  /// let image_config = ImageConfigBuilder::new();
  /// image_config.color_gradient(([200, 0, 0], [0, 0, 0])); // Sets the gradient from red to black
  /// ```
  pub fn color_gradient(mut self, rgbs: ([u8; 3], [u8; 3])) -> Self {
    self.color_gradient = Some(rgbs);
    self
  }

  /// Sets the color of the oposite (background) pixel using an RGB value
  ///
  /// # Arguments
  /// * `rgb` - A 3-element array representing the RGB value of the pixel (e.g., `[255, 255, 255]` for white).
  ///
  /// # Example
  /// ```
  /// let image_config = ImageConfigBuilder::new()
  /// image_config.background_color([255, 255, 255]); // Sets the pixel to white
  /// ```
  pub fn background_color(mut self, rgb: [u8; 3]) -> Self {
    self.background_color = Some(rgb);
    self
  }

  /// Sets auto generated gradient color
  pub fn is_gradient_enabled(mut self) -> Self {
    self.is_gradient_enabled = true;
    return self
  }

}
