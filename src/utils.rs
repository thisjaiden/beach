use std::io::Read;

/// Reads `num_bytes` from a [Read] source, returning the output in a new [Vec].
// TODO: Errors, example usage
pub fn read_n_bytes<R: Read>(reader: &mut R, num_bytes: usize) -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer = vec![0x00; num_bytes];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub struct StringReader {
    string_data: String,
    location: usize,
    char_data: Vec<(usize, char)>
}

impl StringReader {
    /// Creates a new [StringReader] from a [Read] type. Invalidates the input data.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<StringReader, anyhow::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let char_indices = string.char_indices();
        let char_data = char_indices.collect();
        Ok(StringReader { string_data: string, location: 0, char_data })
    }
    pub fn from_string(string: String) -> StringReader {
        let char_indices = string.char_indices();
        let char_data = char_indices.collect();
        StringReader { string_data: string, location: 0, char_data }
    }
    pub fn read_char(&mut self) -> char {
        if let Ok(arr_loc) = self.char_data.binary_search_by_key(&self.location, |&(a, b)| a) {
            self.location = self.char_data[arr_loc + 1].0;
            return self.char_data[arr_loc].1;
        }
        else {
            todo!()
        }
    }
    pub fn read_line(&mut self) -> String {
        let mut output = String::new();
        loop {
            let this_char = self.read_char();
            if this_char == '\r' {
                self.read_char();
                break;
            }
            else if this_char == '\n' {
                break;
            }
            else {
                output += &this_char.to_string();
            }
        }
        output
    }
    /// Returned [String] does *not* include the [char] it terminates at.
    pub fn read_until(&mut self, watch_for: char) -> String {
        let mut output = String::new();
        loop {
            let this_char = self.read_char();
            if this_char == watch_for {
                break;
            }
            else {
                output += &this_char.to_string();
            }
        }
        output
    }
    pub fn next_non_whitespace_char(&mut self) -> char {
        loop {
            let this_char = self.read_char();
            if !this_char.is_whitespace() {
                return this_char
            }
        }
    }
}

/// TODO
pub struct Bigint {

}

/// TODO
pub struct Bigfloat {

}

/// TODO
pub struct Bigcplx {

}
