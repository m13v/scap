use std::sync::mpsc;

use super::Options;
use crate::frame::Frame;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "linux")]
mod linux;

pub struct Engine {
    options: Options,
    #[cfg(target_os = "macos")]
    mac: screencapturekit::sc_stream::SCStream,

    #[cfg(target_os = "windows")]
    win: win::WCStream,

    #[cfg(target_os = "linux")]
    linux: linux::LinuxCapturer,
}

impl Engine {
    pub fn new(options: &Options, tx: mpsc::Sender<Frame>) -> Result<Engine, String> {
        #[cfg(target_os = "macos")]
        {
            let mac = mac::create_capturer(&options, tx)?;
            return Ok(Engine {
                mac,
                options: (*options).clone(),
            });
        }

        #[cfg(target_os = "windows")]
        {
            let win = win::create_capturer(&options, tx);
            return Ok(Engine {
                win,
                options: (*options).clone(),
            });
        }

        #[cfg(target_os = "linux")]
        {
            let linux = linux::create_capturer(&options, tx);
            return Ok(Engine {
                linux,
                options: (*options).clone(),
            });
        }
    }

    pub fn start(&mut self) {
        #[cfg(target_os = "macos")]
        {
            match self.mac.start_capture() {
                Ok(_) => println!("Capture started successfully"),
                Err(e) => {
                    eprintln!("Failed to start capture: {:?}", e);
                    if let Some(error_code) = e.to_string().split(':').next() {
                        eprintln!("Error code: {}", error_code);
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            self.win.start_capture();
        }

        #[cfg(target_os = "linux")]
        {
            self.linux.start_capture();
        }
    }

    pub fn stop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.mac.stop_capture().expect("Failed to stop capture");
        }

        #[cfg(target_os = "windows")]
        {
            self.win.stop_capture();
        }

        #[cfg(target_os = "linux")]
        {
            self.linux.stop_capture();
        }
    }

    pub fn get_output_frame_size(&mut self) -> [u32; 2] {
        #[cfg(target_os = "macos")]
        {
            mac::get_output_frame_size(&self.options)
        }

        #[cfg(target_os = "windows")]
        {
            win::get_output_frame_size(&self.options)
        }

        #[cfg(target_os = "linux")]
        {
            return [0, 0];
        }
    }
}