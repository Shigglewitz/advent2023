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

#[test]
fn read_file_works() {
    let expected: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .lines()
        .collect();

    let actual: String = read_file("day1", "test_1.txt").lines().collect();

    assert_eq!(expected, actual);
}
