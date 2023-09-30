use std::fs;

use crate::response::Bytes;
use crate::rid::RanID;

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
    data: Bytes,
}

impl Katra {
    pub fn new(name: &str) -> Self {
        Self {
            id: RanID::new(),
            name: name.to_string(),
            size: 0,
            drop_type: FileType::CUSTOM("".to_string()),
            data: Bytes::new(),
        }
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        return match fs::read(path) {
            Ok(data) => {
                self.data = data;
                true
            }

            Err(error) => {
                false
            }
        };
    }


    pub fn write_to_path(&self, path: &str) -> bool {
        return match fs::write(path, &self.data) {
            Ok(_) => {
                true
            }
            Err(error) => {
                false
            }
        };
    }

    pub fn get_buffer(&self) -> &Bytes {
        &self.data
    }

    // Function to update the data buffer
    pub fn update_data(&mut self, data: Bytes) {
        self.data = data
    }
}