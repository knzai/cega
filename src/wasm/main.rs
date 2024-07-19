#![cfg(feature = "wasm")]
extern crate base64;
use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine};
use gloo::file::{callbacks::FileReader, File};
use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html, NodeRef};

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Submit(File),
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
            Msg::Loaded(file_name, file_type, data) => {
                self.files.push(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                true
            }
            Msg::Submit(file) => {
                let link = ctx.link().clone();
                let name = file.name().clone();
                let file_type = file.raw_mime_type();
                let task = {
                    gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        link.send_message(Msg::Loaded(
                            name,
                            file_type,
                            res.expect("failed to read file"),
                        ));
                    })
                };
                self.readers.insert(file.name(), task);
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
                    accept="image/*"
                    multiple={false}
                    onchange={ctx.link().callback( |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let files = gloo::file::FileList::from(input.files().unwrap());
                        let file = files[0].clone();
                        let name = file.name().clone();
                        let file_type = file.raw_mime_type();




                        let msg = Msg::Submit(file);
                        msg
                    })}
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
        let src = format!(
            "data:{};base64,{}",
            file.file_type,
            STANDARD.encode(&file.data)
        );

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
