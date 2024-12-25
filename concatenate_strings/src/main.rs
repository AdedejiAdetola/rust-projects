fn main() {
    let mut string1 = String::from("Hello");
    let string2 = String::from("Human");

    let concatenated_string = concatenate_strings(&mut string1, &string2);
    println!("Concatenated string is {}", concatenated_string)
}


fn concatenate_strings(s1: &mut String, s2: &String) -> String {
    s1.push_str(s2);
    let result = s1.clone();
    result
}