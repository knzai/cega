use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FileUpload {
    pub name: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

pub struct FileInput {
    readers: HashMap<String, FileReader>,
}

pub enum Msg {
    Loaded(FileUpload),
    Submit(Option<web_sys::FileList>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onload: Callback<FileUpload>,
    #[prop_or(AttrValue::Static(""))]
    pub accept: AttrValue,
    #[prop_or(false)]
    pub multiple: bool,
    #[prop_or(false)]
    pub drag_and_drop: bool,
    #[prop_or(AttrValue::Static("Drop Files Here"))]
    pub label: AttrValue,
    pub children: Option<Html>,
}

impl Component for FileInput {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                ctx.props().onload.emit(file);
            }
            Msg::Submit(files) => {
                if let Some(files) = files {
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
        let label_text = &ctx.props().label;
        let children = ctx.props().children.clone();
        let accept = &ctx.props().accept;
        let multiple = ctx.props().multiple;

        let noop_drag = Callback::from(|e: DragEvent| {
            e.prevent_default();
        });
        let onchange = ctx.link().callback(|e: Event| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Submit(input.files())
        });
        let ondrop = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            Msg::Submit(event.data_transfer().unwrap().files())
        });

        html! {
            <label onchange={ &onchange } >
                if ctx.props().drag_and_drop {
                    <i class="drop-container" { ondrop } ondragover={ &noop_drag } ondragenter={ &noop_drag }>
                        { label_text.clone() }
                    </i>
                } else {
                    <i>{ label_text.clone() }</i>
                }
                if children.is_none() {
                    <input type="file" { accept } { multiple } { onchange } />
                } else {
                    { children }
                }
            </label>
        }
    }
}
