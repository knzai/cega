#![cfg(feature = "wasm")]
use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use cega::color::palette::palette_from_abbr;
use cega::image::Image;
use cega::parser::ParserType;
use cega::png;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

impl FileDetails {
    pub fn output(&self) -> String {
        format!(
            "data:{};base64,{}",
            self.file_type,
            STANDARD.encode(&self.data)
        )
    }

    pub fn process(mut self) -> Self {
        if !self.file_type.contains("image") {
            let image = Image::new(&self.data, 320, ParserType::CGA);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();
            let _ = png::write_to(&mut bytes, image.data(), palette.clone());
            self.data = bytes;
            self.name += ".png";
        }
        self
    }
}

pub enum Msg {
    Loaded(FileDetails),
    Submit(Event),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                self.files.push(file.process());
            }
            Msg::Submit(e) => {
                let input: HtmlInputElement = e.target_unchecked_into();
                if let Some(files) = input.files() {
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
            }
        }
        true
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
                    multiple={false}
                    onchange={ctx.link().callback(Msg::Submit)}
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
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ &file.name }</p>
                <div class="preview-media">
                    <img src={file.output()} />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
