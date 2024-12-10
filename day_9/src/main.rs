#[derive(Clone, Copy)]
enum MemSlot {
    File(u64, u64), // ? (id, size)
    FreeSpace(u64), // ? (size)
}
impl MemSlot {
    fn id(&self) -> u64 {
        match self {
            MemSlot::File(id, _) => *id,
            MemSlot::FreeSpace(_) => 0,
        }
    }
    fn size(&self) -> u64 {
        match self {
            MemSlot::File(_, size) => *size,
            MemSlot::FreeSpace(size) => *size,
        }
    }
}
impl std::fmt::Debug for MemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemSlot::File(id, size) => {
                // Write size times the id number
                let mut s: String = String::new();
                for _ in 0..*size {
                    s.push_str(&id.to_string());
                }
                write!(f, "{}", s)
            }
            MemSlot::FreeSpace(size) => {
                let mut s: String = String::new();
                for _ in 0..*size {
                    s.push('.');
                }
                write!(f, "{}", s)
            }
        }
    }
}
type Memory = Vec<MemSlot>;

fn load_data(filename: &str) -> Memory {
    let data: String = std::fs::read_to_string(filename).expect("Failed to read file!");

    let mut memory: Vec<MemSlot> = Vec::new();
    let mut id = 0;
    for (idx, block) in data.chars().enumerate() {
        if idx % 2 == 0 {
            if block.to_digit(10).unwrap() == 0 {
                continue;
            }
            memory.push(MemSlot::File(id, block.to_digit(10).unwrap() as u64));
            id += 1;
        } else {
            if block.to_digit(10).unwrap() == 0 {
                continue;
            }
            memory.push(MemSlot::FreeSpace(block.to_digit(10).unwrap() as u64));
        }
    }

    memory
}

fn get_rightmost_file(memory: &Memory) -> Option<usize> {
    for idx in (0..memory.len()).rev() {
        if let MemSlot::File(_, _) = memory[idx] {
            return Some(idx);
        }
    }
    None
}

fn get_leftmost_free_space(memory: &Memory) -> Option<usize> {
    for idx in 0..memory.len() {
        if let MemSlot::FreeSpace(_) = memory[idx] {
            return Some(idx);
        }
    }
    None
}

fn is_defragged(memory: &Memory) -> bool {
    let mut file_count: usize = 0;
    for slot in memory {
        match slot {
            MemSlot::File(_, _) => file_count += 1,
            MemSlot::FreeSpace(_) => continue,
        }
    }
    // if we can cut the array in two, and the first half is all files and the second half is all free spaces, then it's defragged.
    let first_half: &[MemSlot] = &memory[0..file_count];
    let second_half: &[MemSlot] = &memory[file_count..];
    let first_half_is_files: bool = first_half.iter().all(|slot| match slot {
        MemSlot::File(_, _) => true,
        _ => false,
    });
    let second_half_is_free_spaces: bool = second_half.iter().all(|slot| match slot {
        MemSlot::FreeSpace(_) => true,
        _ => false,
    });

    first_half_is_files && second_half_is_free_spaces
}

fn part_one(memory: &Memory) -> u64 {
    let mut new_memory: Vec<MemSlot> = memory.clone();
    while !is_defragged(&new_memory) {
        let rightmost_file_idx: usize =
            get_rightmost_file(&new_memory).expect("No files found in memory!");
        let rightmost_file: MemSlot = new_memory[rightmost_file_idx].clone();

        let freespace: usize =
            get_leftmost_free_space(&new_memory).expect("No free spaces found in memory!");
        let leftmost_freespace: MemSlot = new_memory[freespace].clone();

        if rightmost_file.size() > leftmost_freespace.size() {
            let new_file_size: u64 = rightmost_file.size() - leftmost_freespace.size();
            new_memory[freespace] = MemSlot::File(rightmost_file.id(), leftmost_freespace.size());
            new_memory[rightmost_file_idx] = MemSlot::File(rightmost_file.id(), new_file_size);
        } else {
            let new_freespace_size: u64 = leftmost_freespace.size() - rightmost_file.size();
            new_memory[freespace] = MemSlot::File(rightmost_file.id(), rightmost_file.size());
            if new_freespace_size > 0 {
                new_memory.insert(freespace + 1, MemSlot::FreeSpace(new_freespace_size));
                new_memory[rightmost_file_idx + 1] = MemSlot::FreeSpace(rightmost_file.size());
            } else {
                new_memory[rightmost_file_idx] = MemSlot::FreeSpace(rightmost_file.size());
            }
        }
    }

    let mut position_counter: u64 = 0;
    let mut sum: u64 = 0;
    for slot in new_memory {
        match slot {
            MemSlot::File(_, _) => {
                for _ in 0..slot.size() {
                    sum += position_counter * slot.id();
                    position_counter += 1;
                }
            }
            _ => continue,
        }
    }

    sum
}

fn part_two(memory: &Memory) -> u64 {
    let mut new_memory: Vec<MemSlot> = memory.clone();
    let mut greatest_file_id: u64 = new_memory.iter().max_by_key(|slot| slot.id()).unwrap().id();
    while greatest_file_id != 0 {
        // ? Get file with ID = greatest_file_id
        let rightmost_file_idx: usize = new_memory
            .iter()
            .position(|slot| slot.id() == greatest_file_id)
            .expect("No files found in memory!");

        // ? Get the leftmost free space big enough to fit the rightmost file
        let freespace: usize = new_memory
            .iter()
            .position(|slot| match slot {
                MemSlot::FreeSpace(size) => *size >= new_memory[rightmost_file_idx].size(),
                _ => false,
            })
            .unwrap_or(rightmost_file_idx);

        if freespace >= rightmost_file_idx {
            greatest_file_id -= 1;
            continue;
        }

        let rightmost_file: MemSlot = new_memory[rightmost_file_idx].clone();
        let leftmost_freespace: MemSlot = new_memory[freespace].clone();

        let new_freespace_size: u64 = leftmost_freespace.size() - rightmost_file.size();
        new_memory[freespace] = MemSlot::File(rightmost_file.id(), rightmost_file.size());
        if new_freespace_size > 0 {
            new_memory.insert(freespace + 1, MemSlot::FreeSpace(new_freespace_size));
            new_memory[rightmost_file_idx + 1] = MemSlot::FreeSpace(rightmost_file.size());
        } else {
            new_memory[rightmost_file_idx] = MemSlot::FreeSpace(rightmost_file.size());
        }

        greatest_file_id -= 1;
    }

    let mut position_counter: u64 = 0;
    let mut sum: u64 = 0;
    for slot in new_memory {
        match slot {
            MemSlot::File(_, _) => {
                for _ in 0..slot.size() {
                    sum += position_counter * slot.id();
                    position_counter += 1;
                }
            }
            _ => {
                position_counter += slot.size();
            }
        }
    }

    sum
}

fn main() {
    let memory: Vec<MemSlot> = load_data("input.txt");
    println!("Part one: {}", part_one(&memory));
    println!("Part two: {}", part_two(&memory));
}
