#![cfg(feature = "wasm")]
use std::collections::VecDeque;

use yew::prelude::*;
use yew::virtual_dom::VNode;

use cega::wasm::image::*;
use cega::wasm::{FileInput, FileUpload};

pub enum Msg {
    Loaded(FileUpload),
}

pub struct App {
    images: VecDeque<VNode>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            images: VecDeque::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.images
                    .push_front(html! { <ImageComponent file={file} /> });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let images: Vec<VNode> = self.images.clone().into();
        html! {
            <div id="wrapper">
                <h1>{ "Process your CGA/EGAs" }</h1>
                <FileInput accept="image/png,.bin,.cga,.ega" onload={ctx.link().callback( Msg::Loaded )} children={None}/>
                <div id="preview-area">{{ images }}</div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
