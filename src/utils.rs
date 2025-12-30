pub fn list_options(opts: Vec<String>) -> String {
    let mut output = String::new();
    output.push_str("------------------------\n");
    output.push_str("Available options:\n");
    for (index, option) in opts.into_iter().enumerate() {
        output.push_str(&format!("{} - {}\n", index + 1, option))
    }
    output.push_str("------------------------");
    return output;
}