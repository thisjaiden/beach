use crate::{parser::beach::ast::{Definition, Task, Value}, generator::generic::{AssemblyGenerator, HardwareData}};

#[derive(Debug)]
pub struct Executable {
    pub platform_requirements: Vec<String>,
    pub data: Vec<Data>,
    pub code_sections: Vec<CodeSegment>,
    /// Section that should be run on startup
    pub entry_point: CodeSegment,
    internal_data_index: usize,
}

impl Executable {
    fn empty() -> Self {
        Executable {
            platform_requirements: vec![],
            data: vec![],
            code_sections: vec![],
            entry_point: CodeSegment::new(""),
            internal_data_index: 0,
        }
    }
    pub fn from_ast(ast: super::ast::Program) -> Self {
        let mut program = Self::empty();
        for def in ast.definitions {
            if let Definition::System { label } = def {
                program.platform_requirements.push(label.clone());
            }
        }
        for task in ast.main_tasks {
            match task {
                Task::ExitBlock => {}, //program.entry_point.add_task(GeneratableTask::EndCall),
                Task::Call { label, arguments } => {
                    for (argument_index, argument_value) in arguments.iter().enumerate() {
                        match argument_value {
                            Value::Label(label) => {
                                program.entry_point.add_task(GeneratableTask::SetCallArgument {
                                    argument_number: argument_index,
                                    argument_value: ImmediateOrRefrence::Refrence(label.clone())
                                });
                            }
                            Value::String(data) => {
                                let string_as_bytes = data.as_bytes();
                                let local_label = format!("compiler_ir_const_data_allocation_{}", program.internal_data_index);
                                program.data.push(Data {
                                    label: local_label.clone(),
                                    size: string_as_bytes.len(),
                                    default: Some(string_as_bytes.to_vec())
                                });
                                program.internal_data_index += 1;
                                program.entry_point.add_task(GeneratableTask::SetCallArgument {
                                    argument_number: argument_index,
                                    argument_value: ImmediateOrRefrence::Refrence(local_label)
                                });
                            }
                            Value::Integer(data) => {
                                if data.bit_width() <= 32 {
                                    program.entry_point.add_task(GeneratableTask::SetCallArgument {
                                        argument_number: argument_index,
                                        argument_value: ImmediateOrRefrence::Immediate(data.to_le_bytes().unwrap())
                                    });
                                }
                                else {
                                    let local_label = format!("compiler_ir_const_data_allocation_{}", program.internal_data_index);
                                    program.data.push(Data {
                                        label: local_label.clone(),
                                        size: (data.bit_width() as f32 / 8.0).floor() as usize,
                                        default: Some(data.to_le_bytes().unwrap())
                                    });
                                    program.internal_data_index += 1;
                                    program.entry_point.add_task(GeneratableTask::SetCallArgument {
                                        argument_number: argument_index,
                                        argument_value: ImmediateOrRefrence::Refrence(local_label)
                                    });
                                }
                            }
                            _ => todo!()
                        }
                    }
                    program.entry_point.add_task(GeneratableTask::Call(label));
                }
                _ => todo!()
            }
        }
        program
    }
}

#[derive(Debug)]
pub struct CodeSegment {
    pub label: String,
    pub tasks: Vec<GeneratableTask>
}

impl CodeSegment {
    pub fn new(label: &'static str) -> CodeSegment {
        CodeSegment {
            label: label.to_string(),
            tasks: vec![]
        }
    }
    fn add_task(&mut self, task: GeneratableTask) {
        self.tasks.push(task);
    }
}

#[derive(Debug)]
pub enum GeneratableTask {
    GoTo(String),
    SetCallArgument { argument_number: usize, argument_value: ImmediateOrRefrence },
    Call(String),
    EndCall,
    Add(String, String),
    Set(String),
    RequiredExtension(Vec<String>)
}

impl GeneratableTask {
    pub fn call_generator<G: AssemblyGenerator>(&self) -> String {
        match self {
            Self::SetCallArgument { argument_number, argument_value } => {
                let mut workspace = String::new();
                let reg = G::ARGUMENT_REGISTERS.get(*argument_number)
                    .expect("TODO: Too many arguments!")
                    .to_string();
                workspace += &G::push(
                    HardwareData::ImmediateRegister(reg.clone())
                );
                workspace += &G::set(
                    crate::generator::generic::HardwareData::ImmediateRegister(
                        reg
                    ),
                    argument_value.into_hardware_data()
                );
                return workspace;
            }
            Self::Call(to_call) => {
                return G::call(HardwareData::Label(to_call.clone()));
            }
            Self::EndCall => {
                return G::endcall();
            }
            _ => todo!("{:?}", self)
        }
    }
}

#[derive(Debug)]
enum ImmediateOrRefrence {
    // Try not to pass things larger than ~4 bytes as immediate arguments.
    // (basically strings and large data)
    // Immediate data must be stored in LE order if numeric.
    Immediate(Vec<u8>),
    Refrence(String)
}

impl ImmediateOrRefrence {
    pub fn into_hardware_data(&self) -> HardwareData {
        match self {
            Self::Immediate(data) => HardwareData::Immediate(data.clone()),
            Self::Refrence(ref_name) => HardwareData::Label(ref_name.clone())
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub label: String,
    pub size: usize,
    // default data is in LE order if relevant.
    pub default: Option<Vec<u8>>
}
