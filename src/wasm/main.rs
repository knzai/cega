#![cfg(feature = "wasm")]
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Submit,
}

struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub struct App {
    width: NodeRef,
    file: NodeRef,
    width_value: String,
    file_value: Option<FileDetails>,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file: NodeRef::default(),
            width: NodeRef::default(),
            width_value: "".to_string(),
            file_value: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                self.file_value = Some(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                true
            }
            Msg::Submit => {
                let width_field = self.width.cast::<HtmlInputElement>().unwrap();
                self.width_value = if width_field.value() == "" {
                    width_field.placeholder()
                } else {
                    width_field.value()
                };
                // let el = self.file.cast::<HtmlInputElement>().unwrap();
                // if let Some(file) = el.files().and_then(|m| m.item(0)) {
                //     let file_name = el.name();
                //     let file_type = file.raw_mime_type();

                //     let task = {
                //         let link = ctx.link().clone();
                //         let file_name = file_name.clone();

                //         gloo::file::callbacks::read_as_bytes(&file, move |res| {
                //             link.send_message(Msg::Loaded(
                //                 file_name,
                //                 file_type,
                //                 res.expect("failed to read file"),
                //             ))
                //         })
                //     };
                // }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="main">
                <h1>{"Process your CGA"}</h1>
                <div class="input-container">
                    <label>{ "Width" }</label>
                    <input
                        type="text"
                        ref={&self.width}
                        class="input-element"
                        placeholder="320"
                        value={self.width_value.clone()}
                    />
                </div>
                <input
                    id="file-upload"
                    type="file"
                    ref={&self.file}
                    accept="image/*,.bin,.cga,.ega"
                    multiple={false}
                    onchange={ctx.link().callback(|_| Msg::Submit)}
                />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
