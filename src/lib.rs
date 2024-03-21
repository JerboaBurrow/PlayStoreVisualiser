use semver::{BuildMetadata, Prerelease, Version};

pub mod util;

const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

pub struct RuntimeOptions
{
    pub debug: bool,
    pub debug_timestamp: bool
}

pub static mut OPTIONS: RuntimeOptions = RuntimeOptions { debug: false, debug_timestamp: false };

pub fn debug(msg: String, context: Option<String>)
{
    unsafe { if OPTIONS.debug == false { return } }

    let mut message = String::new();

    let time = chrono::offset::Utc::now().to_rfc3339();



    let tag = match context
    {
        Some(s) => format!("[{s}] "),
        None => format!("[DEBUG] ")
    };

    for line in msg.split("\n")
    {
        unsafe { if OPTIONS.debug_timestamp { message.push_str(&format!("{time} ")); } }
        message.push_str(&tag);
        message.push_str(line);
        message.push_str("\n");
    }

    print!("{message}");
}

pub fn program_version() -> Version 
{
    Version
    {
        major: MAJOR.parse().unwrap(),
        minor: MINOR.parse().unwrap(),
        patch: PATCH.parse().unwrap(),
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY
    }
}