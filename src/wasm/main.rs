#![cfg(feature = "wasm")]
extern crate base64;
use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

use cega::color::palette::palette_from_abbr;
use cega::image::Image;
use cega::parser::ParserType;
use cega::png;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(FileDetails),
    Submit,
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    file_browser: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file_browser: NodeRef::default(),
            readers: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                let name = file.name.clone();
                self.files.push(file);
                self.readers.remove(&name);
                true
            }
            Msg::Submit => {
                let el = self.file_browser.cast::<HtmlInputElement>().unwrap();
                if let Some(files) = el.files() {
                    for file in gloo::file::FileList::from(files).iter() {
                        let link = ctx.link().clone();
                        let name = file.name().clone();
                        let file_type = file.raw_mime_type();
                        let task = {
                            gloo::file::callbacks::read_as_bytes(&file, move |res| {
                                link.send_message(Msg::Loaded(FileDetails {
                                    data: res.expect("failed to read file"),
                                    file_type,
                                    name,
                                }))
                            })
                        };
                        self.readers.insert(file.name(), task);
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <p id="title">{ "Process your image files" }</p>
                <label for="file-upload">
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*,.bin,.cga,.ega"
                    multiple={true}
                    ref={&self.file_browser}
                    onchange={ctx.link().callback(|_| Msg::Submit)}
                />
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_file(file: &FileDetails) -> Html {
        let src = if file.file_type.contains("image") {
            format!(
                "data:{};base64,{}",
                file.file_type,
                STANDARD.encode(&file.data)
            )
        } else {
            let image = Image::new(&file.data, 320, ParserType::CGA);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();
            let _ = png::write_to(&mut bytes, image.data(), palette.clone());
            format!("data:image/png;base64,{}", STANDARD.encode(bytes))
        };

        html! {
            <div class="preview-tile">
                <p class="preview-name">{ format!("{}", file.name) }</p>
                <div class="preview-media">
                    <img src={src} />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
