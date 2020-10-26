use wasm_bindgen::prelude::*;
mod options;
pub use options::{KaTeXOptions};

#[wasm_bindgen(module = "/src/katex.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = renderToString)]
    pub fn render_to_string(expr: &str, args: &JsValue) -> String;
}
