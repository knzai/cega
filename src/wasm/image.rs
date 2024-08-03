use base64::{engine::general_purpose::STANDARD, Engine};

use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Event, Html, Properties, SubmitEvent, TargetCast};

use crate::color::palette::palette_from_abbr;
use crate::file_data;
use crate::image::tile;
use crate::parser::ParserType;
use crate::png;

use crate::wasm::FileUpload;

pub struct ImageFile<'a> {
    file_input: &'a FileUpload,

    width: usize,
    height: usize,
}

impl ImageFile<'_> {
    pub fn name(&self) -> String {
        if self.file_input.mime_type.contains("image") {
            self.file_input.name.to_string()
        } else {
            format!("{}{}", self.file_input.name, ".png")
        }
    }

    pub fn mime_type(&self) -> String {
        if self.file_input.mime_type.contains("image") {
            self.file_input.mime_type.to_string()
        } else {
            "image/png".to_string()
        }
    }

    pub fn data(&self) -> Vec<u8> {
        if self.file_input.mime_type.contains("image") {
            self.file_input.data.clone()
        } else {
            let file_data = file_data::Raw::new(&self.file_input.data);
            let parser = ParserType::CGA;
            let image = file_data.parse(parser, self.width);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();

            let _ = png::write_to(&mut bytes, tile(image.data(), self.height), palette.clone());
            bytes
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub file: FileUpload,
}

pub struct ImageComponent {
    width: usize,
    height: usize,
}

pub enum Msg {
    Width(Event),
    Height(Event),
}

impl Component for ImageComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            width: 320,
            height: 200,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Width(e) => {
                let input: HtmlInputElement = e.target_unchecked_into();
                self.width = input.value().parse().expect("fail to parse width");
            }
            Msg::Height(e) => {
                let input: HtmlInputElement = e.target_unchecked_into();
                self.height = input.value().parse().expect("fail to parse width");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image = ImageFile {
            file_input: &ctx.props().file,
            width: self.width,
            height: self.height,
        };

        let output = format!(
            "data:{};base64,{}",
            image.mime_type(),
            STANDARD.encode(image.data())
        );

        let noop = Callback::from(|e: SubmitEvent| {
            e.prevent_default();
        });

        html! {
            <div class="preview-tile ">
                <form onsubmit={noop}>
                        <label for="width">{"[Tile] Width"}</label>
                        <input name="width" type="number" value={image.width.to_string()} onchange={ctx.link().callback(Msg::Width)} />
                        <label for="height">{"[Tile] Height"}</label>
                        <input name="height" type="number" value={image.height.to_string()} onchange={ctx.link().callback(Msg::Height)} />
                </form>
                <p class="preview-name">{ &image.name() }</p>
                <div class=".preview-media">
                    <img src={output} />
                </div>
            </div>
        }
    }
}
