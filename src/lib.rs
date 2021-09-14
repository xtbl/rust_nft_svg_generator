use std::fmt::Debug;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct ItemGenerator {
    attributes: Vec<AttributeOptions>,
}

impl ItemGenerator {
    pub fn new() -> ItemGenerator {
        ItemGenerator {
            attributes: Vec::new(),
        }
    }

    pub fn set_attribute_options(&mut self, attr_options: AttributeOptions) -> &mut ItemGenerator {
        self.attributes.push(attr_options);
        self
    }


    pub fn print_content(&self) {
        for attr in &self.attributes {
            println!("attributes: {:?}", attr);
        }
    }
}


#[derive(Debug)]
pub struct AttributeOptions {
    pub name: String,
    pub attribute_options: Vec<String>,
    pub selected_attribute: Option<String>
}

impl AttributeOptions {
    pub fn set_random_attribute(&mut self) -> &mut AttributeOptions {
        // return one item from vec
        let selected = self.attribute_options.choose(&mut rand::thread_rng());
        match selected {
            Some(_) => {
                // unwrap then put in option
                let selection = selected.unwrap();
                println!("selected is {:?}", selected);
                self.selected_attribute = Some(selection.to_owned());
            },
            None => println!("no selected")
        }
        // set into selected_attribute field
        self
    }

}