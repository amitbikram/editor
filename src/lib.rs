use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
struct Image {
    width: usize,
    height: usize,
    cells: Vec<Rgb>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();
        cells.resize(height * width, Rgb{
            r: 200,
            g: 200,
            b: 255,
        });
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize{
        self.width
    }

    pub fn height(&self) -> usize{
        self.height
    }

    pub fn cells(&self) -> Vec<u8>{
        self.cells.iter()
            .map(|&rgb| vec![rgb.r, rgb.g, rgb.b])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
}
