use serde::{Deserialize, Serialize};

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
    Blur,
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

impl Kind {
    pub fn to_string(&self) -> String {
        match self {
            Kind::Blur => "blur".to_string(),
            Kind::Mono => "mono".to_string(),
            Kind::Negate => "negate".to_string(),
            Kind::Custom => "custom".to_string(),
        }
    }
    pub fn is_custom(&self) -> bool {
        if self == &Kind::Custom { true } else { false }
    }
}

impl ImageType {
    pub fn to_string(&self) -> String {
        match self {
            ImageType::Xsmall => "xsmall".to_string(),
            ImageType::Small => "small".to_string(),
            ImageType::Medium => "medium".to_string(),
            ImageType::Square => "square".to_string(),
        }
    }
}
