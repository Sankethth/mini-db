use core::num;
use std::io::{Read, Seek, Write};

use crate::{PAGE_SIZE, StorageError,Page};

pub struct DiskManager{
    file: std::fs::File,
    num_pages: u32,
}

impl DiskManager {


    pub fn new(file_path: &str) -> Result<Self,StorageError>{

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let metadata = file.metadata()?;
        let file_size = metadata.len();
        let num_pages = (file_size / PAGE_SIZE as u64) as u32;

        file.seek(std::io::SeekFrom::Start((0)));

        Ok(Self { file, num_pages })



    }

    pub fn read_page(&mut self, page_id: u64) -> Result<Page,StorageError>{


        let page_offset = page_id * PAGE_SIZE as u64;
        self.file.seek(std::io::SeekFrom::Start((page_offset as u64)))?;
        let mut buf = [0u8; PAGE_SIZE];
        self.file.read_exact(&mut buf)?;
        Ok(Page {
            id: page_id,
            data: buf,
        })


    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), StorageError> {
        let offset = (page.id as u64) * (PAGE_SIZE as u64);
        self.file.seek(std::io::SeekFrom::Start(offset))?;
        self.file.write(&page.data)?;
        self.num_pages += 1;


        Ok(())
    }
}

#[cfg(tests)]
mod tests{

}

