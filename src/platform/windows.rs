use crate::ClipboardProvider;

use clipboard_win::{get_clipboard_string, set_clipboard_string};
use raw_window_handle::HasRawDisplayHandle;

use std::error::Error;

pub fn connect<W: HasRawDisplayHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    Ok(Box::new(Clipboard))
}

pub struct Clipboard;

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(get_clipboard_string()?)
    }

    fn read_mime(&self, _mime: mime::Mime) -> Result<String, Box<dyn Error>> {
        Err("read_mime not implemented".into())
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        Ok(set_clipboard_string(&contents)?)
    }

    fn write_mime(
        &mut self,
        _mime: mime::Mime,
        _contents: String,
    ) -> Result<(), Box<dyn Error>> {
        Err("write_mime not implemented".into())
    }
}
