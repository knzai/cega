#![cfg(feature = "wasm")]
use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use cega::wasm::image::ImageFile;
use cega::wasm::FileUpload;

pub enum Msg {
    Loaded(FileUpload),
    Submit(Event),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    images: Vec<ImageFile>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            images: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                self.images.push(ImageFile(file));
            }
            Msg::Submit(e) => {
                let input: HtmlInputElement = e.target_unchecked_into();
                if let Some(files) = input.files() {
                    for file in gloo::file::FileList::from(files).iter() {
                        let link = ctx.link().clone();
                        let name = file.name().clone();
                        let mime_type = file.raw_mime_type();
                        let task = {
                            gloo::file::callbacks::read_as_bytes(&file, move |res| {
                                link.send_message(Msg::Loaded(FileUpload {
                                    data: res.expect("failed to read file"),
                                    mime_type,
                                    name,
                                }))
                            })
                        };
                        self.readers.insert(file.name(), task);
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <label for="file-upload">
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*,.bin,.cga,.ega"
                    multiple={false}
                    onchange={ctx.link().callback(Msg::Submit)}
                />
                <div id="preview-area">
                    { for self.images.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_file(file: &ImageFile) -> Html {
        let output = format!(
            "data:{};base64,{}",
            file.mime_type(),
            STANDARD.encode(&file.data())
        );

        html! {
            <form>
                { &file.name() }
                <img src={output} />
            </form>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
