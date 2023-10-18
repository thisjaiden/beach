use crate::platform::generic::Platform;
use crate::generator::generic::AssemblyGenerator;

pub mod generic;
mod macos_aarch64;

pub fn get_all_platforms() -> Vec<Platform<impl AssemblyGenerator>> {
    vec![
        macos_aarch64::get_this()
    ]
}
