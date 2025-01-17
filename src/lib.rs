#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten",
        target_os = "redox"
    ))
))]
#[path = "platform/linux.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod platform;

#[cfg(not(any(
    all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "android",
            target_os = "emscripten",
            target_os = "redox"
        ))
    ),
    target_os = "windows",
    target_os = "macos",
)))]
#[path = "platform/dummy.rs"]
mod platform;

use raw_window_handle::HasRawDisplayHandle;
use std::error::Error;

pub use mime;

pub struct Clipboard {
    raw: Box<dyn ClipboardProvider>,
}

impl Clipboard {
    pub fn connect<W: HasRawDisplayHandle>(
        window: &W,
    ) -> Result<Self, Box<dyn Error>> {
        let raw = platform::connect(window)?;

        Ok(Clipboard { raw })
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        self.raw.read()
    }

    pub fn read_mime(
        &self,
        mime: mime::Mime,
    ) -> Result<String, Box<dyn Error>> {
        self.raw.read_mime(mime)
    }

    pub fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.raw.write(contents)
    }

    pub fn write_mime(
        &mut self,
        mime: mime::Mime,
        contents: String,
    ) -> Result<(), Box<dyn Error>> {
        self.raw.write_mime(mime, contents)
    }
}

pub trait ClipboardProvider {
    fn read(&self) -> Result<String, Box<dyn Error>>;

    fn read_mime(&self, mime: mime::Mime) -> Result<String, Box<dyn Error>>;

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>>;

    fn write_mime(
        &mut self,
        mime: mime::Mime,
        contents: String,
    ) -> Result<(), Box<dyn Error>>;
}
