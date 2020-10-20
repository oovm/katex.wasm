#![recursion_limit = "1024"]

use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};
use yew_katex::KaTeX;

pub fn header_view() -> Html {
    let title = "Markups to Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/note-to-yew">{"Fork me!"}</a>
    </header>
    }
}

pub enum Event {
    Input(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    input: String,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, input: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => self.input = s,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {header_view()}
                <main>
                <textarea
                     placeholder="输入待加解密内容"
                     name="text"
                     oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                 />
                <div><label>{"KaTeX Output:"}</label></div>
                <KaTeX math=&self.input inline=false/>
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
