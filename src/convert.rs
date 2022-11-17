pub fn convert_str_input_to_string_input(input: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    let mut v = Vec::with_capacity(input.len());
    for i in input {
        let mut vi = Vec::with_capacity(i.len());
        for value in i {
            vi.push(String::from(value));
        }

        v.push(vi);
    }

    v
}
