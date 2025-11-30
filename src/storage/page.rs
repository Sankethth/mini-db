use std::fmt;

/// Fixed size for each page on disk/in memory.
pub const PAGE_SIZE: usize = 4096;

/// Logical identifier for a page.
pub type PageId = u64;

/// Errors that can occur in the storage layer.
#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    InvalidPageSize { expected: usize, got: usize },
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::Io(err)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "I/O error: {e}"),
            StorageError::InvalidPageSize { expected, got } => {
                write!(f, "invalid page size: expected {expected}, got {got}")
            }
        }
    }
}

impl std::error::Error for StorageError {}

/// In-memory representation of a page.
pub struct Page {
    pub id: PageId,
    pub data: [u8; PAGE_SIZE],
}

impl Page {
    /// Create a zeroed page with the given id.
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            data: [0u8; PAGE_SIZE],
        }
    }

    /// Reset the page contents to zero.
    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_page_is_zeroed() {
        let page = Page::new(5);
        assert_eq!(page.id, 5);
        assert!(page.data.iter().all(|b| *b == 0));
    }

    #[test]
    fn clear_resets_data() {
        let mut page = Page::new(1);
        page.data[0] = 42;
        page.clear();
        assert!(page.data.iter().all(|b| *b == 0));
    }
}
