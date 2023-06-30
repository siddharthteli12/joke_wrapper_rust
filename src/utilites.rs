use std::vec;

pub fn enum_join<T: ToString>(list_enum: &Vec<T>) -> Vec<String> {
    let mut list_string = vec![];
    for item in list_enum {
        list_string.push(item.to_string());
    }
    list_string
}
