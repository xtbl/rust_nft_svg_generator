use rust_nft_generator::ItemGenerator;
use rust_nft_generator::AttributeOptions;

fn main() {
    let mut new_item = ItemGenerator::new();
    println!("new item is: {:?}", new_item);
    //TODO:
    // x seems like I would need to create a constructor or new
        // function to add the AttributeOptions
    // x read attributeoptions from file
    // open template svg file
    // replace template spaces with attribute content
    // save new file


    // setup AttributeOptions structs
    let mut attr_options_01 = AttributeOptions{name:"eyes".to_string(), attribute_options:vec![], selected_attribute: None};
    let mut attr_options_02 = AttributeOptions{name:"mouth".to_string(), attribute_options:vec![], selected_attribute: None};
    let mut attr_options_03 = AttributeOptions{name:"hat".to_string(), attribute_options:vec![], selected_attribute: None};

    let attribute_files_eyes = vec!["generative_templates/attribute_options_eyes_01.txt",
        "generative_templates/attribute_options_eyes_02.txt"];
    let attribute_files_mouth = vec!["generative_templates/attribute_options_mouth_01.txt",
        "generative_templates/attribute_options_mouth_02.txt"];
    let attribute_files_hat = vec!["generative_templates/attribute_options_hat_01.txt",
        "generative_templates/attribute_options_hat_02.txt"];

    // load file content
    attr_options_01.load_attribute_options("eyes".to_string(), attribute_files_eyes);
    attr_options_02.load_attribute_options("mouth".to_string(), attribute_files_mouth);
    attr_options_03.load_attribute_options("hat".to_string(), attribute_files_hat);

    attr_options_01.set_random_attribute();
    attr_options_02.set_random_attribute();
    attr_options_03.set_random_attribute();

    let mut new_item_with_options = new_item.set_attribute_options(attr_options_01);
    new_item_with_options = new_item_with_options.set_attribute_options(attr_options_02);
    new_item_with_options = new_item_with_options.set_attribute_options(attr_options_03);

    new_item_with_options.print_content();
    // new_item_with_options.generate_file("generated_new_file_01.svg");


    new_item_with_options.generate_files(3);


}

