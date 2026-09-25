use crate::lib::elf_header::reader::{read_u16,read_u32} ;
use std::io::Read;
use crate::lib::elf_header::ident::ElfIdent;
use std::io;
#[repr(C)]
pub struct Elf32Header {
    e_ident: ElfIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}


impl Elf32Header {
    pub fn parse<R: Read>(
        reader: &mut R,
        ident: ElfIdent,
        endian: u8
    ) -> io::Result<Self> {
        let e_type = read_u16(reader, endian)?;
        let e_machine = read_u16(reader, endian)?;
        let e_version = read_u32(reader, endian)?;
        let e_entry = read_u32(reader, endian)?;
        let e_phoff = read_u32(reader, endian)?;
        let e_shoff = read_u32(reader, endian)?;
        let e_flags = read_u32(reader, endian)?;
        let e_ehsize = read_u16(reader, endian)?;
        let e_phentsize = read_u16(reader, endian)?;
        let e_phnum = read_u16(reader, endian)?;
        let e_shentsize = read_u16(reader, endian)?;
        let e_shnum = read_u16(reader, endian)?;
        let e_shstrndx = read_u16(reader, endian)?;

        Ok(Self {
            e_ident: ident,
            e_type,
            e_machine,

            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,

            e_shentsize,
            e_shnum,
            e_shstrndx,
        })
    }
}

