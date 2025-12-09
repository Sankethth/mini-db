pub struct NamePage{
    pub id : u32,
    pub name: String,
}

impl NamePage{
    pub fn to_bytes(&self,page:&mut Page) -> Result<(),StorageError>{

        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len();


        page.data[0..4].copy_from_slice(&self.id.to_le_bytes());
        page.data[4..8].copy_from_slice(str_len as u32).to_le_bytes();
        page.data[8..(name_len+8)].copy_from_slice(&self.name.as_bytes());


        Ok(())


    }

    pub fn from_bytes(page: &Page) -> Result<NamePage,StorageError>{

        let buf = &page.data;        
        let id = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        let str_len = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
        let name = String::from_utf8(buf[8..(8+str_len)].to_vec()).unwrap();

        Ok(NamePage{
            id,
            name,
        })

    }


}