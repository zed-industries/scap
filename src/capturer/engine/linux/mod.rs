use std::{env, sync::mpsc};

use anyhow::{anyhow, Result};
use wayland::WaylandCapturer;
use x11::X11Capturer;

use crate::{capturer::Options, frame::Frame};

mod error;

mod wayland;
mod x11;

pub trait LinuxCapturerImpl {
    fn start_capture(&mut self);
    fn stop_capture(&mut self);
}

pub struct LinuxCapturer {
    pub imp: Box<dyn LinuxCapturerImpl>,
}

type Type = mpsc::Sender<Result<Frame>>;

impl LinuxCapturer {
    pub fn new(options: &Options, tx: Type) -> Result<Self> {
        if env::var("WAYLAND_DISPLAY").is_ok() {
            log::debug!("Creating new Wayland screen capturer.");
            Ok(Self {
                imp: Box::new(WaylandCapturer::new(options, tx)?),
            })
        } else if env::var("DISPLAY").is_ok() {
            log::debug!("Creating new X11 screen capturer.");
            Ok(Self {
                imp: Box::new(X11Capturer::new(options, tx)?),
            })
        } else {
            Err(anyhow!(
                "Unsupported platform. Could not detect Wayland or X11 displays"
            ))
        }
    }
}

pub fn create_capturer(
    options: &Options,
    tx: mpsc::Sender<Result<Frame>>,
) -> Result<LinuxCapturer> {
    LinuxCapturer::new(options, tx)
}
