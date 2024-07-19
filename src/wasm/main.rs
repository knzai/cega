#![cfg(feature = "wasm")]
extern crate base64;
use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine};
use gloo::file::{callbacks::FileReader, File};
use gloo_console::debug;
use web_sys::File as RawFile;
use web_sys::{Event, FormData, HtmlFormElement};
use yew::prelude::*;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Submit(FormData),
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
            }
            Msg::Submit(form) => {
                // let link = ctx.link().clone();
                // let name = file.name().clone();
                // let file_type = file.raw_mime_type();
                // let task = {
                //     gloo::file::callbacks::read_as_bytes(&file, move |res| {
                //         link.send_message(Msg::Loaded(
                //             name,
                //             file_type,
                //             res.expect("failed to read file"),
                //         ));
                //     })
                // };
                // self.readers.insert(file.name(), task);

                //                 let form: HtmlFormElement = e.target_unchecked_into();
                //
                // let alt_text = form_data.get("alt-text");
                // let image_file = File::from(RawFile::from(form_data.get("file-upload")));
                // let link = ctx.link().clone();
                // let name = image_file.name().clone();
                // let file_type = image_file.raw_mime_type();
                // let task = {
                //     gloo::file::callbacks::read_as_bytes(&image_file, move |res| {
                //         link.send_message(Msg::Loaded(
                //             name,
                //             file_type,
                //             res.expect("failed to read file"),
                //         ));
                //     })
                // };
                // self.readers.insert(name.clone(), task);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();

        let onsubmit = move |e: SubmitEvent| {
            let form: HtmlFormElement = e.target_unchecked_into();
            let form_data = FormData::new_with_form(&form).expect("form data");
            //let image_file = File::from(RawFile::from(form_data.get("file-upload")));
            debug!(form_data.clone()); //seems to be an empty formdata
            link.send_message(Msg::Submit(form_data));
            e.prevent_default();
        };

        html! {
            <div id="wrapper">
                <form onsubmit={onsubmit}>
                    <p id="title">{ "Process your image files" }</p>
                    <input name="alt-text"/>
                    <input
                        id="file-upload"
                        name="file-upload"
                        type="file"
                        accept="image/*"
                        multiple={false}
                    />
                    <input type="submit"/>
                </form>
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
