use crate::render_to_string;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::{prelude::*, JsValue};

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
/// Read <https://katex.org/docs/options.html> for more information.
pub struct KaTeXOptions {
    /// Whether to render the math in the display mode.
    displayMode: bool,
    /// KaTeX output type.
    output: String,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: bool,
    /// Whether to make display math flush left.
    fleqn: bool,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throwOnError: bool,
    /// Color used for invalid LaTeX.
    errorColor: String,
    /// Collection of custom macros.
    macros: HashMap<String, String>,
    /// Specifies a minimum thickness, in ems.
    minRuleThickness: f64,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    maxSize: f64,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    maxExpand: i32,
    /// strict mode
    strict: String,
    /// Whether to trust users' input.
    /// Cannot be assigned at the same time with [`OptsBuilder::trust_callback`].
    trust: bool,
    /// group
    globalGroup: bool,
}

impl Default for KaTeXOptions {
    fn default() -> Self {
        Self {
            displayMode: false,
            output: "html".to_string(),
            leqno: false,
            fleqn: false,
            throwOnError: false,
            errorColor: "#cc0000".to_string(),
            macros: Default::default(),
            minRuleThickness: 0.04,
            maxSize: f64::INFINITY,
            maxExpand: 1000,
            strict: "warn".to_string(),
            trust: false,
            globalGroup: false,
        }
    }
}

impl KaTeXOptions {
    pub fn display_mode() -> KaTeXOptions {
        let mut o = KaTeXOptions::default();
        o.displayMode = true;
        return o;
    }
    pub fn inline_mode() -> KaTeXOptions {
        let mut o = KaTeXOptions::default();
        o.displayMode = false;
        return o;
    }
    pub fn render(&self, input: &str) -> String {
        render_to_string(input, &JsValue::from_serde(self).unwrap())
    }
}

impl KaTeXOptions {
    pub fn set_output_format(&mut self, format: &OutputType) -> bool {
        let set = match format {
            OutputType::Html => "html",
            OutputType::Mathml => "mathml",
            OutputType::HtmlAndMathml => "htmlAndMathml",
        };
        self.output = String::from(set);
        return true;
    }
    pub fn set_max_size(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_max_expand(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_strict_mode(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_error_color(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_macro_rules(&mut self) -> bool {
        unimplemented!()
    }
    pub fn insert_macro_rule(&mut self) -> bool {
        unimplemented!()
    }
}
