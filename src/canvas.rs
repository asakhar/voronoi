use wasm_bindgen::{Clamped, JsValue};
use web_sys::ImageData;
pub struct Canvas {
    width: u32,
    height: u32,
    data: Vec<u32>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let mut data = Vec::new();
        data.resize(width as usize * height as usize, 0);
        Self {
            width,
            height,
            data,
        }
    }
    pub fn image(&mut self) -> Result<ImageData, JsValue> {
        let slice = &self.data[..];
        let u8_slice = unsafe { slice.align_to::<u8>() }.1;
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(u8_slice), self.width, self.height)
    }
    #[inline(always)]
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe {
            self.put_pixel_unchecked(y as usize * self.width as usize + x as usize, color)
        })
    }
    #[inline(always)]
    pub unsafe fn put_pixel_unchecked(&mut self, offset: usize, color: u32) {
        *self.data.get_unchecked_mut(offset) = color;
    }
    pub fn draw_circle(&mut self, x: u32, y: u32, r: u32, color: u32) {
        use std::cmp::{max, min};
        let stride = self.width as i64;
        let dx = x as i64;
        let dy = y as i64;
        let r = r as i64;
        let r_sq = r * r;
        let x_low = max(-dx, -r);
        let x_high = min(dx + r, stride) - dx;
        let y_low = max(-dy, -r);
        let y_high = min(dy + r, self.height as i64);
        for y in y_low..=y_high {
            let y_sq = y * y;
            let y_offset = (y + dy) * stride;
            for x in x_low..=x_high {
                if y_sq + x * x > r_sq {
                    continue;
                }
                let offset = (y_offset + x + dx) as usize;
                unsafe { self.put_pixel_unchecked(offset, color) };
            }
        }
    }
}
