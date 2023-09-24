use crate::ranid::RanID;
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
            id: RanID::new(),
            name: name.to_string(),
            size: 0,
            drop_type: FileType::CUSTOM("".to_string()),
            data: Buffer::new(),
        }
    }

    pub fn load_from_path(&mut self, path: &str) {}

    pub fn get_buffer(&self) -> &Buffer {
        &self.data
    }

    // Function to update the data buffer
    pub fn update_data(&mut self, data: Buffer) {
        self.data = data
    }
}