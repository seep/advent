pub mod array;

#[macro_export]
macro_rules! read_input {
    () => {{
        let stem = std::path::Path::new(file!()).file_stem().unwrap();
        let path = format!("src/input/{}.txt", stem.to_str().unwrap());
        std::fs::read_to_string(path).unwrap().as_str()
    }};
}
