use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read_file(folder_name: &str, file_name: &str) -> String {
    let path = format!("data/{folder_name}/{file_name}");
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}
