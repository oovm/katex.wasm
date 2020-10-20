use wasm_bindgen::prelude::*;
mod options;
pub use options::{KaTeXOptions, OutputType};

#[wasm_bindgen(module = "/src/katex.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = renderToString)]
    pub fn render_to_string(expr: &str, args: &JsValue) -> String;
//#[wasm_bindgen(js_name = renderToDomTree)]
// pub fn render_to_dom(expr: &str, args: &JsValue) -> JsValue;
//#[wasm_bindgen(js_name = renderToHTMLTree)]
// pub fn render_to_html(expr: &str, args: &JsValue) -> JsValue;
}
