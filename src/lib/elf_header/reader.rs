use std::io;
use std::io::Read;
use crate::lib::consts::{ELFDATA2LSB,ELFDATA2MSB};

pub fn read_u16<R: Read>(
    reader: &mut R,
    endian: u8,
) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;

    Ok(match endian {
        ELFDATA2LSB => u16::from_le_bytes(buf),
        ELFDATA2MSB => u16::from_be_bytes(buf),
        _ => unreachable!(),
    })
}

pub fn read_u32<R: Read>(
    reader: &mut R,
    endian: u8,
) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;

    Ok(match endian {
        ELFDATA2LSB => u32::from_le_bytes(buf),
        ELFDATA2MSB => u32::from_be_bytes(buf),
        _ => unreachable!(),
    })
}

pub fn read_u64<R: Read>(
    reader: &mut R,
    endian: u8,
) -> io::Result<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;

    Ok(match endian {
        ELFDATA2LSB => u64::from_le_bytes(buf),
        ELFDATA2MSB => u64::from_be_bytes(buf),
        _ => unreachable!(),
    })
}