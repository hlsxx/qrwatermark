use std::path::{Path, PathBuf};

use crate::traits::builder::Builder;

/// Image pixel shape type
#[derive(PartialEq)]
pub enum ImagePixelType {
  Square,
  Dot
}

pub struct ImageConfig {
  // Size of the pixel
  pub pixel_size: u32,

  // Type of the pixel
  pub pixel_type: ImagePixelType,

  // Margin of the image
  pub margin_size: u32,

  // Pixel color
  pub color: [u8; 3],

  // Pixel gradient color
  pub color_gradient: Option<([u8; 3], [u8; 3])>,

  // Background pixel color
  pub background_color: [u8; 3],

  // Background image, exclude the background pixel
  pub background_image_path: Option<PathBuf>,

  // Auto gradient creation
  pub is_auto_gradient_enabled: bool,

  // Random color for the specific pixel
  pub has_random_pixel_color: bool
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      pixel_size: 10,
      pixel_type: ImagePixelType::Square,
      margin_size: 1,
      color: [0, 0, 0],
      color_gradient: None,
      background_color: [255, 255, 255],
      background_image_path: None,
      is_auto_gradient_enabled: false,
      has_random_pixel_color: false
    }
  }
}

pub struct ImageConfigBuilder {
  pixel_size: Option<u32>,
  pixel_type: Option<ImagePixelType>,
  margin_size: Option<u32>,
  color: Option<[u8; 3]>,
  color_gradient: Option<([u8; 3], [u8; 3])>,
  background_color: Option<[u8; 3]>,
  background_image_path: Option<PathBuf>,
  is_auto_gradient_enabled: bool,
  has_random_pixel_color: bool
}

impl Builder<ImageConfig> for ImageConfigBuilder {
  fn new() -> Self {
    Self {
      pixel_size: None,
      pixel_type: None,
      margin_size: None,
      color: None,
      color_gradient: None,
      background_color: None,
      background_image_path: None,
      is_auto_gradient_enabled: false,
      has_random_pixel_color: false
    }
  }

  fn build(self) -> ImageConfig {
    let image_config_default = ImageConfig::default();

    ImageConfig {
      pixel_size: self.pixel_size.unwrap_or(image_config_default.pixel_size),
      pixel_type: self.pixel_type.unwrap_or(image_config_default.pixel_type),
      margin_size: self.margin_size.unwrap_or(image_config_default.margin_size),
      color: self.color.unwrap_or(image_config_default.color),
      color_gradient: self.color_gradient,
      background_color: self.background_color.unwrap_or(image_config_default.background_color),
      background_image_path: self.background_image_path,
      is_auto_gradient_enabled: self.is_auto_gradient_enabled,
      has_random_pixel_color: self.has_random_pixel_color
    }
  }
}

impl ImageConfigBuilder {
  /// Sets the default pixel size
  pub fn pixel_size(mut self, size: u32) -> Self {
    self.pixel_size = Some(size);
    self
  }

  /// Sets the default pixel shape type
  pub fn pixel_type(mut self, pixel_type: ImagePixelType) -> Self {
    self.pixel_type = Some(pixel_type);
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

  /// Sets the background image
  /// This option exclude background_color
  pub fn background_image(mut self, path: impl Into<PathBuf>) -> Self {
    self.background_image_path = Some(path.into());
    self
  }

  /// Sets auto generated gradient color
  pub fn is_auto_gradient_enabled(mut self) -> Self {
    self.is_auto_gradient_enabled = true;
    return self
  }

  /// Sets auto random pixel color generation
  pub fn has_random_pixel_color(mut self) -> Self {
    self.has_random_pixel_color = true;
    return self
  }

}
