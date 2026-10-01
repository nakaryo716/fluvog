use std::io;

const OFFSET_SIZE: u32 = 8;
const KEY_LEN_SIZE: u32 = 4;
const VALUE_LEN_SIZE: u32 = 4;
const META_SIZE: u32 = OFFSET_SIZE + KEY_LEN_SIZE + VALUE_LEN_SIZE;

#[derive(Debug, Clone)]
pub struct Record {
    following_len: u32,
    meta: RecordMeta,
    entry: Entry,
}

#[derive(Debug, Clone)]
struct RecordMeta {
    offset: u64,
    key_len: u32,
    value_len: u32,
}

#[derive(Debug, Clone)]
struct Entry {
    key: Vec<u8>,
    value: Vec<u8>,
}

// ===== impl Record =====
impl Record {
    pub fn new(offset: u64, key: Vec<u8>, value: Vec<u8>) -> Self {
        let key_len = key.len() as u32;
        let value_len = value.len() as u32;

        let meta = RecordMeta {
            offset,
            key_len,
            value_len,
        };
        let entry = Entry { key, value };

        let following_len = META_SIZE + key_len + value_len;

        Record {
            following_len,
            meta,
            entry,
        }
    }

    pub fn write(&self, dest: &mut impl io::Write) -> io::Result<()> {
        dest.write_all(&self.following_len.to_be_bytes())?;
        dest.write_all(&self.meta.offset.to_be_bytes())?;

        dest.write_all(&self.entry.key)?;
        dest.write_all(&self.meta.key_len.to_be_bytes())?;

        dest.write_all(&self.entry.value)?;
        dest.write_all(&self.meta.value_len.to_be_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::Record;

    #[test]
    fn record_following_len() {
        let key = "key";
        let value = "value";
        let record = Record::new(0, key.as_bytes().to_vec(), value.as_bytes().to_vec());

        assert_eq!(record.following_len, 24);
    }

    #[test]
    fn record_following_len_write() {
        let key = "key";
        let value = "value";
        let record = Record::new(0, key.as_bytes().to_vec(), value.as_bytes().to_vec());

        let mut buf = Cursor::new(Vec::new());
        record.write(&mut buf).unwrap();

        let total_size = buf.into_inner().len() as u32;
        // following_len type is u32, so 4 byte
        let following_len_field_size = 4;

        assert_eq!(record.following_len, total_size - following_len_field_size);
    }
}
