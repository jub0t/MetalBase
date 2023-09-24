use uuid::Uuid;

use crate::response::Buffer;

pub enum FileType {
    // Images/Graphics
    GIF,
    PNG,
    WEB,
    JPEG,
    JPG,

    // Text-Based
    TXT,
    MD,

    // Advanced
    PDF,
    DOCX,
    CUSTOM(String),
}

pub struct Katra {
    id: String,
    name: String,
    size: usize,
    drop_type: FileType,
    data: Buffer,
}

impl Katra {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            size: 0,
            drop_type: FileType::CUSTOM("".to_string()),
            data: Buffer::new(),
        }
    }

    pub fn load_from_path(&mut self, path: &str) {}
}