use crate::generator::generic::AssemblyGenerator;

/// Represents a platform and the things it supports.
pub struct Platform<G: AssemblyGenerator> {
    /// A valid generator for assembly for this platform.
    pub assembly_generator: G,
    /// The width of pointers on this platform, in bytes.
    pub pointer_width: u8,
    /// The minimum memory size on this platform, in bytes.
    pub memory_size_minimum: u128,
    /// The maximum memory size on this platform, in bytes.
    pub memory_size_maximum: u128,
    /// A name for this platform your grandparents could use.
    pub friendly_name: &'static str,
    /// A name for this platform that makes it distinct from others.
    pub technical_name: &'static str,
    /// A list of features supported by this platform, and the data to include.
    pub features: Vec<(&'static str, &'static str)>,
}

impl <G: AssemblyGenerator>Platform<G> {
    pub fn generate_assembly(&self, program: crate::parser::beach::Executable) -> String {
        let mut master_output = String::new();
        for section in &program.code_sections {
            master_output += &G::label(section.label.clone());
            for task in &section.tasks {
                master_output += &task.call_generator::<G>();
            }
        }
        for data in &program.data {
            if let Some(def_val) = &data.default {
                assert_eq!(data.size, def_val.len());
                master_output += &G::data(data.label.clone(), def_val);
            }
            else {
                master_output += &G::data(data.label.clone(), &vec![0; data.size]);
            }
        }
        for requirement in &program.platform_requirements {
            let mut found = false;
            for (feature, data) in &self.features {
                if requirement == feature {
                    master_output += data;
                    found = true;
                }
            }
            assert!(found);
        }
        println!("master pt1: \n{}", master_output);
        todo!();
    }
}
