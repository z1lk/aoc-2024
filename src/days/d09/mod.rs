use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Clone,Debug)]
enum Chunk {
    File { size: u8, id: i32 },
    Free { size: u8 }
}

fn parse(input: &str) -> Vec<Chunk> {
    let mut disk: Vec<Chunk> = Vec::new();
    let mut file = true;
    let mut file_id = 0;
    for c in input.chars() {
        if let Some(n) = c.to_digit(10) {
            if file {
                disk.push(Chunk::File { size: n as u8, id: file_id });
                file_id += 1;
            } else {
                disk.push(Chunk::Free { size: n as u8 });
            }
            file = !file;
        }
    }
    disk
}

pub fn part_1(input: &str) -> i64 {
    let disk = parse(input);
    let mut fragmented: Vec<Chunk> = Vec::new();

    for chunk in &disk {
        match chunk {
            Chunk::Free { size } => {
                for _ in (0..*size) {
                    fragmented.push(Chunk::Free { size: 1_u8 })
                }
            }
            Chunk::File { size, id } => {
                for _ in (0..*size) {
                    fragmented.push(Chunk::File { size: 1_u8, id: *id })
                }
            }
        }
    }

    fragmented = frontload(fragmented);
    //print(&fragmented);
    checksum(fragmented)
}

pub fn part_2(input: &str) -> i64 {
    let mut disk = parse(input);
    disk = frontload(disk);
    //print(&disk);
    checksum(disk)
}

// Generalized algorithm for both parts 1 & 2, that moves chunks of files towards
// the end of the disk to free space toward the beginning of the disk.
// To avoid having to borrow the disk vec as we search over it, we maintain two pointers:
// - `file_ptr` starts at the end of the disk and works backwards, pointing to a File, which it is
// trying to find space for towards the beginning of the disk.
// - `free_ptr` is for the inner loop that is searching for free disk space.
// By not borrowing, when an ample free chunk is found, we are able to mutate the disk.
fn frontload(mut disk: Vec<Chunk>) -> Vec<Chunk> {
    let mut file_ptr = disk.len() - 1;
    let mut earliest_free = 0;
    while file_ptr > 0 {
        //println!("file ptr: {:?}", file_ptr);
        match disk.get(file_ptr).unwrap().clone() {
            // skip over free chunks; we don't move them.
            Chunk::Free { size } => { file_ptr -= 1 }
            Chunk::File { size, id } => {
                let mut free_ptr = earliest_free;
                let mut found_new_earliest_free = false;
                while free_ptr < file_ptr {
                    if let Chunk::Free { size: size2 } = disk.get(free_ptr).unwrap() {
                        // track the earliest free chunk as we are looking for space for files,
                        // so we don't have to start from the beginning each time.
                        if !found_new_earliest_free {
                            if free_ptr > earliest_free {
                                earliest_free = free_ptr;
                            }
                            found_new_earliest_free = true;
                        }
                        if *size2 >= size {
                            let mut replace_with = vec![Chunk::File { size: size, id: id }];
                            let remaining = size2 - size;
                            // for part 2. if the free space is larger than the file, fill
                            // the remaining free space with a new Free chunk.
                            if remaining > 0 {
                                replace_with.push(Chunk::Free { size: remaining });
                                file_ptr += 1;
                            }
                            disk.splice(free_ptr..(free_ptr+1), replace_with);
                            disk.remove(file_ptr);
                            // replace the moved file with free space in case some later files
                            // weren't moved, because file position matters for the final checksum
                            disk.insert(file_ptr, Chunk::Free { size });
                            // if we added an extra Free in addition to the File, inc free_ptr so
                            // that when it is decremented below, it will effectively stay where it
                            // is, yet reference the next element since everything been shifted up.
                            if remaining > 0 {
                                free_ptr += 1;
                            }
                            // doesn't seem to affect solution speed
                            /* if empty_beyond(&disk, file_ptr) {
                                println!("trimming disk @ {:?}", file_ptr);
                                disk.drain(file_ptr..);
                                print(&disk);
                            } */
                            break;
                        }
                    }
                    free_ptr += 1;
                }
                file_ptr -= 1;

            }
        }
    }
    disk
}

fn empty_beyond(disk: &Vec<Chunk>, index: usize) -> bool {
    for chunk in &disk[index..] {
        match chunk {
            Chunk::Free { size } => (),
            Chunk::File { size, id } => { return false }
        }
    }
    true
}

fn print(disk: &Vec<Chunk>) {
    println!("");
    for chunk in disk {
        match chunk {
            Chunk::Free { size } => {
                for _ in (0..*size) { print!(". ") }
            }
            Chunk::File { size, id } => {
                for _ in (0..*size) { print!("{:?} ", id) }
            }
        }
    }
    println!("");
    println!("");
}

fn checksum(disk: Vec<Chunk>) -> i64 {
    let mut checksum = 0;
    let mut ptr = 0_i64;
    for chunk in disk {
        match chunk {
            Chunk::Free { size } => {
                ptr += size as i64;
            }
            Chunk::File { size, id } => {
                // checksum is calculated from each block in the chunk
                for i in (0..size) {
                    checksum += ((ptr as i64) + i as i64) * (id as i64);
                }
                ptr += size as i64;
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 1928);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 6448989155953);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 2858);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 6476642796832);
    }
}
