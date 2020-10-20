pub use katex_wasmbind::{KaTeXOptions, OutputType};
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct KaTeXProperties {
    pub math: String,
    #[prop_or(true)]
    pub inline: bool,
    #[prop_or(OutputType::Html)]
    pub output: OutputType,
}

pub struct KaTeX {
    pub math_rendered: Html,
    pub props: KaTeXProperties,
}

impl Component for KaTeX {
    type Message = ();
    type Properties = KaTeXProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { math_rendered: Default::default(), props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => false,
            false => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let mut render = if self.props.inline {
            KaTeXOptions::inline_mode()
        }
        else {
            KaTeXOptions::display_mode()
        };
        render.set_output_format(&self.props.output);
        let t = yew::utils::document().create_element("span").unwrap();
        t.set_inner_html(&render.render(&self.props.math));
        Html::VRef(t.into())
    }
}
