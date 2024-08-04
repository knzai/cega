use base64::{engine::general_purpose::STANDARD, Engine};

use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Event, Html, Properties, SubmitEvent, TargetCast};

use crate::color::palette::palette_from_abbr;
use crate::image::tile;
use crate::parser::ParserType;
use crate::{file_data, png};

use crate::wasm::FileUpload;

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

impl ImageComponent {
    pub fn src(&self, file: &FileUpload) -> String {
        let data = if file.mime_type.contains("image") {
            file.data.clone()
        } else {
            let file_data = file_data::Raw::new(&file.data);
            let parser = ParserType::CGA;
            let image = file_data.parse(parser, self.width);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();

            let _ = png::write_to(&mut bytes, tile(image.data(), self.height), palette.clone());
            bytes
        };
        format!("data:application/png;base64,{}", STANDARD.encode(data))
    }

    pub fn previews(&self, file: &FileUpload) -> Html {
        if file.mime_type.contains("image") {
            "".into()
        } else {
            let file_data = file_data::Raw::new(&file.data);
            file_data
                .previews()
                .iter()
                .map(|p| {
                    let palette = palette_from_abbr("cga0");
                    let mut bytes: Vec<u8> = Vec::new();

                    let _ = png::write_to(&mut bytes, tile(p.data(), self.height), palette.clone());
                    let src = format!("data:application/png;base64,{}", STANDARD.encode(bytes));
                    html! {
                        <span>
                            {p.width()}
                            <img src={ src } />
                        </span>
                    }
                })
                .collect()
        }
    }
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
        let noop = Callback::from(|e: SubmitEvent| {
            e.prevent_default();
        });
        let file = &ctx.props().file;

        html! {
            <>
                <div class="preview-tile ">
                    <div class=".preview-media">
                        <p class="preview-name">{ file.name.to_string() }</p>
                        <img src={ self.src(file) } />
                    </div>
                    <form onsubmit={noop}>
                            <label for="width">{"[Tile] Width"}</label>
                            <input name="width" type="number" value={self.width.to_string()} onchange={ctx.link().callback(Msg::Width)} />
                            <label for="height">{"[Tile] Height"}</label>
                            <input name="height" type="number" value={self.height.to_string()} onchange={ctx.link().callback(Msg::Height)} />
                    </form>
                </div>
                {self.previews(file)}
            </>
        }
    }
}
