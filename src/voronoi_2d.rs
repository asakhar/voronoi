use crate::canvas::Canvas;
use serde::Deserialize;
use serde::Serialize;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Seed {
    x: u32,
    y: u32,
    c: u32,
}

impl Seed {
    pub fn dist_sq(self, x: u32, y: u32) -> u64 {
        ((self.x as i64 - x as i64).pow(2) + (self.y as i64 - y as i64).pow(2)) as u64
    }
    pub fn color(self) -> u32 {
        self.c | 0xFF000000
    }
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    seeds: &str,
) -> Result<(), JsValue> {
    let seeds: Vec<Seed> = serde_json::from_str(seeds).map_err(|e| e.to_string())?;
    let mut data = generate(width, height, &seeds);

    ctx.put_image_data(&data.image()?, 0., 0.)
}

fn generate(w: u32, h: u32, seeds: &[Seed]) -> Canvas {
    let mut canvas = Canvas::new(w, h);
    let dims = (w * h) as usize;
    let mut dists = Vec::new();
    dists.resize(dims, u64::MAX);
    seeds.iter().for_each(|seed| {
        let color = seed.color();
        for y in 0..h {
            let y_offset = y * w;
            for x in 0..w {
                let dist_sq = seed.dist_sq(x, y);
                let offset = (y_offset + x) as usize;
                let dist_mr = &mut dists[offset];

                if dist_sq < *dist_mr {
                    *dist_mr = dist_sq;
                    unsafe { canvas.put_pixel_unchecked(offset, color) };
                }
            }
        }
    });

    for seed in seeds {
        canvas.draw_circle(seed.x, seed.y, 5, 0xFF000000);
    }

    canvas
}
