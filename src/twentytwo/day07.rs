use crate::create_advent_day;

create_advent_day!("2022", "07");

fn part1_with_input(input: &str) -> i32 {
    let mut file_system = FileSystem::parse(&input);
    file_system.root_dir.size();
    return file_system.sum_small_dirs();
}

fn part2_with_input(input: &str) -> i32 {
    let mut file_system = FileSystem::parse(&input);
    file_system.root_dir.size();
    return file_system.smallest_dir_to_delete();
}

struct FileSystem {
    root_dir: Dir,
}

impl FileSystem {
    fn parse(input: &str) -> FileSystem {
        let mut curr_path: Vec<&str> = Vec::new();
        let mut file_system: FileSystem = FileSystem {
            root_dir: Dir::new("/"),
        };
        let mut curr_dir = &mut file_system.root_dir;
        for line in input.lines().skip(1) {
            if line.starts_with("$ cd ..") {
                curr_path.pop();
                curr_dir = file_system.get_dir(&path(&curr_path));
            } else if line.starts_with("$ cd") {
                curr_path.push(line.split("cd ").nth(1).unwrap());
                curr_dir = file_system.get_dir(&path(&curr_path));
            } else if line.starts_with("$ ls") {
            } else if line.starts_with("dir ") {
                let mut split = line.split(" ");
                let name = split.nth(1).unwrap();
                curr_dir.dirs.push(Dir::new(name));
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                let file = File {
                    size: size.parse::<i32>().unwrap(),
                    _name: name.to_owned(),
                };
                curr_dir.files.push(file);
            }
        }

        return file_system;
    }

    fn get_dir(&mut self, full_path: &str) -> &mut Dir {
        let path_parts = full_path[1..].split("/");
        let mut dir = &mut self.root_dir;
        'outer: for path in path_parts {
            if path.len() == 0 {
                continue;
            }
            let num_sub_dirs = dir.dirs.len();
            for i in 0..num_sub_dirs {
                if dir.dirs[i].name == path {
                    dir = &mut dir.dirs[i];
                    continue 'outer;
                }
            }
            panic!("did not find {} in dir {}", path, dir.name);
        }

        return dir;
    }

    fn sum_small_dirs(self) -> i32 {
        let mut dirs = Vec::new();
        let mut add_my_children = Vec::new();

        add_my_children.push(&self.root_dir);
        while !add_my_children.is_empty() {
            let current_dir = add_my_children.pop().unwrap();
            for dir in &current_dir.dirs {
                add_my_children.push(&dir);
                dirs.push(dir);
            }
        }

        return dirs
            .iter()
            .filter(|dir| dir.size <= 100_000)
            .map(|dir| dir.size)
            .sum();
    }

    fn smallest_dir_to_delete(self) -> i32 {
        // TODO: can we reuse the previous method instead of copy/paste?
        let mut dirs = Vec::new();
        let mut add_my_children = Vec::new();

        add_my_children.push(&self.root_dir);
        while !add_my_children.is_empty() {
            let current_dir = add_my_children.pop().unwrap();
            for dir in &current_dir.dirs {
                add_my_children.push(&dir);
                dirs.push(dir);
            }
        }

        let current_free_space = 70_000_000 - self.root_dir.size;
        return dirs
            .iter()
            .filter(|dir| dir.size + current_free_space >= 30_000_000)
            .map(|dir| dir.size)
            .min()
            .unwrap();
    }
}

struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
    size: i32,
}

impl Dir {
    fn new(name: &str) -> Dir {
        return Dir {
            name: name.to_string(),
            dirs: Vec::new(),
            files: Vec::new(),
            size: 0,
        };
    }

    fn size(&mut self) -> i32 {
        if self.size != 0 {
            return self.size;
        }
        let file_size: i32 = self.files.iter().map(|file| file.size).sum();
        let dir_size: i32 = self.dirs.iter_mut().map(|dir| dir.size()).sum();
        self.size = file_size + dir_size;
        return self.size;
    }
}

struct File {
    _name: String,
    size: i32,
}

fn path(path: &Vec<&str>) -> String {
    return format!("/{}", path.join("/"));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_file_system() -> FileSystem {
        return FileSystem::parse(&utils::read_file("2022/day07", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "95437");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "24933642");
    }

    #[test]
    fn file_system_parse_works() {
        let actual = test_file_system();

        assert_eq!(actual.root_dir.dirs.len(), 2);
        assert_eq!(actual.root_dir.files.len(), 2);
    }

    #[test]
    fn file_system_get_dir_works() {
        let mut file_system = test_file_system();

        let dir_a = file_system.get_dir("/a");
        assert_eq!(dir_a.name, "a");
        assert_eq!(dir_a.dirs.len(), 1);
        assert_eq!(dir_a.files.len(), 3);

        let dir_e = file_system.get_dir("/a/e");
        assert_eq!(dir_e.name, "e");
        assert!(dir_e.dirs.is_empty());
        assert_eq!(dir_e.files.len(), 1);
    }
}
