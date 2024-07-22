pub mod image;

#[derive(Clone, PartialEq)]
pub struct FileUpload {
    pub name: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}
