#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct ImagePointer {
    image_id: String,
}

impl ImagePointer {
    pub fn new(image_id: String) -> Self {
        Self { image_id }
    }

    pub fn set_id(&mut self, image_id: String) {
        self.image_id = image_id
    }
}
