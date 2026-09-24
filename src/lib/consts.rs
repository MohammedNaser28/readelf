pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;

pub const ET_LOOS: u16 = 0xfe00;
pub const ET_HIOS: u16 = 0xfeff;

pub const ET_LOPROC: u16 = 0xff00;
pub const ET_HIPROC: u16 = 0xffff;
/**
 * Machine Architecture
 */

pub const EM_NONE: u16 = 0;
pub const EM_M32: u16 = 1;
pub const EM_SPARC: u16 = 2;
pub const EM_386: u16 = 3;
pub const EM_68K: u16 = 4;
pub const EM_88K: u16 = 5;
pub const EM_IAMCU: u16 = 6;
pub const EM_860: u16 = 7;
pub const EM_MIPS: u16 = 8;
pub const EM_S370: u16 = 9;
pub const EM_MIPS_RS3_LE: u16 = 10;
pub const EM_PARISC: u16 = 15;
pub const EM_VPP500: u16 = 17;
pub const EM_SPARC32PLUS: u16 = 18;
pub const EM_960: u16 = 19;
pub const EM_PPC: u16 = 20;
pub const EM_PPC64: u16 = 21;
pub const EM_S390: u16 = 22;
pub const EM_SPU: u16 = 23;
pub const EM_V800: u16 = 36;
pub const EM_FR20: u16 = 37;
pub const EM_RH32: u16 = 38;
pub const EM_RCE: u16 = 39;
pub const EM_ARM: u16 = 40;
pub const EM_ALPHA: u16 = 41;
pub const EM_SH: u16 = 42;
pub const EM_SPARCV9: u16 = 43;
pub const EM_TRICORE: u16 = 44;
pub const EM_ARC: u16 = 45;
pub const EM_H8_300: u16 = 46;
pub const EM_H8_300H: u16 = 47;
pub const EM_H8S: u16 = 48;
pub const EM_H8_500: u16 = 49;
pub const EM_IA_64: u16 = 50;
pub const EM_MIPS_X: u16 = 51;
pub const EM_COLDFIRE: u16 = 52;
pub const EM_68HC12: u16 = 53;
pub const EM_MMA: u16 = 54;
pub const EM_PCP: u16 = 55;
pub const EM_NCPU: u16 = 56;
pub const EM_NDR1: u16 = 57;
pub const EM_STARCORE: u16 = 58;
pub const EM_ME16: u16 = 59;
pub const EM_ST100: u16 = 60;
pub const EM_TINYJ: u16 = 61;
pub const EM_X86_64: u16 = 62;
pub const EM_PDSP: u16 = 63;
pub const EM_PDP10: u16 = 64;
pub const EM_PDP11: u16 = 65;
pub const EM_FX66: u16 = 66;
pub const EM_ST9PLUS: u16 = 67;
pub const EM_ST7: u16 = 68;
pub const EM_68HC16: u16 = 69;
pub const EM_68HC11: u16 = 70;
pub const EM_68HC08: u16 = 71;
pub const EM_68HC05: u16 = 72;
pub const EM_SVX: u16 = 73;
pub const EM_ST19: u16 = 74;
pub const EM_VAX: u16 = 75;
pub const EM_CRIS: u16 = 76;
pub const EM_JAVELIN: u16 = 77;
pub const EM_FIREPATH: u16 = 78;
pub const EM_ZSP: u16 = 79;
pub const EM_MMIX: u16 = 80;
pub const EM_HUANY: u16 = 81;
pub const EM_PRISM: u16 = 82;
pub const EM_AVR: u16 = 83;
pub const EM_FR30: u16 = 84;
pub const EM_D10V: u16 = 85;
pub const EM_D30V: u16 = 86;
pub const EM_V850: u16 = 87;
pub const EM_M32R: u16 = 88;
pub const EM_MN10300: u16 = 89;
pub const EM_MN10200: u16 = 90;
pub const EM_PJ: u16 = 91;
pub const EM_OPENRISC: u16 = 92;
pub const EM_ARC_COMPACT: u16 = 93;
pub const EM_XTENSA: u16 = 94;
pub const EM_VODEPCPRE: u16 = 95;
pub const EM_TMM_GPP: u16 = 96;
pub const EM_NS32K: u16 = 97;
pub const EM_TPC: u16 = 98;
pub const EM_SNP1K: u16 = 99;
pub const EM_ST200: u16 = 100;
pub const EM_IP2K: u16 = 101;
pub const EM_MAX: u16 = 102;
pub const EM_CR: u16 = 103;
pub const EM_F2MC16: u16 = 104;
pub const EM_MSP430: u16 = 105;
pub const EM_BLACKFIN: u16 = 106;
pub const EM_SE_C33: u16 = 107;
pub const EM_SEP: u16 = 108;
pub const EM_ARCA: u16 = 109;
pub const EM_UNICORE: u16 = 110;
pub const EM_EXCESS: u16 = 111;
pub const EM_DXP: u16 = 112;
pub const EM_ALTERA_NIOS2: u16 = 113;
pub const EM_CRX: u16 = 114;
pub const EM_XGATE: u16 = 115;
pub const EM_C116: u16 = 116;
pub const EM_M16C: u16 = 117;
pub const EM_DSPIC30F: u16 = 118;
pub const EM_CE: u16 = 119;
pub const EM_M32C: u16 = 120;

