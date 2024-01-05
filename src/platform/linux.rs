use crate::ClipboardProvider;

use raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle};
use std::error::Error;

pub use clipboard_wayland as wayland;
pub use clipboard_x11 as x11;

pub fn connect<W: HasRawDisplayHandle>(
    window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    let clipboard = match window.raw_display_handle() {
        RawDisplayHandle::Wayland(handle) => {
            assert!(!handle.display.is_null());

            Box::new(unsafe {
                wayland::Clipboard::connect(handle.display as *mut _)
            }) as _
        }
        _ => Box::new(x11::Clipboard::connect()?) as _,
    };

    Ok(clipboard)
}

impl ClipboardProvider for wayland::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn read_mime(&self, _mime: mime::Mime) -> Result<String, Box<dyn Error>> {
        Err("read_mime not implemented".into())
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents)
    }

    fn write_mime(
        &mut self,
        _mime: mime::Mime,
        _contents: String,
    ) -> Result<(), Box<dyn Error>> {
        Err("write_mime not implemented".into())
    }
}

impl ClipboardProvider for x11::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read().map_err(Box::from)
    }

    fn read_mime(&self, _mime: mime::Mime) -> Result<String, Box<dyn Error>> {
        Err("read_mime not implemented".into())
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents).map_err(Box::from)
    }

    fn write_mime(
        &mut self,
        _mime: mime::Mime,
        _contents: String,
    ) -> Result<(), Box<dyn Error>> {
        Err("write_mime not implemented".into())
    }
}
