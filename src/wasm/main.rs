#![cfg(feature = "wasm")]
use yew::prelude::*;

use cega::wasm::image::*;
use cega::wasm::{FileInput, FileUpload};

pub enum Msg {
    Loaded(FileUpload),
}

pub struct App {
    files: Vec<FileUpload>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: Vec::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.files.push(file);
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
                    {for self.files.iter().map(|f|
                        html! { <ImageComponent file={f.clone()} /> }
                    )}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
