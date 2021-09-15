use std::fmt::Debug;
use rand::seq::SliceRandom;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;

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
        println!("ItemGenerator -----------------");
        for attr in &self.attributes {
            println!("selected_attribute: {:?}", attr.selected_attribute);
        }
    }

    pub fn generate_file(&self) {
        // read file
        let mut file_contents = fs::read_to_string("generative_templates/generative_template_01.svg").expect("Error reading file");
        println!("-------- filecontents {}", file_contents);

        // replace contents with attributes
        for attr in &self.attributes {
            match attr.name.as_str() {
                "eyes" => {
                    file_contents = file_contents.replace("$EYES$", &attr.selected_attribute.as_ref().unwrap());
                },
                "mouth" => {
                    file_contents = file_contents.replace("$MOUTH$", &attr.selected_attribute.as_ref().unwrap());
                },
                "hat" => {
                    file_contents = file_contents.replace("$HAT$", &attr.selected_attribute.as_ref().unwrap());
                },
                _ => {
                    ()
                }
            }
        }
        println!("-------- filecontents {}", file_contents);

        // TODO:
        // set to generate n new files instead of hardcoded name

        let path = Path::new("generated_07.svg");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(file_contents.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
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
    pub fn get_attributes_from_file(&mut self, filename: &str) -> String {
        let file_contents = fs::read_to_string(filename).expect("Error reading file");
        println!("file contents is: {}", file_contents);
        // TODO: push file_contents into attribute_options
        file_contents
    }

    pub fn load_attribute_options(&mut self, attribute_name: String, file_names: Vec<&str>) {
        self.name = attribute_name;
        let mut loaded_attributes = vec![];
        for file_name in file_names {
            loaded_attributes.push(self.get_attributes_from_file(file_name));
        }
        println!("loaded_attributes: {:?}", loaded_attributes);
        self.attribute_options = loaded_attributes;
    }

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