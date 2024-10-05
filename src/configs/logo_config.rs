use crate::traits::builder::Builder;

pub struct LogoConfig {
  pub width: u32,
  pub height: u32,
}

impl Default for LogoConfig {
  fn default() -> Self {
    Self {
      width: 50,
      height: 50
    }
  }
}

pub struct LogoConfigBuilder {
  width: Option<u32>,
  height: Option<u32>
}

impl Builder<LogoConfig> for LogoConfigBuilder {
  fn new() -> Self {
    Self {
      width: None,
      height: None
    }
  }

  fn build(self) -> LogoConfig {
    let logo_config_default = LogoConfig::default();

    LogoConfig {
      width: self.width.unwrap_or(logo_config_default.width),
      height: self.height.unwrap_or(logo_config_default.height)
    }
  }
}

impl LogoConfigBuilder {
  pub fn width(mut self, width: u32) -> Self {
    self.width = Some(width);
    self
  }

  pub fn height(mut self, height: u32) -> Self {
    self.height = Some(height);
    self
  }
}
