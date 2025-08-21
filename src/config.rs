use core::fmt;
use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

#[derive(Deserialize)]
pub struct Says {
    pub sentence: String,
    pub size: Option<u8>,
    pub color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Mono,
    Negate,
    Custom,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    Xsmall,
    Small,
    Medium,
    Square,
}

#[derive(Deserialize)]
pub struct Filter {
    pub kind: Option<Kind>,
    pub brightness: Option<f32>,
    pub lightness: Option<f32>,
    pub saturation: Option<f32>,
    pub hue: Option<f32>,
    pub rgb: Option<Rgb>,
}

#[derive(Deserialize)]
pub struct Rgb {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
}

#[derive(Deserialize)]
pub struct Config {
    pub offline: bool,
    pub cache: bool,
    pub image_type: Option<ImageType>,
    pub alignment: Option<Alignment>,
    pub tags: Option<Vec<String>>,
    pub says: Option<Says>,
    pub filter: Option<Filter>,
    pub dimensions: Option<Dimensions>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            offline: false,
            cache: false,
            alignment: Some(Alignment::Left),
            image_type: None,
            tags: None,
            says: None,
            filter: None,
            dimensions: None,
        }
    }
}

impl Config {
    pub fn alignment_to_string(&self) -> String {
        if let Some(alignment) = &self.alignment {
            match alignment {
                Alignment::Left => "left".to_string(),
                Alignment::Center => "center".to_string(),
                Alignment::Right => "right".to_string(),
            }
        } else {
            "left".to_string()
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Mono => write!(f, "mono"),
            Kind::Negate => write!(f, "negate"),
            Kind::Custom => write!(f, "custom"),
        }
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageType::Xsmall => write!(f, "xsmall"),
            ImageType::Small => write!(f, "small"),
            ImageType::Medium => write!(f, "medium"),
            ImageType::Square => write!(f, "square"),
        }
    }
}

impl Kind {
    pub fn is_custom(&self) -> bool {
        if self == &Kind::Custom { true } else { false }
    }
}
