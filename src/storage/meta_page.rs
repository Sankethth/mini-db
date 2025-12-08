use core::num;

use crate::{Page, StorageError};
use crate::storage::TableMeta;

pub struct MetaPage{
    pub num_tables: u32,
    pub table_metas: Vec<TableMeta>,

}

impl MetaPage{

    pub fn to_bytes(&self,page:&mut Page) -> Result<(),StorageError>{

        let buf = &mut page.data;
        buf[0..4].copy_from_slice(&self.num_tables.to_le_bytes());

        Ok(())
    

        //TODO: Implement serialization of MetaPage

    }

    pub fn from_bytes(page: &Page) -> Result<MetaPage,StorageError>{

        let buf = &page.data;
        let num_tables = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        Ok(MetaPage{
            num_tables,
            table_metas: Vec::new(),
        })
        //TODO: Implement deserialization of MetaPage
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::page::Page;

    #[test]
    fn test_meta_page_in_memory_roundtrip() {
        let original = MetaPage {
            num_tables: 5,
            table_metas: vec![],
        };

        let mut page = Page::new(0);

        original.to_bytes(&mut page).unwrap();

        let reconstructed = MetaPage::from_bytes(&page).unwrap();

        assert_eq!(reconstructed.num_tables, 5);
        assert_eq!(reconstructed.table_metas.len(), 0);
    }

     #[test]
    fn test_meta_page_file() {
        let original = MetaPage {
            num_tables: 7,
            table_metas: vec![],
        };

        let path = "test_meta.db";

        let mut disk_manager = crate::storage::disk_manager::DiskManager::new(path).unwrap();

        let mut page = Page::new(0);
        original.to_bytes(&mut page).unwrap();
        disk_manager.write_page(&mut page).unwrap();

        let read_page = disk_manager.read_page(0).unwrap();

        let reconstructed = MetaPage::from_bytes(&read_page);

        assert_eq!(reconstructed.unwrap().num_tables, 7);
    }
}
