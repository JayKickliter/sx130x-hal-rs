use libc;
extern "C" {
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    /* *
    @brief LoRa concentrator register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
    #[no_mangle]
    fn tinymt32_init(random: *mut tinymt32_t, seed: uint32_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
/* *
@struct lgw_conf_debug_s
@brief Configuration structure for debug
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_ref_payload_s {
    pub id: uint32_t,
    pub payload: [uint8_t; 255],
    pub prev_cnt: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_debug_s {
    pub nb_ref_payload: uint8_t,
    pub ref_payload: [conf_ref_payload_s; 16],
    pub log_file_name: [libc::c_char; 128],
}
pub type tinymt32_t = TINYMT32_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TINYMT32_T {
    pub status: [uint32_t; 4],
    pub mat1: uint32_t,
    pub mat2: uint32_t,
    pub tmat: uint32_t,
}
#[inline]
unsafe extern "C" fn tinymt32_temper(mut random: *mut tinymt32_t) -> uint32_t {
    let mut t0: uint32_t = 0;
    let mut t1: uint32_t = 0;
    t0 = (*random).status[3];
    t1 = (*random).status[0].wrapping_add((*random).status[2] >> 8i32);
    t0 ^= t1;
    t0 ^= -((t1 & 1i32 as libc::c_uint) as int32_t) as libc::c_uint & (*random).tmat;
    return t0;
}
#[inline]
unsafe extern "C" fn tinymt32_next_state(mut random: *mut tinymt32_t) {
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    y = (*random).status[3];
    x = (*random).status[0] & 0x7fffffffu32 ^ (*random).status[1] ^ (*random).status[2];
    x ^= x << 1i32;
    y ^= y >> 1i32 ^ x;
    (*random).status[0] = (*random).status[1];
    (*random).status[1] = (*random).status[2];
    (*random).status[2] = x ^ y << 10i32;
    (*random).status[3] = y;
    (*random).status[1] ^=
        -((y & 1i32 as libc::c_uint) as int32_t) as libc::c_uint & (*random).mat1;
    (*random).status[2] ^=
        -((y & 1i32 as libc::c_uint) as int32_t) as libc::c_uint & (*random).mat2;
}
#[inline]
unsafe extern "C" fn tinymt32_generate_uint32(mut random: *mut tinymt32_t) -> uint32_t {
    tinymt32_next_state(random);
    return tinymt32_temper(random);
}
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    LoRa concentrator debug functions

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* printf fprintf */
/* memcmp */
/* -------------------------------------------------------------------------- */
/* --- DEBUG CONSTANTS ------------------------------------------------------ */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS & TYPES -------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
static mut tinymt: tinymt32_t = tinymt32_t {
    status: [0; 4],
    mat1: 0,
    mat2: 0,
    tmat: 0,
};
/* *
@brief
@param
*/
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DECLARATION ---------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn dbg_init_random() {
    tinymt.mat1 = 0x8f7011eeu32;
    tinymt.mat2 = 0xfc78ff1fu32;
    tinymt.tmat = 0x3793fdffi32 as uint32_t;
}
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_init_gpio() {
    /* Select GPIO_6 to be controlled by HOST */
    lgw_reg_w(288i32 as uint16_t, 0i32);
    /* Configure it as an OUTPUT */
    lgw_reg_w(275i32 as uint16_t, 0xffi32);
}
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_toggle_gpio() {
    /* Set GPIO_6 to high */
    lgw_reg_w(277i32 as uint16_t, 64i32);
    /* Set GPIO_6 to low */
    lgw_reg_w(277i32 as uint16_t, 0i32);
}
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    LoRa concentrator HAL debug functions

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* library configuration options (dynamically generated) */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC MACROS -------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_log_buffer_to_file(
    mut file: *mut FILE,
    mut buffer: *mut uint8_t,
    mut size: uint16_t,
) {
    let mut i: libc::c_int = 0;
    let mut stat_timestamp: [libc::c_char; 24] = [0; 24];
    let mut t: time_t = 0;
    t = time(0 as *mut time_t);
    strftime(
        stat_timestamp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
        b"%F %T %Z\x00" as *const u8 as *const libc::c_char,
        gmtime(&mut t),
    );
    fprintf(
        file,
        b"---------(%s)------------\n\x00" as *const u8 as *const libc::c_char,
        stat_timestamp.as_mut_ptr(),
    );
    i = 0i32;
    while i < size as libc::c_int {
        fprintf(
            file,
            b"%02X \x00" as *const u8 as *const libc::c_char,
            *buffer.offset(i as isize) as libc::c_int,
        );
        i += 1
    }
    fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
    fflush(file);
}
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_log_payload_diff_to_file(
    mut file: *mut FILE,
    mut buffer1: *mut uint8_t,
    mut buffer2: *mut uint8_t,
    mut size: uint16_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nb_bits_diff: uint16_t = 0i32 as uint16_t;
    let mut debug_payload_diff: [uint8_t; 255] = [0; 255];
    fprintf(file, b"Diff: \x00" as *const u8 as *const libc::c_char);
    /* bit comparison of payloads */
    j = 0i32;
    while j < size as libc::c_int {
        debug_payload_diff[j as usize] = (*buffer1.offset(j as isize) as libc::c_int
            ^ *buffer2.offset(j as isize) as libc::c_int)
            as uint8_t;
        fprintf(
            file,
            b"%02X \x00" as *const u8 as *const libc::c_char,
            debug_payload_diff[j as usize] as libc::c_int,
        );
        j += 1
    }
    fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
    /* count number of bits flipped, and display bit by bit */
    j = 0i32;
    while j < size as libc::c_int {
        i = 7i32;
        while i >= 0i32 {
            fprintf(
                file,
                b"%u\x00" as *const u8 as *const libc::c_char,
                debug_payload_diff[j as usize] as libc::c_int >> i & (1i32 << 1i32) - 1i32,
            );
            if debug_payload_diff[j as usize] as libc::c_int >> i & (1i32 << 1i32) - 1i32 == 1i32 {
                nb_bits_diff = (nb_bits_diff as libc::c_int + 1i32) as uint16_t
            }
            i -= 1
        }
        fprintf(file, b" \x00" as *const u8 as *const libc::c_char);
        j += 1
    }
    fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(
        file,
        b"%u bits flipped\n\x00" as *const u8 as *const libc::c_char,
        nb_bits_diff as libc::c_int,
    );
    fflush(file);
}
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_generate_random_payload(
    mut pkt_cnt: uint32_t,
    mut buffer_expected: *mut uint8_t,
    mut size: uint8_t,
) {
    let mut k: libc::c_int = 0;
    /* construct payload we should get for this packet counter */
    tinymt32_init(&mut tinymt, pkt_cnt as libc::c_int as uint32_t); /* dummy: for sync with random size generation */
    *buffer_expected.offset(4) = (pkt_cnt >> 24i32) as uint8_t;
    *buffer_expected.offset(5) = (pkt_cnt >> 16i32) as uint8_t;
    *buffer_expected.offset(6) = (pkt_cnt >> 8i32) as uint8_t;
    *buffer_expected.offset(7) = (pkt_cnt >> 0i32) as uint8_t;
    tinymt32_generate_uint32(&mut tinymt);
    k = 8i32;
    while k < size as libc::c_int {
        *buffer_expected.offset(k as isize) = tinymt32_generate_uint32(&mut tinymt) as uint8_t;
        k += 1
    }
}
/* *
@brief
@param
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn dbg_check_payload(
    mut context: *mut lgw_conf_debug_s,
    mut file: *mut FILE,
    mut payload_received: *mut uint8_t,
    mut size: uint8_t,
    mut ref_payload_idx: uint8_t,
    mut sf: uint8_t,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut debug_payload_cnt: uint32_t = 0;
    /* If the 4 first bytes of received payload match with the expected ones, go on with comparison */
    if memcmp(
        payload_received as *mut libc::c_void,
        (*context).ref_payload[ref_payload_idx as usize]
            .payload
            .as_mut_ptr() as *mut libc::c_void,
        4i32 as libc::c_ulong,
    ) == 0i32
    {
        /* get counter to initialize random seed */
        debug_payload_cnt = ((*payload_received.offset(4) as libc::c_int) << 24i32) as libc::c_uint
            | ((*payload_received.offset(5) as libc::c_int) << 16i32) as libc::c_uint
            | ((*payload_received.offset(6) as libc::c_int) << 8i32) as libc::c_uint
            | ((*payload_received.offset(7) as libc::c_int) << 0i32) as libc::c_uint;
        /* check if we missed some packets */
        if debug_payload_cnt
            > (*context).ref_payload[ref_payload_idx as usize]
                .prev_cnt
                .wrapping_add(1i32 as libc::c_uint)
        {
            printf(
                b"ERROR: 0x%08X missed %u pkt before %u (SF%u, size:%u)\n\x00" as *const u8
                    as *const libc::c_char,
                (*context).ref_payload[ref_payload_idx as usize].id,
                debug_payload_cnt
                    .wrapping_sub((*context).ref_payload[ref_payload_idx as usize].prev_cnt)
                    .wrapping_sub(1i32 as libc::c_uint),
                debug_payload_cnt,
                sf as libc::c_int,
                size as libc::c_int,
            );
            if !file.is_null() {
                fprintf(
                    file,
                    b"ERROR: 0x%08X missed %u pkt before %u (SF%u, size:%u)\n\x00" as *const u8
                        as *const libc::c_char,
                    (*context).ref_payload[ref_payload_idx as usize].id,
                    debug_payload_cnt
                        .wrapping_sub((*context).ref_payload[ref_payload_idx as usize].prev_cnt)
                        .wrapping_sub(1i32 as libc::c_uint),
                    debug_payload_cnt,
                    sf as libc::c_int,
                    size as libc::c_int,
                );
                fflush(file);
            }
        } else if debug_payload_cnt < (*context).ref_payload[ref_payload_idx as usize].prev_cnt {
            if !file.is_null() {
                fprintf(
                    file,
                    b"INFO:  0x%08X got missing pkt %u (SF%u, size:%u) ?\n\x00" as *const u8
                        as *const libc::c_char,
                    (*context).ref_payload[ref_payload_idx as usize].id,
                    debug_payload_cnt,
                    sf as libc::c_int,
                    size as libc::c_int,
                );
                fflush(file);
            }
        }
        (*context).ref_payload[ref_payload_idx as usize].prev_cnt = debug_payload_cnt;
        /* generate the random payload which is expected for this packet count */
        dbg_generate_random_payload(
            debug_payload_cnt,
            (*context).ref_payload[ref_payload_idx as usize]
                .payload
                .as_mut_ptr(),
            size,
        );
        /* compare expected with received */
        if memcmp(
            payload_received as *mut libc::c_void,
            (*context).ref_payload[ref_payload_idx as usize]
                .payload
                .as_mut_ptr() as *mut libc::c_void,
            size as libc::c_ulong,
        ) != 0i32
        {
            if !file.is_null() {
                fprintf(file, b"RECEIVED:\x00" as *const u8 as *const libc::c_char);
                k = 0i32;
                while k < size as libc::c_int {
                    fprintf(
                        file,
                        b"%02X \x00" as *const u8 as *const libc::c_char,
                        *payload_received.offset(k as isize) as libc::c_int,
                    );
                    k += 1
                }
                fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
                fprintf(file, b"EXPECTED:\x00" as *const u8 as *const libc::c_char);
                k = 0i32;
                while k < size as libc::c_int {
                    fprintf(
                        file,
                        b"%02X \x00" as *const u8 as *const libc::c_char,
                        (*context).ref_payload[ref_payload_idx as usize].payload[k as usize]
                            as libc::c_int,
                    );
                    k += 1
                }
                fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
            }
            return -1i32;
        } else {
            return 1i32;
            /* matches */
        }
    }
    return 0i32;
    /* ignored */
}
