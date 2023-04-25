use crate::{DisplayInfo, Image};
use anyhow::{anyhow, Result};
use core_graphics::{
  display::{CGDisplay, CGRect},
  geometry::{CGPoint, CGSize},
};

fn capture(rect: CGRect, display: CGDisplay) -> Result<Image> {
  let cg_image = display
    .image_for_rect(rect)
    .ok_or_else(|| anyhow!("Screen:{} screenshot failed", display.id))?;

  let image = Image::from_bgra(
    Vec::from(cg_image.data().bytes()),
    cg_image.width() as u32,
    cg_image.height() as u32,
    cg_image.bytes_per_row(),
  )?;
  Ok(image)
}

pub fn capture_screen(display_info: &DisplayInfo) -> Result<Image> {
  let cg_display = CGDisplay::new(display_info.id);
  capture(cg_display.bounds(), cg_display)
}

pub fn capture_screen_area(
  display_info: &DisplayInfo,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
) -> Result<Image> {
  let cg_display = CGDisplay::new(display_info.id);
  let mut cg_rect = cg_display.bounds();
  let origin = cg_rect.origin;

  let rect_x = origin.x + (x as f64);
  let rect_y = origin.y + (y as f64);
  let rect_width = width as f64;
  let rect_height = height as f64;

  cg_rect.origin = CGPoint::new(rect_x, rect_y);
  cg_rect.size = CGSize::new(rect_width, rect_height);

  capture(cg_rect, cg_display)
}
