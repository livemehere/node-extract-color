#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use screenshots::Screen;
use device_query::{DeviceQuery, DeviceState};

#[napi]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

#[napi]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}

#[napi]
pub fn get_mouse_position() -> MousePosition {
    let device_state = DeviceState::new();
    let mouse = device_state.get_mouse();

    MousePosition {
        x: mouse.coords.0,
        y: mouse.coords.1,
    }
}

#[napi]
pub fn get_pixel_color_at_cursor() -> Option<PixelColor> {
    let device_state = DeviceState::new();
    let mouse = device_state.get_mouse();
    let pos = mouse.coords;

    let screens = match Screen::all() {
        Ok(screens) => screens,
        Err(_) => return None,
    };

    for screen in screens {
        let display_x = pos.0 - screen.display_info.x;
        let display_y = pos.1 - screen.display_info.y;

        if display_x >= 0 && display_x < screen.display_info.width as i32 &&
            display_y >= 0 && display_y < screen.display_info.height as i32 {

            if let Ok(image) = screen.capture() {
                let pixel = image.get_pixel(display_x as u32, display_y as u32);
                let r = pixel.0[0];
                let g = pixel.0[1];
                let b = pixel.0[2];

                let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

                return Some(PixelColor { r, g, b, hex });
            }
        }
    }

    None
}

#[napi]
pub fn get_screens_info() -> Vec<ScreenInfo> {
    let screens = match Screen::all() {
        Ok(screens) => screens,
        Err(_) => return vec![],
    };

    screens.into_iter().map(|screen| {
        ScreenInfo {
            x: screen.display_info.x,
            y: screen.display_info.y,
            width: screen.display_info.width as i32,
            height: screen.display_info.height as i32,
        }
    }).collect()
}

#[napi]
pub struct ScreenInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
