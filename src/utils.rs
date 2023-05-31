use std::io::Read;

/// Reads `num_bytes` from a [Read] source, returning the output in a new [Vec].
// TODO: Errors, example usage
pub fn read_n_bytes<R: Read>(reader: &mut R, num_bytes: usize) -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer = vec![0x00; num_bytes];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}
