use crate::lib::consts::*;
pub struct ElfIdent {
    pub bytes: [u8; 16],
}

impl ElfIdent {
    pub fn check_magic(&self)-> bool {
        self.bytes[EI_MAG0] == ELFMAG0
            && self.bytes[EI_MAG1] == ELFMAG1
            && self.bytes[EI_MAG2] == ELFMAG2
            && self.bytes[EI_MAG3] == ELFMAG3
}
    
    pub fn get_magic(&self) -> [u8;16] {
        self.bytes
    }
    pub fn get_class_name(&self) -> &'static str {
        match self.bytes[EI_CLASS]{
            ELFCLASSNONE => "none",
            ELFCLASS32 => "ELF32",
            ELFCLASS64 => "ELF64",
            _ => "unknown",
        }
    }
    pub fn get_data_encoding(&self) -> &'static str {
        match self.bytes[EI_DATA] {
            ELFDATANONE => "none",
            ELFDATA2LSB => "2's complement, little endian",
            ELFDATA2MSB => "2's complement, big endian",
            _ => "unknown",
        }
    }
    pub fn get_elf_version(&self) -> &'static str {
        match self.bytes[EI_VERSION] {
            EV_CURRENT => "1 (current)",
            EV_NONE => "0",
            _ => "unknown",
        }
    }

    pub fn get_osabi_name(&self) -> &'static str {
        match self.bytes[EI_OSABI] {
            ELFOSABI_NONE => "UNIX - System V",
            ELFOSABI_HPUX =>"UNIX - HP-UX",
            ELFOSABI_NETBSD => "UNIX - NETBSD",
            ELFOSABI_GNU => "UNIX - GNU",
            ELFOSABI_SOLARIS => "UNIX - Solaris",
            ELFOSABI_AIX => "UNIX - AIX",
            ELFOSABI_IRIX => "UNIX - IRIX",
            ELFOSABI_FREEBSD => "UNIX - FreeBSD",
            ELFOSABI_TRU64 => "UNIX - Tru64",
            ELFOSABI_MODESTO => "UNIX - Modesto",
            ELFOSABI_OPENBSD => "UNIX - OpenBSD",
            ELFOSABI_OPENVMS =>"UNIX - OpenVMS",
            ELFOSABI_NSK =>"UNIX - NSK",
            ELFOSABI_AROS => "UNIX - AOS",
            ELFOSABI_FENIXOS => "UNIX - FENIXOS",
            ELFOSABI_CLOUDABI => "UNIX - CloudABI",
            ELFOSABI_OPENVOS =>"UNIX - OpenVOS",
            _ => "unknown",
        }
    }

    pub fn get_abi_version(&self) -> u8 {
        self.bytes[EI_ABIVERSION]
    }

}