pub const EM_TSK3000: u16 = 131;
pub const EM_RS08: u16 = 132;
pub const EM_SHARC: u16 = 133;
pub const EM_ECOG2: u16 = 134;
pub const EM_SCORE7: u16 = 135;
pub const EM_DSP24: u16 = 136;
pub const EM_VIDEOCORE3: u16 = 137;
pub const EM_LATTICEMICO32: u16 = 138;
pub const EM_SE_C17: u16 = 139;
pub const EM_TI_C6000: u16 = 140;
pub const EM_TI_C2000: u16 = 141;
pub const EM_TI_C5500: u16 = 142;
pub const EM_TI_ARP32: u16 = 143;
pub const EM_TI_PRU: u16 = 144;

pub const EM_MMDSP_PLUS: u16 = 160;
pub const EM_CYPRESS_M8C: u16 = 161;
pub const EM_R32C: u16 = 162;
pub const EM_TRIMEDIA: u16 = 163;
pub const EM_QDSP6: u16 = 164;
pub const EM_8051: u16 = 165;
pub const EM_STXP7X: u16 = 166;
pub const EM_NDS32: u16 = 167;
pub const EM_ECOG1: u16 = 168;
pub const EM_ECOG1X: u16 = 168;
pub const EM_MAXQ30: u16 = 169;
pub const EM_XIMO16: u16 = 170;
pub const EM_MANIK: u16 = 171;
pub const EM_CRAYNV2: u16 = 172;
pub const EM_RX: u16 = 173;
pub const EM_METAG: u16 = 174;
pub const EM_MCST_ELBRUS: u16 = 175;
pub const EM_ECOG16: u16 = 176;
pub const EM_CR16: u16 = 177;
pub const EM_ETPU: u16 = 178;
pub const EM_SLE9X: u16 = 179;
pub const EM_L10M: u16 = 180;
pub const EM_K10M: u16 = 181;

pub const EM_AARCH64: u16 = 183;
pub const EM_AVR32: u16 = 195;
pub const EM_STM8: u16 = 186;
pub const EM_TILE64: u16 = 187;
pub const EM_TILEPRO: u16 = 188;
pub const EM_MICROBLAZE: u16 = 189;
pub const EM_CUDA: u16 = 190;
pub const EM_TILEGX: u16 = 191;
pub const EM_CLOUDSHIELD: u16 = 192;
pub const EM_COREA_1ST: u16 = 193;
pub const EM_COREA_2ND: u16 = 194;
pub const EM_ARC_COMPACT2: u16 = 195;
pub const EM_OPEN8: u16 = 196;
pub const EM_RL78: u16 = 197;
pub const EM_VIDEOCORE5: u16 = 198;
pub const EM_78KOR: u16 = 199;
pub const EM_56800EX: u16 = 200;
pub const EM_BA1: u16 = 201;
pub const EM_BA2: u16 = 202;
pub const EM_XCORE: u16 = 203;
pub const EM_MCHP_PIC: u16 = 204;
pub const EM_INTEL205: u16 = 205;
pub const EM_INTEL206: u16 = 206;
pub const EM_INTEL207: u16 = 207;
pub const EM_INTEL208: u16 = 208;
pub const EM_INTEL209: u16 = 209;
pub const EM_KM32: u16 = 210;
pub const EM_KMX32: u16 = 211;
pub const EM_KMX16: u16 = 212;
pub const EM_KMX8: u16 = 213;
pub const EM_KVARC: u16 = 214;
pub const EM_CDP: u16 = 215;
pub const EM_COGE: u16 = 216;
pub const EM_COOL: u16 = 217;
pub const EM_NORC: u16 = 218;
pub const EM_CSR_KALIMBA: u16 = 219;
pub const EM_Z80: u16 = 220;
pub const EM_VISIUM: u16 = 221;
pub const EM_FT32: u16 = 222;
pub const EM_MOXIE: u16 = 223;
pub const EM_AMDGPU: u16 = 224;
pub const EM_RISCV: u16 = 243;

/**
 * Object File Version
 */

pub const EV_NONE: u8 = 0;
pub const EV_CURRENT: u8 = 1;

pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_ABIVERSION: usize = 8;
pub const EI_PAD: usize = 9;

// ELF identification

pub const EI_NIDENT: usize = 16;

pub const ELFMAG0: u8 = 0x7f;
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';

// ELF class

pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;

// ELF data encoding / endianness

pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;

// ELF OS / ABI

pub const ELFOSABI_NONE: u8 = 0;
pub const ELFOSABI_HPUX: u8 = 1;
pub const ELFOSABI_NETBSD: u8 = 2;
pub const ELFOSABI_GNU: u8 = 3;
pub const ELFOSABI_LINUX: u8 = 3;
pub const ELFOSABI_SOLARIS: u8 = 6;
pub const ELFOSABI_AIX: u8 = 7;
pub const ELFOSABI_IRIX: u8 = 8;
pub const ELFOSABI_FREEBSD: u8 = 9;
pub const ELFOSABI_TRU64: u8 = 10;
pub const ELFOSABI_MODESTO: u8 = 11;
pub const ELFOSABI_OPENBSD: u8 = 12;
pub const ELFOSABI_OPENVMS: u8 = 13;
pub const ELFOSABI_NSK: u8 = 14;
pub const ELFOSABI_AROS: u8 = 15;
pub const ELFOSABI_FENIXOS: u8 = 16;
pub const ELFOSABI_CLOUDABI: u8 = 17;
pub const ELFOSABI_OPENVOS: u8 = 18;
