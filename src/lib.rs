use std::fmt::Debug;

#[derive(Debug)]
pub struct ItemGenerator {
    attributes: Vec<AttributeOptions>,
}

#[derive(Debug)]
pub struct AttributeOptions {
    name: String,
    attribute_options: Vec<String>,
}

impl ItemGenerator {
    pub fn new() -> ItemGenerator {
        ItemGenerator {
            attributes: Vec::new(),
        }
    }

    pub fn setAttributeOptions(&mut self, attr_options: AttributeOptions) -> &mut ItemGenerator {
        self.attributes.push(attr_options);
        self
    }
}