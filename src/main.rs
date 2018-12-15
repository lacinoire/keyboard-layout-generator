#![feature(custom_attribute)]

use failure::{bail, Error};
use serde_derive::Deserialize;
use std::fs::File;
use std::fmt;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use t4rust_derive::Template;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
struct Key {
    id: usize,
    normal: String,
    shift: String,
    alt: String,
    alt_shift: String,
    math: String,
    math_shift: String,
    math_alt: String,
    math_alt_shift: String,
}

enum ModifierName {
    shift, alt, math
}

struct KeyboardLayout {
    keys: Vec<Key>,
}

#[derive(Template)]
#[TemplatePath = "ios.tt"]
struct IosKeyboardLayout {
    keys: Vec<Key>,
}

#[derive(Debug, PartialEq, Eq)]
enum Platform {
    Ios,
    Linux,
    MacOs
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Platform::Ios => "ios",
            Platform::Linux => "linux",
            Platform::MacOs => "macos",
        };
        write!(f, "{}", printable)
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Platform> {
        match s {
            "ios" => Ok(Platform::Ios),
            "linux" => Ok(Platform::Linux),
            "macos" => Ok(Platform::MacOs),
            _ => bail!("{} could not be parsed as platform", s),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "keyboard-layout-generator-input")]
struct InputArgs {
    /// Input file with keylayout definition
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    input_file: PathBuf,

    /// Platform
    #[structopt(short = "p", long = "platform")]
    platform: Platform,
}

fn parse_keylayout_csv(path: PathBuf) -> Result<KeyboardLayout> {
    let reader = csv::Reader::from_path(path);
    let mut layout = KeyboardLayout { keys: Vec::new() };
    for result in reader?.deserialize() {
        let key: Key = result?;
        println!("{:?}", key);
        layout.keys.push(key);
    }
    Ok(layout)
}

fn map_keys_to_ios(keys: Vec<Key>) -> Vec<Key> {
    let mut ios_keys = Vec::new();
    for key in keys {
        let ios_key = Key { id: key.id,
        normal: map_character_to_ios(&key.normal),
        shift: map_character_to_ios(&key.shift),
        alt: map_character_to_ios(&key.alt),
        alt_shift: map_character_to_ios(&key.alt_shift),
        math: map_character_to_ios(&key.math),
        math_shift: map_character_to_ios(&key.math_shift),
        math_alt: map_character_to_ios(&key.math_alt),
        math_alt_shift: map_character_to_ios(&key.math_alt_shift) };
        ios_keys.push(ios_key);
    }
    ios_keys
}

fn map_character_to_ios(original_char: &str) -> String {
    let ios_char = match original_char {
        "|" => "&stick",
        "," => "&comma",
        ":" => "&twodots",
        "{" => "&lclosure",
        "}" => "&rclosure",
        "[" => "&lbracket",
        "]" => "&rbracket",
        "&" => "&symbol",
        x => x,
    };
    ios_char.into()
}

fn main() -> Result<()> {
    let input = InputArgs::from_args();
    println!("inputFile: {:?}, platform: {:?}", input.input_file, input.platform);
    let layout = parse_keylayout_csv(input.input_file)?;

    if input.platform == Platform::Ios {
        let ios_layout = IosKeyboardLayout { keys: map_keys_to_ios(layout.keys) };
        let output = format!("{}", ios_layout);
        println!("{}", output);
        let mut file = File::create("ioslayout.txt")?;
        write!(file, "{}", output)?;
    }

    Ok(())
}

