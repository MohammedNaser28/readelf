mod consts;
mod structs;

use crate::lib::consts::*;
use std::fs::File;
use std::io::{self, BufReader, Read};
pub fn get_elf_header(file: &str) {
    let mut reader = BufReader::new(File::open(&file).unwrap());
    let mut e_ident = [0u8; 16];
    reader.read_exact(&mut e_ident);
    let mut rest = [0u8; 32];
    println!("{:02x?}", e_ident);
}

pub fn print_magic_number(e_ident: &[u8; 16]) {
    println!("Magic number: {:02x?}", e_ident);
}
pub fn get_class_name(class: u8) -> String {
    match class {
        ELFCLASSNONE => String::from("none"),
        ELFCLASS32 => String::from("ELF32"),
        ELFCLASS64 => String::from("ELF64"),
        _ => String::from("unknown"),
    }
}

pub fn get_data_encoding(data: u8) -> String {
    match data {
        ELFDATANONE => String::from("none"),
        ELFDATA2LSB => String::from("2's complement, little endian"),
        ELFDATA2MSB => String::from("2's complement, big endian"),
        _ => String::from("unknown"),
    }
}

pub fn get_elf_version(version: u8) -> String {
    match version {
        EV_CURRENT => String::from("1 (current)"),
        EV_NONE => String::from("0"),
        _ => String::from("unknown"),
    }
}

pub fn get_osabi_name(osabi: u8) -> String {
    match osabi {
        ELFOSABI_NONE => String::from("UNIX - System V"),
        ELFOSABI_HPUX => String::from("UNIX - HP-UX"),
        ELFOSABI_NETBSD => String::from("UNIX - NETBSD"),
        ELFOSABI_GNU => String::from("UNIX - GNU"),
        ELFOSABI_SOLARIS => String::from("UNIX - Solaris"),
        ELFOSABI_AIX => String::from("UNIX - AIX"),
        ELFOSABI_IRIX => String::from("UNIX - IRIX"),
        ELFOSABI_FREEBSD => String::from("UNIX - FreeBSD"),
        ELFOSABI_TRU64 => String::from("UNIX - Tru64"),
        ELFOSABI_MODESTO => String::from("UNIX - Modesto"),
        ELFOSABI_OPENBSD => String::from("UNIX - OpenBSD"),
        ELFOSABI_OPENVMS => String::from("UNIX - OpenVMS"),
        ELFOSABI_NSK => String::from("UNIX - NSK"),
        ELFOSABI_AROS => String::from("UNIX - AOS"),
        ELFOSABI_FENIXOS => String::from("UNIX - FENIXOS"),
        ELFOSABI_CLOUDABI => String::from("UNIX - CloudABI"),
        ELFOSABI_OPENVOS => String::from("UNIX - OpenVOS"),
        _ => String::from("unknown"),
    }
}

pub fn get_file_type(e_type: u16) -> String {
    match e_type {
        ET_NONE => String::from("NONE (None)"),
        ET_REL => String::from("REL (Relocatable file)"),
        ET_EXEC => String::from("EXEC (Executable file)"),
        ET_DYN => String::from("DYN (Dylib)"),
        ET_CORE => String::from("CORE (Core)"),
        _ => {
            if e_type >= ET_LOOS && e_type <= ET_HIOS {
                String::from(format!("OS Specific {}!", e_type))
            } else if e_type >= ET_LOPROC && e_type <= ET_HIPROC {
                String::from(format!("Processor Specific {:?}", e_type))
            } else {
                String::from(format!("Unknown {:?}", e_type))
            }
        }
    }
}

