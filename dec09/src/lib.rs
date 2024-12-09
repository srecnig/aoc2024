#[derive(Debug)]
pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn new(disk_map: &str) -> Disk {
        let mut blocks: Vec<Block> = Vec::new();
        let mut file_number: u32 = 0;
        for (i, char) in disk_map.chars().enumerate() {
            let count: u32 = char.to_digit(10).unwrap();
            let block: Block = if i % 2 == 0 {
                let content: Vec<u32> = vec![file_number; count as usize];
                let block = Block {
                    size: count,
                    content,
                };
                file_number += 1;
                block
            } else {
                let content: Vec<u32> = vec![];
                Block {
                    size: count,
                    content,
                }
            };

            blocks.push(block);
        }
        Disk { blocks }
    }

    pub fn compact(&mut self) {
        let mut block_index_front: usize = 0;
        let mut block_index_back: usize = self.blocks.len() - 1;

        loop {
            // we're done if we ended up in the same block
            if block_index_back == block_index_front {
                break;
            }

            if self.blocks[block_index_back].content.is_empty() {
                // last block is empty, go to the next.
                block_index_back -= 1;
                continue;
            }

            if self.blocks[block_index_front].content.len()
                == self.blocks[block_index_front].size as usize
            {
                // this block is already full, go to the next
                block_index_front += 1;
                continue;
            }

            // there's a last element, and space. so let's go for it.
            let last_element_index = self.blocks[block_index_back].content.len() - 1;
            let last_element: Vec<u32> = self.blocks[block_index_back]
                .content
                .drain(last_element_index..)
                .collect();
            self.blocks[block_index_front].content.push(last_element[0]);
        }
    }

    pub fn checksum(&self) -> i64 {
        let mut checksum: i64 = 0;
        let mut global_index: u32 = 0;
        for block in self.blocks.iter() {
            for part in block.content.iter() {
                checksum += (part * global_index) as i64;
                global_index += 1;
            }
        }
        checksum
    }
}

#[derive(Debug)]
struct Block {
    size: u32,
    content: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_blocks() {
        let disk = Disk::new("12345");
        assert_eq!(5, disk.blocks.len());
        assert_eq!(1, disk.blocks[0].size);
        assert_eq!(0, disk.blocks[0].content[0]);
        assert_eq!(2, disk.blocks[1].size);
        assert_eq!(0, disk.blocks[1].content.len());
        assert_eq!(3, disk.blocks[2].size);
        assert_eq!(1, disk.blocks[2].content[0]);
        assert_eq!(4, disk.blocks[3].size);
        assert_eq!(0, disk.blocks[3].content.len());
        assert_eq!(5, disk.blocks[4].size);
        assert_eq!(2, disk.blocks[4].content[0]);
    }

    #[test]
    fn can_do_simple_compact() {
        let mut disk: Disk = Disk::new("234");
        disk.compact();
        assert_eq!(vec![0, 0], disk.blocks[0].content);
        assert_eq!(vec![1, 1, 1], disk.blocks[1].content);
        assert_eq!(vec![1], disk.blocks[2].content);
    }

    #[test]
    fn can_do_another_compact() {
        let mut disk: Disk = Disk::new("12345");
        disk.compact();
        assert_eq!(vec![0], disk.blocks[0].content);
        assert_eq!(vec![2, 2], disk.blocks[1].content);
        assert_eq!(vec![1, 1, 1], disk.blocks[2].content);
        assert_eq!(vec![2, 2, 2], disk.blocks[3].content);
        assert_eq!(0, disk.blocks[4].content.len());
    }

    #[test]
    fn can_do_last_compact() {
        let mut disk: Disk = Disk::new("2333133121414131402");
        disk.compact();
        assert_eq!(vec![0, 0], disk.blocks[0].content);
        assert_eq!(vec![9, 9, 8], disk.blocks[1].content);
        assert_eq!(vec![1, 1, 1], disk.blocks[2].content);
        assert_eq!(vec![8, 8, 8], disk.blocks[3].content);
        assert_eq!(vec![2], disk.blocks[4].content);
        assert_eq!(vec![7, 7, 7], disk.blocks[5].content);
        assert_eq!(vec![3, 3, 3], disk.blocks[6].content);
        assert_eq!(vec![6], disk.blocks[7].content);
        assert_eq!(vec![4, 4], disk.blocks[8].content);
        assert_eq!(vec![6], disk.blocks[9].content);
        assert_eq!(vec![5, 5, 5, 5], disk.blocks[10].content);
        assert_eq!(vec![6], disk.blocks[11].content);
        assert_eq!(vec![6], disk.blocks[12].content);
        assert_eq!(1928, disk.checksum());
    }
}
