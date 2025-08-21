use crate::config::*;
use ansi_term::Color;
use reqwest::blocking::get;
use serde_json::Value;
use std::{
    env::home_dir,
    error::Error,
    ffi::OsStr,
    fmt::{self},
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
    process,
};

enum CustomValue {
    F32(Option<f32>),
    U8(Option<u8>),
}

impl fmt::Display for CustomValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomValue::F32(Some(val)) => write!(f, "{}", val),
            CustomValue::F32(None) => write!(f, ""),
            CustomValue::U8(Some(val)) => write!(f, "{}", val),
            CustomValue::U8(None) => write!(f, ""),
        }
    }
}

pub fn get_url(config: &Config) -> String {
    let base = "https://cataas.com/cat";
    let mut params: Vec<String> = Vec::new();
    let mut path: Vec<String> = Vec::new();

    if let Some(tags) = &config.tags {
        match tags.as_slice() {
            [] => {}
            [only] => path.push(format!("/{}", only)),
            many => path.push(format!("/{}", many.join(","))),
        }
    }

    if let Some(says) = &config.says {
        path.push(format!("says/{}", says.sentence));
        if let Some(size) = says.size {
            params.push(format!("fontSize={size}"));
        }
        if let Some(color) = &says.color {
            params.push(format!("fontColor={color}"));
        }
    }

    if let Some(filter) = &config.filter {
        if let Some(kind) = &filter.kind {
            params.push(format!("filter={}", kind));

            if kind.is_custom() {
                for (key, value) in [
                    ("brightness", filter.brightness),
                    ("lightness", filter.lightness),
                    ("saturation", filter.saturation),
                    ("hue", filter.hue),
                ] {
                    custom_filter_params(&mut params, key, Some(CustomValue::F32(value)));
                }

                if let Some(rgb) = &filter.rgb {
                    for (key, value) in [("r", rgb.r), ("g", rgb.g), ("b", rgb.b)] {
                        custom_filter_params(&mut params, key, Some(CustomValue::U8(value)));
                    }
                }
            }
        }
    }

    if let Some(image_type) = &config.image_type {
        params.push(format!("type={}", image_type));
    }

    if let Some(dimensions) = &config.dimensions {
        params.push(format!(
            "height={}&width={}",
            dimensions.height, dimensions.width
        ));
    }

    let url = format!("{}{}?{}", base, path.join("/"), params.join("&"));
    url
}

pub fn get_config() -> PathBuf {
    if let Some(mut config_location) = home_dir() {
        config_location.push(".config/kitty-cat/config.toml");

        if config_location.exists() {
            config_location
        } else {
            println!("Config file not found!");
            println!("Run kitty-cat -c or create the file yourself.");
            process::exit(1)
        }
    } else {
        panic!("Cant get home_dir")
    }
}

pub fn get_local_data() -> String {
    if let Some(mut image_location) = home_dir() {
        image_location.push(".local/share/kitty-cat");

        if image_location.exists() {
            image_location.to_string_lossy().into_owned()
        } else {
            println!("Image folder not found!");
            println!("Run kitty-cat -c or create the folder yourself");
            process::exit(1)
        }
    } else {
        panic!("Cant get image location")
    }
}

pub fn get_cached_images(cache_location: String) -> Vec<String> {
    let mut images: Vec<String> = Vec::new();

    let supported_formats: [&OsStr; 3] = [OsStr::new("png"), OsStr::new("jpeg"), OsStr::new("gif")];

    if let Ok(files) = fs::read_dir(cache_location) {
        for file in files
            .flatten()
            .filter(|file| supported_formats.contains(&file.path().extension().unwrap()))
        {
            if let Some(name) = file.file_name().to_str() {
                images.push(name.to_string());
            }
        }
    };
    images
}

pub fn get_tags() -> Result<(), Box<dyn Error>> {
    let url: String = "https://cataas.com/api/tags".to_string();
    let response = get(&url)?;

    let file = File::create("tags.json")?;
    let mut writer = BufWriter::new(file);

    let data: Value = serde_json::from_str::<Value>(&response.text()?)?;
    serde_json::to_writer_pretty(&mut writer, &data)?;
    println!(
        "{}",
        Color::Green.bold().paint("Successfully retrived tags")
    );
    process::exit(0);
}

fn custom_filter_params(params: &mut Vec<String>, key: &str, value: Option<CustomValue>) {
    if let Some(val) = value {
        match val {
            CustomValue::F32(Some(v)) => params.push(format!("{}={}", key, v)),
            CustomValue::U8(Some(v)) => params.push(format!("{}={}", key, v)),
            _ => {}
        }
    }
}
