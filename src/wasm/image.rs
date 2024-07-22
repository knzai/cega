use base64::{engine::general_purpose::STANDARD, Engine};

use yew::{html, Component, Context, Html, Properties};

use crate::color::palette::palette_from_abbr;
use crate::image::Image;
use crate::parser::ParserType;
use crate::png;

use crate::wasm::FileUpload;

pub struct ImageFile<'a>(pub &'a FileUpload);

impl ImageFile<'_> {
    pub fn name(&self) -> String {
        if self.0.mime_type.contains("image") {
            self.0.name.to_string()
        } else {
            format!("{}{}", self.0.name, ".png")
        }
    }

    pub fn mime_type(&self) -> String {
        if self.0.mime_type.contains("image") {
            self.0.mime_type.to_string()
        } else {
            "image/png".to_string()
        }
    }

    pub fn data(&self) -> Vec<u8> {
        if self.0.mime_type.contains("image") {
            self.0.data.clone()
        } else {
            let image = Image::new(&self.0.data, 320, ParserType::CGA);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();
            let _ = png::write_to(&mut bytes, image.data(), palette.clone());
            bytes
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ICProps {
    pub file: FileUpload,
}

pub struct ImageComponent;

impl Component for ImageComponent {
    type Message = ();
    type Properties = ICProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image = ImageFile(&ctx.props().file);

        let output = format!(
            "data:{};base64,{}",
            image.mime_type(),
            STANDARD.encode(image.data())
        );

        html! {
            <form>
                { &image.name() }
                <img src={output} />
            </form>
        }
    }
}
