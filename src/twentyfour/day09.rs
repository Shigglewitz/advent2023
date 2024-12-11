use std::usize;

use crate::create_advent_day;

create_advent_day!("2024", "09");

fn part1_with_input(input: &str) -> i64 {
    let mut disk: Vec<DiskBlock> = input
        .chars()
        .enumerate()
        .map(|(char_num, char)| {
            let length = char.to_digit(10).unwrap() as usize;
            let mut blocks = Vec::with_capacity(length);
            if length != 0 {
                let is_even = char_num % 2 == 0;
                for _ in 0..length {
                    if is_even {
                        blocks.push(DiskBlock {
                            file_id: char_num / 2,
                            is_empty: false,
                        });
                    } else {
                        blocks.push(DiskBlock {
                            file_id: usize::MAX,
                            is_empty: true,
                        })
                    }
                }
            }
            blocks
        })
        .flatten()
        .collect::<Vec<DiskBlock>>();

    let num_blocks = disk.len();
    let mut j: usize = 0;
    for i in (0..num_blocks).rev() {
        if disk[i].is_empty {
            continue;
        };
        loop {
            if disk[j].is_empty {
                break;
            }
            j += 1;
        }
        if j >= i {
            break;
        }
        disk.swap(i, j);
    }

    let checksum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(block_num, block)| {
            if block.is_empty {
                None
            } else {
                Some(block.file_id * block_num)
            }
        })
        .sum();
    return checksum as i64;
}

struct DiskBlock {
    file_id: usize,
    is_empty: bool,
}

fn part2_with_input(input: &str) -> i64 {
    let mut start_index: usize = 0;
    let mut disk: Vec<DiskSegment> = input
        .chars()
        .enumerate()
        .map(|(char_num, char)| {
            let length = char.to_digit(10).unwrap() as usize;
            let is_even = char_num % 2 == 0;
            let segment = if is_even {
                DiskSegment {
                    file_id: char_num / 2,
                    is_empty: false,
                    start_index,
                    length,
                }
            } else {
                DiskSegment {
                    file_id: usize::MAX,
                    is_empty: true,
                    start_index,
                    length,
                }
            };
            start_index += length;
            segment
        })
        .collect::<Vec<DiskSegment>>();

    let mut index = disk.len() - 1;
    loop {
        if disk[index].is_empty {
            index -= 1;
            continue;
        }
        let mut free_index = usize::MAX;
        for j in 0..index {
            if disk[j].is_empty && disk[j].length >= disk[index].length {
                free_index = j;
                break;
            }
        }
        if free_index != usize::MAX {
            let fragment_free_space = disk[free_index].length > disk[index].length;
            disk.swap(index, free_index);
            let temp = disk[index].start_index;
            disk[index].start_index = disk[free_index].start_index;
            disk[free_index].start_index = temp;
            if fragment_free_space {
                let difference = disk[index].length - disk[free_index].length;
                disk[index].length = disk[free_index].length;
                disk.insert(
                    free_index + 1,
                    DiskSegment {
                        file_id: usize::MAX,
                        is_empty: true,
                        start_index: disk[free_index].start_index + disk[free_index].length,
                        length: difference,
                    },
                );
                index += 1;
            }
        }
        index -= 1;
        if index == 0 {
            break;
        }
    }

    return disk.iter().map(|segment| segment.checksum()).sum();
}

struct DiskSegment {
    file_id: usize,
    is_empty: bool,
    start_index: usize,
    length: usize,
}

impl DiskSegment {
    fn checksum(&self) -> i64 {
        if self.is_empty {
            return 0;
        }
        let mut sum = 0;
        for i in 0..self.length {
            sum += self.file_id * (self.start_index + i)
        }

        return sum as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("1928", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("2858", &actual);
    }
}
