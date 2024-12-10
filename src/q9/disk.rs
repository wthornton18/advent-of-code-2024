use std::{fmt::Display, str::FromStr};

use itertools::repeat_n;

pub trait Checksum {
    fn get_data(&self) -> Vec<Option<usize>>;

    fn checksum(&self) -> usize {
        self.get_data()
            .iter()
            .enumerate()
            .filter_map(|(i, id)| id.map(|id| id * i))
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct SimpleDisk {
    data: Vec<Option<usize>>,
}

impl From<Disk> for SimpleDisk {
    fn from(disk: Disk) -> Self {
        Self {
            data: disk.construct_data(),
        }
    }
}

impl Display for SimpleDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in &self.data {
            match elem {
                Some(id) => write!(f, "{}", id)?,
                None => write!(f, ".")?,
            }
        }
        writeln!(f)
    }
}

impl Checksum for SimpleDisk {
    fn get_data(&self) -> Vec<Option<usize>> {
        self.data.clone()
    }
}

impl SimpleDisk {
    #[inline(always)]
    pub fn get_first_empty_space(&self, offset: usize) -> usize {
        self.data
            .iter()
            .skip(offset)
            .position(|id| id.is_none())
            .map(|idx| idx + offset)
            .unwrap()
    }

    pub fn maximally_compact(&mut self) {
        let mut start = self.get_first_empty_space(0);
        let mut end = self.data.len() - 1;

        while start < end {
            if self.data[end].is_some() {
                self.data.swap(start, end);
                start = self.get_first_empty_space(start);
            }
            end -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Disk {
    files: Vec<FileDescriptor>,
    empty_space: Vec<EmptySpace>,
}

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub id: usize,
    pub size: usize,
    pub pos: usize,
}

#[derive(Debug, Clone)]
pub struct EmptySpace {
    pub size: usize,
    pub pos: usize,
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut empty_space = Vec::new();

        let s = s.trim();

        let mut file = true;
        let mut id = 0;
        let mut pos = 0;

        for c in s.chars() {
            let num = c.to_digit(10).ok_or(())? as usize;
            if file {
                files.push(FileDescriptor { id, size: num, pos });
                id += 1;
            } else {
                empty_space.push(EmptySpace { pos, size: num });
            }
            file = !file;
            pos += num;
        }

        Ok(Self { files, empty_space })
    }
}

impl Disk {
    pub fn compact_no_frag(&mut self) {
        for file in self.files.iter_mut().rev() {
            for empty_space in self.empty_space.iter_mut() {
                if empty_space.pos > file.pos || empty_space.size < file.size {
                    continue;
                }
                let original_pos = empty_space.pos;
                let original_file_pos = file.pos;
                let original_file_size = file.size;

                empty_space.size -= file.size;
                empty_space.pos += file.size;
                file.pos = original_pos;

                self.empty_space.push(EmptySpace {
                    size: original_file_size,
                    pos: original_file_pos,
                });

                break;
            }
        }
    }

    pub fn construct_data(&self) -> Vec<Option<usize>> {
        pub enum FileOrEmpty<'a> {
            File(&'a FileDescriptor),
            Empty(&'a EmptySpace),
        }

        let mut files_or_empty = self
            .files
            .iter()
            .map(FileOrEmpty::File)
            .chain(self.empty_space.iter().map(FileOrEmpty::Empty))
            .collect::<Vec<_>>();

        files_or_empty.sort_by_key(|file_or_empty| match file_or_empty {
            FileOrEmpty::File(f) => f.pos,
            FileOrEmpty::Empty(e) => e.pos,
        });

        let mut data = Vec::new();

        for file_or_empty in files_or_empty {
            match file_or_empty {
                FileOrEmpty::File(f) => {
                    data.extend(repeat_n(Some(f.id), f.size));
                }
                FileOrEmpty::Empty(e) => {
                    data.extend(repeat_n(None, e.size));
                }
            }
        }

        data
    }
}

impl Checksum for Disk {
    fn get_data(&self) -> Vec<Option<usize>> {
        self.construct_data()
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in self.construct_data() {
            match elem {
                Some(id) => write!(f, "{}", id)?,
                None => write!(f, ".")?,
            }
        }
        writeln!(f)
    }
}
