pub struct LoadCommand {
    command: u32,
    command_size: u32,
    command_data: LoadCommands,
}

pub enum LoadCommands {
    SegmentLoad(SegmentLoad)
}

pub struct SegmentLoad {
    /// Name of this segment. Maximum 16 characters long.
    segment_name: String,
    /// The target virtual address of this segment.
    virtual_address: u64,
    /// The size of this segment in virtual memory. Any excess size beyond what is being copied is
    /// zeroed out.
    virtual_size: u64,
    /// Offset from the end of commands to this segment
    file_offset: u64,
    /// Size of this segment source
    file_size: u64,
    maximum_protection: u32,
    inital_protection: u32,
    number_of_sections: u32,
    flags: u32,
}
