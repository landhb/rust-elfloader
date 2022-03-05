use crate::{ElfLoaderErr, Machine};

pub mod x86;
pub mod x86_64;

pub mod tilegx;
pub use tilegx as TileGX;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RelocationType {
    x86(x86::RelocationTypes),
    x86_64(x86_64::RelocationTypes),
    TileGX(TileGX::RelocationTypes),
}

impl RelocationType {
    /// Match an architecture and value to a Relocation type
    pub fn from(machine: Machine, type_num: u32) -> Result<RelocationType, ElfLoaderErr> {
        let typ = match machine {
            Machine::X86 => RelocationType::x86(x86::RelocationTypes::from(type_num)),
            Machine::X86_64 => RelocationType::x86_64(x86_64::RelocationTypes::from(type_num)),
            Machine::Other(191) => RelocationType::TileGX(TileGX::RelocationTypes::from(type_num)),
            _ => return Err(ElfLoaderErr::UnsupportedArchitecture),
        };
        Ok(typ)
    }
}
