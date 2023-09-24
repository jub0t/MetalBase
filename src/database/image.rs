use crate::ranid::RanID;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct ImagePointer {
    id: String,
    image_id: String,
}

impl ImagePointer {
    pub fn new(image_id: String) -> Self {
        Self {
            id: RanID::new(),
            image_id,
        }
    }

    pub fn set_image_id(&mut self, image_id: String) {
        self.image_id = image_id
    }
}
