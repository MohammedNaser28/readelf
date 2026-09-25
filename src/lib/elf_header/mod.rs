use std::io;
use std::io::Read;
use crate::lib::elf_header::ident::ElfIdent;
use crate::lib::consts::{ELFCLASS32, ELFCLASS64};
use crate::lib::elf_header::{elf32::Elf32Header, elf64::Elf64Header};
mod ident;
mod elf32;
mod elf64;
mod reader;

pub fn parser<R: Read >(reader: &mut R) -> io::Result<()> {
    let mut e_ident = [0u8; 16];
    reader.read_exact(&mut e_ident)? ;
    let ident = ElfIdent { bytes: e_ident };
    assert!(ident.check_magic());
    let class = ident.get_class();
    let endian = ident.get_endian();
    match class {
        ELFCLASS32 => {
            Elf32Header::parse(reader,ident,endian)?;
        }
        ELFCLASS64 => {
            Elf64Header::parse(reader,ident,endian)?;
        }
        _ => { }
    }
    Ok(())
}