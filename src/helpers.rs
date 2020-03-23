use std::fs;

pub fn read_input_file(year: i16, day: i8) -> String {
    let file_name = format!("./data/{}/{}.txt", year, day);
    fs::read_to_string(file_name).expect("File not found")
}
