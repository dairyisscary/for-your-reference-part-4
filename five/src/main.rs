fn find_first_char(haystack: &str, needle: char) -> &str {
    for (i, item) in haystack.chars().enumerate() {
        if needle == item {
            return &haystack[i..];
        }
    }
    ""
}

fn main() {
    let vec = vec![1, 2, 3];
    let without_first = &vec[1..];
    println!("{:?}", without_first);

    let only_second = &vec[1..2];
    println!("{:?}", only_second);

    let without_last = &vec[..2];
    println!("{:?}", without_last);

    let mut dynamic_string: String = String::from("Hello Noah");
    println!("\nA String (!):\n{}", dynamic_string);
    dynamic_string.push_str(" -- more stuff");

    let immutable_string_slice: &str = &dynamic_string[1..];
    println!("{}", immutable_string_slice);

    let literal: &str = "hi there I'm a literal";
    println!("\nA literal:\n{}", literal);

    let found_o = find_first_char(&dynamic_string, 'o');
    dynamic_string.push_str(" -- even more stuff");
    println!("\nSearching:\n{}", found_o); // oppps ?!?!
}
