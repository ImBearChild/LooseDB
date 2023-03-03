pub(crate) type Nonce = [u8; 12];
pub(crate) type PageId = [u8; 16];
pub(crate) type KeyId =  [u8; 16];
pub(crate) type TimeStamp = [u8; 8];

struct EncryptionHeader {
    /// Nonce for encryption
    nonce: Nonce,
    page_id: PageId,
}
struct PageHeader {
    page_id: PageId,
    /// File version of this page
    version: u8,
    /// Type of this page
    page_type: u8,
    /// Type of compression
    compression_type: u8,
    reserved: u8,
    /// Unix time stamp
    time_stamp: TimeStamp,
}

struct CaskEntry {
    entry_page_id: PageId,
    cask_name: Vec<u8>,
}

struct BtreePageHeader {
    cell_num: u32,
}

struct BtreeInteriorCell{
    key_id: KeyId,
    page_pointer: PageId,
}

struct BtreeLeafCellHeader {
    // cell_size: u32, 
    key_id: KeyId,
    payload_length: u64
    // payload: Vec<u8> 
    // overflow
}

type BtreeLeafCellOverflow = PageId;