pub fn get_machine_name(e_machine: u16) -> String {
    match e_machine {
        EM_NONE => "None".into(),
        EM_M32 => "WE32100".into(),
        EM_SPARC => "Sparc".into(),
        EM_386 => "Intel 80386".into(),
        EM_68K => "MC68000".into(),
        EM_88K => "MC88000".into(),
        EM_IAMCU => "Intel MCU".into(),
        EM_860 => "Intel 80860".into(),
        EM_MIPS => "MIPS R3000".into(),
        EM_S370 => "IBM System/370".into(),
        EM_MIPS_RS3_LE => "MIPS R4000 big-endian".into(),
        EM_PARISC => "HPPA".into(),
        EM_VPP500 => "Fujitsu VPP500".into(),
        EM_SPARC32PLUS => "Sparc v8+".into(),
        EM_960 => "Intel 90860".into(),
        EM_PPC => "PowerPC".into(),
        EM_PPC64 => "PowerPC64".into(),
        EM_S390 => "IBM S/390".into(),
        EM_SPU => "SPU".into(),
        EM_V800 => "Renesas V850".into(),
        EM_FR20 => "Fujitsu FR20".into(),
        EM_RH32 => "TRW RH32".into(),
        EM_RCE => "MCORE".into(),
        EM_ARM => "ARM".into(),
        EM_ALPHA => "Alpha".into(),
        EM_SH => "Renesas / SuperH SH".into(),
        EM_SPARCV9 => "Sparc v9".into(),
        EM_TRICORE => "Siemens Tricore".into(),
        EM_ARC => "ARC".into(),
        EM_H8_300 => "Renesas H8/300".into(),
        EM_H8_300H => "Renesas H8/300H".into(),
        EM_H8S => "Renesas H8S".into(),
        EM_H8_500 => "Renesas H8/500".into(),
        EM_IA_64 => "Intel IA-64".into(),
        EM_MIPS_X => "Stanford MIPS-X".into(),
        EM_COLDFIRE => "Motorola ColdFire".into(),
        EM_68HC12 => "Motorola 68HC12".into(),
        EM_MMA => "Fujitsu Multimedia Accelerator".into(),
        EM_PCP => "Siemens PCP".into(),
        EM_NCPU => "NCPU".into(),
        EM_NDR1 => "NDR1".into(),
        EM_STARCORE => "StarCore".into(),
        EM_ME16 => "ME16".into(),
        EM_ST100 => "ST100".into(),
        EM_TINYJ => "TinyJ".into(),
        EM_X86_64 => "x86-64".into(),
        EM_PDSP => "PDSP".into(),
        EM_PDP10 => "PDP-10".into(),
        EM_PDP11 => "PDP-11".into(),
        EM_FX66 => "FX66".into(),
        EM_ST9PLUS => "ST9+".into(),
        EM_ST7 => "ST7".into(),
        EM_68HC16 => "Motorola 68HC16".into(),
        EM_68HC11 => "Motorola 68HC11".into(),
        EM_68HC08 => "Motorola 68HC08".into(),
        EM_68HC05 => "Motorola 68HC05".into(),
        EM_SVX => "SVx".into(),
        EM_ST19 => "ST19".into(),
        EM_VAX => "VAX".into(),
        EM_CRIS => "CRIS".into(),
        EM_JAVELIN => "Javelin".into(),
        EM_FIREPATH => "FirePath".into(),
        EM_ZSP => "ZSP".into(),
        EM_MMIX => "MMIX".into(),
        EM_HUANY => "HUANY".into(),
        EM_PRISM => "PRISM".into(),
        EM_AVR => "AVR".into(),
        EM_FR30 => "Fujitsu FR30".into(),
        EM_D10V => "Mitsubishi D10V".into(),
        EM_D30V => "Mitsubishi D30V".into(),
        EM_V850 => "Renesas V850".into(),
        EM_M32R => "Renesas M32R".into(),
        EM_MN10300 => "MN10300".into(),
        EM_MN10200 => "MN10200".into(),
        EM_PJ => "PJ".into(),
        EM_OPENRISC => "OpenRISC".into(),
        EM_ARC_COMPACT => "ARC Compact".into(),
        EM_XTENSA => "Tensilica Xtensa".into(),
        EM_VODEPCPRE => "Vodepcpre".into(),
        EM_TMM_GPP => "TMM GPP".into(),
        EM_NS32K => "National Semiconductor 32000".into(),
        EM_TPC => "TPC".into(),
        EM_SNP1K => "SNP1K".into(),
        EM_ST200 => "ST200".into(),
        EM_IP2K => "IP2K".into(),
        EM_MAX => "MAX".into(),
        EM_CR => "CR".into(),
        EM_F2MC16 => "F2MC16".into(),
        EM_MSP430 => "MSP430".into(),
        EM_BLACKFIN => "Blackfin".into(),
        EM_SE_C33 => "Seiko Epson C33".into(),
        EM_SEP => "SEP".into(),
        EM_ARCA => "ARCA".into(),
        EM_UNICORE => "UniCore".into(),
        EM_EXCESS => "EXCESS".into(),
        EM_DXP => "DXP".into(),
        EM_ALTERA_NIOS2 => "Altera Nios II".into(),
        EM_CRX => "CRX".into(),
        EM_XGATE => "XGATE".into(),
        EM_C116 => "C116".into(),
        EM_M16C => "M16C".into(),
        EM_DSPIC30F => "dsPIC30F".into(),
        EM_CE => "CE".into(),
        EM_M32C => "M32C".into(),

        EM_TSK3000 => "TSK3000".into(),
        EM_RS08 => "RS08".into(),
        EM_SHARC => "SHARC".into(),
        EM_ECOG2 => "Cyan Technology eCOG2".into(),
        EM_SCORE7 => "SUNPLUS S+core7".into(),
        EM_DSP24 => "DSP24".into(),
        EM_VIDEOCORE3 => "VideoCore3".into(),
        EM_LATTICEMICO32 => "Lattice Micro32".into(),
        EM_SE_C17 => "Seiko Epson C17 Family".into(),
        EM_TI_C6000 => "Texas Instruments TMS320C6000 DSP family".into(),
        EM_TI_C2000 => "Texas Instruments TMS320C2000 DSP family".into(),
        EM_TI_C5500 => "Texas Instruments TMS320C55x DSP family".into(),
        EM_TI_ARP32 => "Texas Instruments ARP32".into(),
        EM_TI_PRU => "TI PRU I/O processor".into(),

        EM_MMDSP_PLUS => "STMicroelectronics 64bit VLIW Data Signal Processor".into(),
        EM_CYPRESS_M8C => "Cypress M8C microprocessor".into(),
        EM_R32C => "Renesas R32C series microprocessor".into(),
        EM_TRIMEDIA => "NXP Semiconductors Trimedia architecture family".into(),
        EM_QDSP6 => "QUALCOMM DSP6 Processor".into(),
        EM_8051 => "Intel 8051 and variants".into(),
        EM_STXP7X => "STMicroelectronics STxP7x family".into(),
        EM_NDS32 => "Andes Technology compact code size embedded RISC processor family".into(),

        // EM_ECOG1 and EM_ECOG1X both have value 168.
        EM_ECOG1 => "Cyan Technology eCOG1X family".into(),

        EM_MAXQ30 => "Dakkas Semiconductor MAXQ30 Core microcontrollers".into(),
        EM_XIMO16 => "New Japan Radio (NJR) 16-bit DSP Processor".into(),
        EM_MANIK => "M2000 Reconfigurable RISC Microprocessor".into(),
        EM_CRAYNV2 => "Cray Inc. NV2 vector architecture".into(),
        EM_RX => "Renesas RX".into(),
        EM_METAG => "Imagination Technologies Meta processor architecture".into(),
        EM_MCST_ELBRUS => "MCST Elbrus general purpose hardware architecture".into(),
        EM_ECOG16 => "Cyan Technology eCOG16 family".into(),
        EM_CR16 => "CR16".into(),
        EM_ETPU => "Freescale Extended Time Processing Unit".into(),
        EM_SLE9X => "Infineon Technologies SLE9X core".into(),
        EM_L10M => "Intel L10M".into(),
        EM_K10M => "Intel K10M".into(),

        EM_AARCH64 => "AArch64".into(),

        // EM_AVR32 and EM_ARC_COMPACT2 both have value 195.
        EM_AVR32 => "Atmel Corporation 32-bit microprocessor family".into(),

        EM_STM8 => "STMicroelectronics STM8 8-bit microcontroller".into(),
        EM_TILE64 => "Tilera TILE64 multicore architecture family".into(),
        EM_TILEPRO => "Tilera TILEPro multicore architecture family".into(),
        EM_MICROBLAZE => "Xilinx MicroBlaze".into(),
        EM_CUDA => "NVIDIA CUDA architecture".into(),
        EM_TILEGX => "Tilera TILE-Gx".into(),
        EM_CLOUDSHIELD => "CloudShield".into(),
        EM_COREA_1ST => "C-Core 1st generation".into(),
        EM_COREA_2ND => "C-Core 2nd generation".into(),

        // EM_ARC_COMPACT2 has the same value as EM_AVR32 (195).
        EM_OPEN8 => "Open8".into(),
        EM_RL78 => "Renesas RL78".into(),
        EM_VIDEOCORE5 => "VideoCore5".into(),
        EM_78KOR => "Renesas 78K/OR".into(),
        EM_56800EX => "Freescale 56800EX".into(),
        EM_BA1 => "Beyond BA1".into(),
        EM_BA2 => "Beyond BA2".into(),
        EM_XCORE => "XMOS xCORE".into(),
        EM_MCHP_PIC => "Microchip PIC".into(),
        EM_INTEL205 => "Intel architecture 205".into(),
        EM_INTEL206 => "Intel architecture 206".into(),
        EM_INTEL207 => "Intel architecture 207".into(),
        EM_INTEL208 => "Intel architecture 208".into(),
        EM_INTEL209 => "Intel architecture 209".into(),
        EM_KM32 => "KM32".into(),
        EM_KMX32 => "KMX32".into(),
        EM_KMX16 => "KMX16".into(),
        EM_KMX8 => "KMX8".into(),
        EM_KVARC => "KVARC".into(),
        EM_CDP => "CDP".into(),
        EM_COGE => "COGE".into(),
        EM_COOL => "COOL".into(),
        EM_NORC => "Norc".into(),
        EM_CSR_KALIMBA => "CSR Kalimba".into(),
        EM_Z80 => "Z80".into(),
        EM_VISIUM => "Visium".into(),
        EM_FT32 => "FT32".into(),
        EM_MOXIE => "Moxie".into(),
        EM_AMDGPU => "AMD GPU".into(),
        EM_RISCV => "RISC-V".into(),

        value => format!("<unknown>: 0x{value:x}"),
    }
}

pub fn display_file_header(e_ident: &[u8; 16]) {
    println!(
        "ELF HEADER\n:\
     Magic: {:?}\n\
     Class: {:?}\n\
     Data: {:?} \n\
     Version: {:?}\n\
     OS/ABI: {:?} \n\
     ABI Version: {:?} \n\
     Type:  \n\
     Machine:  \n\
     Version: \n\
     Entry Point address: \n\
     Start of program headers: \n\
     Start of section headers: \n\
     Flags: \n\
     Size of this header: \n\
     Size of program headers: \n\
     Number of program headers: \n\
     Size of section headers: \n\
     Number of section headers: \n\
    Section header string table index: \n\
     ",
        print_magic_number(e_ident),
        get_class_name(e_ident[EI_CLASS]),
        get_data_encoding(e_ident[EI_DATA]),
        get_elf_version(e_ident[EI_VERSION]),
        get_osabi_name(e_ident[EI_OSABI]),
        e_ident[EI_ABIVERSION],
    )
}
