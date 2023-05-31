mod header;
mod load_command;

pub struct MachO<'a> {
    header: header::Header,
    load_commands: Vec<load_command::LoadCommand>,
    data: &'a [u8],
}

impl <'a>MachO<'a> {
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> MachO {
        todo!()
    }
    pub fn write<W: std::io::Write>(&mut self, writer: &mut W) {
        todo!()
    }
}