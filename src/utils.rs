pub fn print_element<T: std::fmt::Debug + ToString>(vec: Vec<T>) {
    for element in vec {
        println!("{}", (element).to_string().replace('"', ""));
    }
}