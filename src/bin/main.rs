use rust_nft_generator::ItemGenerator;
use rust_nft_generator::AttributeOptions;

fn main() {
    let mut new_item = ItemGenerator::new();
    println!("new item is: {:?}", new_item);
    //TODO: seems like I would need to create a constructor or new
    // function to add the AttributeOptions
    // - parse attribute options from file 
    let mut attr_options_01 =
        AttributeOptions{name:"eyes".to_string(),
            attribute_options:vec!["eyes01".to_string(), "eyes02".to_string()], selected_attribute: None};
    let mut attr_options_02 =
        AttributeOptions{name:"mouth".to_string(),
            attribute_options:vec!["mouth01".to_string(), "mouth02".to_string()], selected_attribute: None};

    attr_options_01.set_random_attribute();
    attr_options_02.set_random_attribute();

    let mut new_item_with_options = new_item.set_attribute_options(attr_options_01);

    new_item_with_options = new_item_with_options.set_attribute_options(attr_options_02);
    new_item_with_options.print_content();

}

