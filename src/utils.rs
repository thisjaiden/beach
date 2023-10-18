use std::io::Read;

use crate::parser::beach::RESERVED_LABEL_SYMBOLS;

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
    pub fn read_char(&mut self) -> Option<char> {
        if let Ok(arr_loc) = self.char_data.binary_search_by_key(&self.location, |&(a, b)| a) {
            let tmp = self.char_data.get(arr_loc + 1);
            if let Some(dta) = tmp {
                self.location = dta.0;
            }
            else {
                // end of buffer!
                self.location = usize::MAX;
            }
            return Some(self.char_data[arr_loc].1);
        }
        else {
            None
        }
    }
    pub fn peek_char(&self) -> Option<char> {
        if let Ok(arr_loc) = self.char_data.binary_search_by_key(&self.location, |&(a, b)| a) {
            return Some(self.char_data[arr_loc].1);
        }
        else {
            None
        }
    }
    pub fn peek_word(&self) -> String {
        let mut word = String::new();
        if let Ok(arr_loc) = self.char_data.binary_search_by_key(&self.location, |&(a, b)| a) {
            let mut idx = 0;
            loop {
                let loc_res = self.char_data.get(arr_loc + idx);
                if let Some((_stridx, character)) = loc_res {
                    if character.is_whitespace() || RESERVED_LABEL_SYMBOLS.contains(character) {
                        break;
                    }
                    else {
                        word += &character.to_string();
                    }
                }
                else {
                    break;
                }
                idx += 1;
            }
        }
        word
    }
    pub fn read_word(&mut self) -> String {
        let mut word = String::new();
        if let Ok(arr_loc) = self.char_data.binary_search_by_key(&self.location, |&(a, b)| a) {
            let mut idx = 0;
            loop {
                let loc_res = self.char_data.get(arr_loc + idx);
                if let Some((_stridx, character)) = loc_res {
                    if character.is_whitespace() || RESERVED_LABEL_SYMBOLS.contains(character) {
                        self.location = self.char_data[arr_loc + idx].0;
                        break;
                    }
                    else {
                        word += &character.to_string();
                    }
                }
                else {
                    break;
                }
                idx += 1;
            }
        }
        word
    }
    pub fn read_line(&mut self) -> String {
        let mut output = String::new();
        loop {
            let this_char = self.read_char();
            if this_char == Some('\r') {
                self.read_char();
                break;
            }
            else if this_char == Some('\n') {
                break;
            }
            else if this_char == None {
                break;
            }
            else {
                output += &this_char.unwrap().to_string();
            }
        }
        output
    }
    /// Returned [String] does *not* include the [char] it terminates at.
    pub fn read_until(&mut self, watch_for: char) -> String {
        let mut output = String::new();
        loop {
            let this_char = self.read_char();
            if this_char == Some(watch_for) {
                break;
            }
            else if this_char == None {
                panic!("buffer ended before `watch_for`");
            }
            else {
                output += &this_char.unwrap().to_string();
            }
        }
        output
    }
    pub fn next_non_whitespace_char(&mut self) -> Option<char> {
        loop {
            let this_char = self.read_char()?;
            if !this_char.is_whitespace() {
                return Some(this_char);
            }
        }
    }
}

/// TODO
#[derive(Debug, PartialEq, Clone)]
pub struct Bigint {
    /// true = number negative
    pub sign: bool,
    // later in the array is bigger
    bytes: Vec<u8>
}

impl std::fmt::Display for Bigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign {
            write!(f, "-")?;
        }
        todo!();
        write!(f, "")
    }
}

impl Bigint {
    pub fn from_u8(input: u8) -> Bigint {
        Bigint {
            sign: false,
            bytes: vec![input]
        }
    }
    pub fn from_i8(input: i8) -> Bigint {
        if input.is_negative() {
            Bigint {
                sign: true,
                bytes: vec![(input * -1) as u8]
            }
        }
        else {
            Bigint {
                sign: false,
                bytes: vec![input as u8]
            }
        }
    }
    pub fn to_u8(&self) -> Result<u8, anyhow::Error> {
        if self.bit_width() <= 8 && !self.sign {
            return Ok(self.bytes[0]);
        }
        else if !self.sign {
            return Err(anyhow::Error::msg(
                "Bigint too large to convert to u8 (> 255)"
            ));
        }
        else {
            return Err(anyhow::Error::msg(
                "Bigint is negative, which can't be stored in a u8"
            ));
        }
    }
    pub fn to_i8(&self) -> Result<i8, anyhow::Error> {
        if self.bit_width() <= 7 {
            let mut as_i8 = self.bytes[0] as i8;
            if self.sign {
                as_i8 *= -1;
            }
            return Ok(as_i8);
        }
        else {
            return Err(anyhow::Error::msg(
                "Bigint too large to convert to i8 (> 127 or < -128)"
            ));
        }
    }
    pub fn bit_width(&self) -> usize {
        highest_bit(*self.bytes.last().unwrap()) + (8 * (self.bytes.len() - 1))
    }
}

fn highest_bit(input: u8) -> usize {
    let mut highest = 0;
    for i in 0..8 {
        if input << i & 0b00000001 == 0b00000001 {
            highest = i;
        }
    }
    return highest;
}

/// TODO
#[derive(Debug, PartialEq, Clone)]
pub struct Bigfloat {

}

/// TODO
#[derive(Debug, PartialEq, Clone)]
pub struct Bigcplx {
    pub real: Bigfloat,
    pub immaginary: Bigfloat,
}
