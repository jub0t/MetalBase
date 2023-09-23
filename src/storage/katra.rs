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
    drop_type: FileType,
    size: usize,
}

impl Katra {}