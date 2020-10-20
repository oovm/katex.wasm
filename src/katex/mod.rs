use katex_wasmbind::KaTeXOptions;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct KaTeXProperties {
    pub math: String,
    #[prop_or(true)]
    pub inline: bool,
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
        match self.props.inline {
            true => {
                let render = KaTeXOptions::inline_mode();
                let t = yew::utils::document().create_element("span").unwrap();
                // t.set_class_name("katex-inline");
                t.set_inner_html(&render.render(&self.props.math));
                Html::VRef(t.into())
            }
            false => {
                let render = KaTeXOptions::display_mode();
                let t = yew::utils::document().create_element("span").unwrap();
                // t.set_class_name("katex-display");
                t.set_inner_html(&render.render(&self.props.math));
                Html::VRef(t.first_child().unwrap().into())
            }
        }
    }
}
