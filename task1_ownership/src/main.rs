fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::from(str1);
    result.push_str(str2);
    result
}
fn main() {
    let first = String::from("Hi,");
    let sec = String::from("Amal!");
    let concat_str = concatenate_strings(&first, &sec);
    println!("concat string:{}", concat_str);
}
