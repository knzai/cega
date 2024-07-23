#![cfg(feature = "wasm")]
use yew::prelude::*;
use yew::virtual_dom::VNode;

use cega::wasm::image::*;
use cega::wasm::{FileInput, FileUpload};

pub enum Msg {
    Loaded(FileUpload),
}

pub struct App {
    images: Vec<VNode>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            images: Vec::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.images.push(html! { <ImageComponent file={file} /> });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <h1>{ "Process your CGA/EGAs" }</h1>
                <FileInput accept="image/*,.bin,.cga,.ega" onload={ctx.link().callback( Msg::Loaded )}/>
                <div id="preview-area">
                    {{ self.images.clone() }}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
