use libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Base64 encoding & decoding library

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
//#define DEBUG(args...)    fprintf(stderr,"debug: " args) /* diagnostic message that is destined to the user */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MODULE-WIDE VARIABLES ---------------------------------------- */
static mut code_62: libc::c_char = '+' as i32 as libc::c_char;
/* RFC 1421 standard character for code 62 */
static mut code_63: libc::c_char = '/' as i32 as libc::c_char;
/* RFC 1421 standard character for code 63 */
static mut code_pad: libc::c_char = '=' as i32 as libc::c_char;
/* RFC 1421 padding character if padding */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DECLARATION ---------------------------------------- */
/* *
@brief Convert a code in the range 0-63 to an ASCII character
*/
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn code_to_char(mut x: uint8_t) -> libc::c_char {
    if x as libc::c_int <= 25i32 {
        return ('A' as i32 + x as libc::c_int) as libc::c_char;
    } else if x as libc::c_int >= 26i32 && x as libc::c_int <= 51i32 {
        return ('a' as i32 + (x as libc::c_int - 26i32)) as libc::c_char;
    } else if x as libc::c_int >= 52i32 && x as libc::c_int <= 61i32 {
        return ('0' as i32 + (x as libc::c_int - 52i32)) as libc::c_char;
    } else if x as libc::c_int == 62i32 {
        return code_62;
    } else if x as libc::c_int == 63i32 {
        return code_63;
    } else {
        exit(1i32);
    };
    //TODO: improve error management
}
/* *
@brief Convert an ASCII character to a code in the range 0-63
*/
#[no_mangle]
pub unsafe extern "C" fn char_to_code(mut x: libc::c_char) -> uint8_t {
    if x as libc::c_int >= 'A' as i32 && x as libc::c_int <= 'Z' as i32 {
        return (x as uint8_t as libc::c_int - 'A' as i32 as uint8_t as libc::c_int) as uint8_t;
    } else if x as libc::c_int >= 'a' as i32 && x as libc::c_int <= 'z' as i32 {
        return (x as uint8_t as libc::c_int - 'a' as i32 as uint8_t as libc::c_int + 26i32)
            as uint8_t;
    } else if x as libc::c_int >= '0' as i32 && x as libc::c_int <= '9' as i32 {
        return (x as uint8_t as libc::c_int - '0' as i32 as uint8_t as libc::c_int + 52i32)
            as uint8_t;
    } else if x as libc::c_int == code_62 as libc::c_int {
        return 62i32 as uint8_t;
    } else if x as libc::c_int == code_63 as libc::c_int {
        return 63i32 as uint8_t;
    } else {
        exit(1i32);
    };
    //TODO: improve error management
}
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Base64 encoding & decoding library

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Encode binary data in Base64 string (no padding)
@param in pointer to a table of binary data
@param size number of bytes to be encoded to base64
@param out pointer to a string where the function will output encoded data
@param max_len max length of the out string (including null char)
@return >=0 length of the resulting string (w/o null char), -1 for error
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn bin_to_b64_nopad(
    mut in_0: *const uint8_t,
    mut size: libc::c_int,
    mut out: *mut libc::c_char,
    mut max_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* size of the result */
    let mut result_len: libc::c_int = 0; /* number of 3 unsigned chars / 4 characters blocks */
    let mut full_blocks: libc::c_int = 0; /* number of unsigned chars <3 in the last block */
    let mut last_bytes: libc::c_int = 0; /* number of characters <4 in the last block */
    let mut last_chars: libc::c_int = 0;
    let mut b: uint32_t = 0;
    /* check input values */
    if out.is_null() || in_0.is_null() {
        return -1i32;
    } /* null string */
    if size == 0i32 {
        *out = 0i32 as libc::c_char;
        return 0i32;
    }
    /* calculate the number of base64 'blocks' */
    full_blocks = size / 3i32;
    last_bytes = size % 3i32;
    match last_bytes {
        0 => {
            /* no byte left to encode */
            last_chars = 0i32
        }
        1 => {
            /* 1 byte left to encode -> +2 chars */
            last_chars = 2i32
        }
        2 => {
            /* 2 bytes left to encode -> +3 chars */
            last_chars = 3i32
        }
        _ => {
            fprintf(
                stderr,
                b"\nCRITICAL file:%s line:%u msg:%s\n\x00" as *const u8 as *const libc::c_char,
                b"src/base64.c\x00" as *const u8 as *const libc::c_char,
                129i32,
                b"switch default that should not be possible\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(1i32);
        }
    }
    /* check if output buffer is big enough */
    result_len = 4i32 * full_blocks + last_chars;
    if max_len < result_len + 1i32 {
        /* 1 char added for string terminator */
        return -1i32;
    }
    /* process all the full blocks */
    i = 0i32;
    while i < full_blocks {
        b = ((0xffi32 & *in_0.offset((3i32 * i) as isize) as libc::c_int) << 16i32) as uint32_t;
        b |= ((0xffi32 & *in_0.offset((3i32 * i + 1i32) as isize) as libc::c_int) << 8i32)
            as libc::c_uint;
        b |= (0xffi32 & *in_0.offset((3i32 * i + 2i32) as isize) as libc::c_int) as libc::c_uint;
        *out.offset((4i32 * i + 0i32) as isize) =
            code_to_char((b >> 18i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 1i32) as isize) =
            code_to_char((b >> 12i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 2i32) as isize) =
            code_to_char((b >> 6i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 3i32) as isize) =
            code_to_char((b & 0x3fi32 as libc::c_uint) as uint8_t);
        i += 1
    }
    /* process the last 'partial' block and terminate string */
    i = full_blocks;
    if last_chars == 0i32 {
        *out.offset((4i32 * i) as isize) = 0i32 as libc::c_char
    /* null character to terminate string */
    } else if last_chars == 2i32 {
        b = ((0xffi32 & *in_0.offset((3i32 * i) as isize) as libc::c_int) << 16i32) as uint32_t;
        *out.offset((4i32 * i + 0i32) as isize) =
            code_to_char((b >> 18i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 1i32) as isize) =
            code_to_char((b >> 12i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 2i32) as isize) = 0i32 as libc::c_char
    /* null character to terminate string */
    } else if last_chars == 3i32 {
        b = ((0xffi32 & *in_0.offset((3i32 * i) as isize) as libc::c_int) << 16i32) as uint32_t;
        b |= ((0xffi32 & *in_0.offset((3i32 * i + 1i32) as isize) as libc::c_int) << 8i32)
            as libc::c_uint;
        *out.offset((4i32 * i + 0i32) as isize) =
            code_to_char((b >> 18i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 1i32) as isize) =
            code_to_char((b >> 12i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 2i32) as isize) =
            code_to_char((b >> 6i32 & 0x3fi32 as libc::c_uint) as uint8_t);
        *out.offset((4i32 * i + 3i32) as isize) = 0i32 as libc::c_char
        /* null character to terminate string */
    }
    return result_len;
}
/* *
@brief Decode Base64 string to binary data (no padding)
@param in string containing only base64 valid characters
@param size number of characters to be decoded from base64 (w/o null char)
@param out pointer to a data buffer where the function will output decoded data
@param out_max_len usable size of the output data buffer
@return >=0 number of bytes written to the data buffer, -1 for error
*/
#[no_mangle]
pub unsafe extern "C" fn b64_to_bin_nopad(
    mut in_0: *const libc::c_char,
    mut size: libc::c_int,
    mut out: *mut uint8_t,
    mut max_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* size of the result */
    let mut result_len: libc::c_int = 0; /* number of 3 unsigned chars / 4 characters blocks */
    let mut full_blocks: libc::c_int = 0; /* number of characters <4 in the last block */
    let mut last_chars: libc::c_int = 0; /* number of unsigned chars <3 in the last block */
    let mut last_bytes: libc::c_int = 0;
    let mut b: uint32_t = 0;
    /* check input values */
    if out.is_null() || in_0.is_null() {
        return -1i32;
    }
    if size == 0i32 {
        return 0i32;
    }
    /* calculate the number of base64 'blocks' */
    full_blocks = size / 4i32;
    last_chars = size % 4i32;
    match last_chars {
        0 => {
            /* no char left to decode */
            last_bytes = 0i32
        }
        1 => {
            /* only 1 char left is an error */
            return -1i32;
        }
        2 => {
            /* 2 chars left to decode -> +1 byte */
            last_bytes = 1i32
        }
        3 => {
            /* 3 chars left to decode -> +2 bytes */
            last_bytes = 2i32
        }
        _ => {
            fprintf(
                stderr,
                b"\nCRITICAL file:%s line:%u msg:%s\n\x00" as *const u8 as *const libc::c_char,
                b"src/base64.c\x00" as *const u8 as *const libc::c_char,
                206i32,
                b"switch default that should not be possible\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(1i32);
        }
    }
    /* check if output buffer is big enough */
    result_len = 3i32 * full_blocks + last_bytes;
    if max_len < result_len {
        return -1i32;
    }
    /* process all the full blocks */
    i = 0i32;
    while i < full_blocks {
        b = ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i) as isize)) as libc::c_int) << 18i32)
            as uint32_t;
        b |= ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 1i32) as isize)) as libc::c_int)
            << 12i32) as libc::c_uint;
        b |= ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 2i32) as isize)) as libc::c_int)
            << 6i32) as libc::c_uint;
        b |= (0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 3i32) as isize)) as libc::c_int)
            as libc::c_uint;
        *out.offset((3i32 * i + 0i32) as isize) = (b >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t;
        *out.offset((3i32 * i + 1i32) as isize) = (b >> 8i32 & 0xffi32 as libc::c_uint) as uint8_t;
        *out.offset((3i32 * i + 2i32) as isize) = (b & 0xffi32 as libc::c_uint) as uint8_t;
        i += 1
    }
    /* process the last 'partial' block */
    i = full_blocks;
    if last_bytes == 1i32 {
        b = ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i) as isize)) as libc::c_int) << 18i32)
            as uint32_t;
        b |= ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 1i32) as isize)) as libc::c_int)
            << 12i32) as libc::c_uint;
        *out.offset((3i32 * i + 0i32) as isize) = (b >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t;
        (b >> 12i32 & 0xfi32 as libc::c_uint) != 0i32 as libc::c_uint;
    } else if last_bytes == 2i32 {
        b = ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i) as isize)) as libc::c_int) << 18i32)
            as uint32_t;
        b |= ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 1i32) as isize)) as libc::c_int)
            << 12i32) as libc::c_uint;
        b |= ((0x3fi32 & char_to_code(*in_0.offset((4i32 * i + 2i32) as isize)) as libc::c_int)
            << 6i32) as libc::c_uint;
        *out.offset((3i32 * i + 0i32) as isize) = (b >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t;
        *out.offset((3i32 * i + 1i32) as isize) = (b >> 8i32 & 0xffi32 as libc::c_uint) as uint8_t;
        (b >> 6i32 & 0x3i32 as libc::c_uint) != 0i32 as libc::c_uint;
    }
    return result_len;
}
/* === derivative functions === */
/* *
@brief Encode binary data in Base64 string (with added padding)
*/
#[no_mangle]
pub unsafe extern "C" fn bin_to_b64(
    mut in_0: *const uint8_t,
    mut size: libc::c_int,
    mut out: *mut libc::c_char,
    mut max_len: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = bin_to_b64_nopad(in_0, size, out, max_len);
    if ret == -1i32 {
        return -1i32;
    }
    match ret % 4i32 {
        0 => {
            /* nothing to do */
            return ret;
        }
        1 => return -1i32,
        2 => {
            /* 2 chars in last block, must add 2 padding char */
            if max_len >= ret + 2i32 + 1i32 {
                *out.offset(ret as isize) = code_pad;
                *out.offset((ret + 1i32) as isize) = code_pad;
                *out.offset((ret + 2i32) as isize) = 0i32 as libc::c_char;
                return ret + 2i32;
            } else {
                return -1i32;
            }
        }
        3 => {
            /* 3 chars in last block, must add 1 padding char */
            if max_len >= ret + 1i32 + 1i32 {
                *out.offset(ret as isize) = code_pad;
                *out.offset((ret + 1i32) as isize) = 0i32 as libc::c_char;
                return ret + 1i32;
            } else {
                return -1i32;
            }
        }
        _ => {
            fprintf(
                stderr,
                b"\nCRITICAL file:%s line:%u msg:%s\n\x00" as *const u8 as *const libc::c_char,
                b"src/base64.c\x00" as *const u8 as *const libc::c_char,
                284i32,
                b"switch default that should not be possible\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(1i32);
        }
    };
}
/* *
@brief Decode Base64 string to binary data (remove padding if necessary)
*/
#[no_mangle]
pub unsafe extern "C" fn b64_to_bin(
    mut in_0: *const libc::c_char,
    mut size: libc::c_int,
    mut out: *mut uint8_t,
    mut max_len: libc::c_int,
) -> libc::c_int {
    if in_0.is_null() {
        return -1i32;
    } /* treat as unpadded Base64 */
    if size % 4i32 == 0i32 && size >= 4i32 {
        /* potentially padded Base64 */
        if *in_0.offset((size - 2i32) as isize) as libc::c_int == code_pad as libc::c_int {
            /* 2 padding char to ignore */
            return b64_to_bin_nopad(in_0, size - 2i32, out, max_len);
        } else if *in_0.offset((size - 1i32) as isize) as libc::c_int == code_pad as libc::c_int {
            return b64_to_bin_nopad(in_0, size - 1i32, out, max_len);
        } else {
            return b64_to_bin_nopad(in_0, size, out, max_len);
        }
    } else {
        return b64_to_bin_nopad(in_0, size, out, max_len);
    }; /* no padding to ignore */
}
/* 1 padding char to ignore */
/* --- EOF ------------------------------------------------------------------ */
