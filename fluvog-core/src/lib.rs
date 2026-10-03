use std::io;

const OFFSET_SIZE: u32 = 8;
const KEY_LEN_SIZE: u32 = 4;
const VALUE_LEN_SIZE: u32 = 4;
const META_SIZE: u32 = OFFSET_SIZE + KEY_LEN_SIZE + VALUE_LEN_SIZE;

const MAX_TOPIC_NAME_LEN: usize = 249;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic(String);

pub enum TopicError {
    InvalidLength,
    InvalidChars,
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

// ===== impl Topic =====
impl Topic {
    pub fn new(name: impl Into<String>) -> Result<Self, TopicError> {
        let name = name.into();

        if name.len() > MAX_TOPIC_NAME_LEN {
            return Err(TopicError::InvalidLength);
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
        {
            return Err(TopicError::InvalidChars);
        }
        Ok(Self(name))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{Record, Topic};

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

    #[test]
    fn topic_len() {
        assert!(Topic::new("name").is_ok());

        // 97u8 == 'a' (ASCII)
        let invalid_topic_name = String::from_utf8_lossy(&[97u8; 250]);
        assert!(Topic::new(invalid_topic_name).is_err());
    }

    #[test]
    fn topic_char() {
        assert!(Topic::new("name_").is_ok());
        assert!(Topic::new("name-").is_ok());
        assert!(Topic::new("name.").is_ok());

        assert!(Topic::new("name/").is_err());
        assert!(Topic::new("name+").is_err());
        assert!(Topic::new("name)").is_err());
    }
}
