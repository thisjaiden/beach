pub fn read_n_bytes<'a, R: std::io::Read>(reader: &mut R, num_bytes: usize) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![0x00; num_bytes];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}
