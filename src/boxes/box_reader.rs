use std::io::{BufRead, Seek};
use super::error;

pub struct BoxReader<T: BufRead> {
    reader: T,
}

impl<T : BufRead> BoxReader<T> {
    pub fn create(reader: T) -> BoxReader<T> {
        Self { reader }
    }

    /// Read the next N bytes as an utf8 string.
    /// TODO ISOBMFF strings always seem to be in ASCII.
    /// Here I'm left with a dilemma:
    ///   - should I return an error if the most significant bit is set to `1`
    ///     (considering ASCII codes are 7 bits only)
    ///   - should I ignore it and just consider the other bits
    /// For now, we parse it as if it was UTF-8 which may be compatible, but seems
    /// overkill. Maybe a better solution can be found.
    pub fn read_str(&mut self, nb_bytes : usize) -> Result<String, error::ReadStrError> {
        let mut buffer = vec![0; nb_bytes];
        self.reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }

    /// Get the next four bytes as an i32.
    pub fn read_i32(&mut self) -> Result<i32, std::io::Error> {
        let mut buffer = [0; 4];
        self.reader.read_exact(&mut buffer)?;
        Ok(i32::from_be_bytes(buffer))
    }

    /// Get the next four bytes as an u32.
    pub fn read_u32(&mut self) -> Result<u32, std::io::Error> {
        let mut buffer = [0; 4];
        self.reader.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    /// Get the next eight bytes as an u64.
    pub fn read_u64(&mut self) -> Result<u64, std::io::Error> {
        let mut buffer = [0; 8];
        self.reader.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    /// Get the next two bytes as an u16.
    pub fn read_u16(&mut self) -> Result<u16, std::io::Error> {
        let mut buffer = [0; 2];
        self.reader.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Get the next byte.
    pub fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buffer = [0; 1];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

//     /// Return the next N bytes as a slice of u8.
//     pub fn read_bytes(&mut self, nb_bytes : usize) -> Result<Vec<u8>, std::io::Error> {
//         let mut buffer = vec![0; nb_bytes];
//         self.reader.read_exact(&mut buffer)?;
//         Ok(buffer)
//     }

    pub fn is_empty(&mut self) -> Result<bool, std::io::Error> {
        Ok(self.reader.fill_buf()?.is_empty())
    }

    pub fn read_to_end(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![];
        self.reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

impl<T : BufRead + Seek> BoxReader<T> {
    pub fn skip_bytes(&mut self, nb_bytes: u64) -> Result<(), std::io::Error> {
        let pos = self.get_pos()?;
        self.reader.seek(std::io::SeekFrom::Start(pos + nb_bytes))?;
        Ok(())
    }

    pub fn get_pos(&mut self) -> Result<u64, std::io::Error> {
        self.reader.seek(std::io::SeekFrom::Current(0))
    }
}
