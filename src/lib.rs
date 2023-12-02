pub mod array;

#[macro_export]
macro_rules! aoc {
    ($part_one:ident, $part_two:ident) => {
        fn main() {
            let stem = std::path::Path::new(file!()).file_stem().unwrap();
            let path = format!("src/input/{}.txt", stem.to_str().unwrap());
            let text = std::fs::read_to_string(path).unwrap();
            println!("{}", $part_one(&text));
            println!("{}", $part_two(&text));
        }
    };
}

pub fn select<T>(condition: bool, a: T, b: T) -> T {
    if condition { a } else { b }
}
