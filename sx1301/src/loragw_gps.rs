use libc;
extern "C" {
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn tzset();
    #[no_mangle]
    static mut timezone: libc::c_long;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    #[no_mangle]
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    #[no_mangle]
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tref {
    pub systime: time_t,
    pub count_us: uint32_t,
    pub utc: timespec,
    pub gps: timespec,
    pub xtal_err: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coord_s {
    pub lat: libc::c_double,
    pub lon: libc::c_double,
    pub alt: libc::c_short,
}
pub type gps_msg = libc::c_uint;
pub const UBX_NAV_TIMEUTC: gps_msg = 16;
pub const UBX_NAV_TIMEGPS: gps_msg = 15;
pub const NMEA_VTG: gps_msg = 14;
pub const NMEA_TXT: gps_msg = 13;
pub const NMEA_GLL: gps_msg = 12;
pub const NMEA_GSV: gps_msg = 11;
pub const NMEA_GSA: gps_msg = 10;
pub const NMEA_GST: gps_msg = 9;
pub const NMEA_GBS: gps_msg = 8;
pub const NMEA_ZDA: gps_msg = 7;
pub const NMEA_GNS: gps_msg = 6;
pub const NMEA_GGA: gps_msg = 5;
pub const NMEA_RMC: gps_msg = 4;
pub const INCOMPLETE: gps_msg = 3;
pub const INVALID: gps_msg = 2;
pub const IGNORED: gps_msg = 1;
pub const UNKNOWN: gps_msg = 0;
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
/* result of the NMEA parsing */
static mut gps_yea: libc::c_short = 0i32 as libc::c_short;
/* year (2 or 4 digits) */
static mut gps_mon: libc::c_short = 0i32 as libc::c_short;
/* month (1-12) */
static mut gps_day: libc::c_short = 0i32 as libc::c_short;
/* day of the month (1-31) */
static mut gps_hou: libc::c_short = 0i32 as libc::c_short;
/* hours (0-23) */
static mut gps_min: libc::c_short = 0i32 as libc::c_short;
/* minutes (0-59) */
static mut gps_sec: libc::c_short = 0i32 as libc::c_short;
/* seconds (0-60)(60 is for leap second) */
static mut gps_fra: libc::c_float = 0.0f64 as libc::c_float;
/* fractions of seconds (<1) */
static mut gps_time_ok: bool = 0i32 != 0;
static mut gps_week: int16_t = 0i32 as int16_t;
/* GPS week number of the navigation epoch */
static mut gps_iTOW: uint32_t = 0i32 as uint32_t;
/* GPS time of week in milliseconds */
static mut gps_fTOW: int32_t = 0i32;
/* Fractional part of iTOW (+/-500000) in nanosec */
static mut gps_dla: libc::c_short = 0i32 as libc::c_short;
/* degrees of latitude */
static mut gps_mla: libc::c_double = 0.0f64;
/* minutes of latitude */
static mut gps_ola: libc::c_char = 0i32 as libc::c_char;
/* orientation (N-S) of latitude */
static mut gps_dlo: libc::c_short = 0i32 as libc::c_short;
/* degrees of longitude */
static mut gps_mlo: libc::c_double = 0.0f64;
/* minutes of longitude */
static mut gps_olo: libc::c_char = 0i32 as libc::c_char;
/* orientation (E-W) of longitude */
static mut gps_alt: libc::c_short = 0i32 as libc::c_short;
/* altitude */
static mut gps_pos_ok: bool = 0i32 != 0;
static mut gps_mod: libc::c_char = 'N' as i32 as libc::c_char;
/* GPS mode (N no fix, A autonomous, D differential) */
static mut gps_sat: libc::c_short = 0i32 as libc::c_short;
/* number of satellites used for fix */
static mut ttyopt_restore: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DECLARATION ---------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/*
Calculate the checksum for a NMEA string
Skip the first '$' if necessary and calculate checksum until '*' character is
reached (or buff_size exceeded).
Checksum must point to a 2-byte (or more) char array.
Return position of the checksum in the string
*/
unsafe extern "C" fn nmea_checksum(
    mut nmea_string: *const libc::c_char,
    mut buff_size: libc::c_int,
    mut checksum: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    let mut check_num: uint8_t = 0i32 as uint8_t;
    /* check input parameters */
    if nmea_string.is_null() || checksum.is_null() || buff_size <= 1i32 {
        return -1i32;
    }
    /* skip the first '$' if necessary */
    if *nmea_string.offset(i as isize) as libc::c_int == '$' as i32 {
        i += 1i32
    }
    /* xor until '*' or max length is reached */
    while *nmea_string.offset(i as isize) as libc::c_int != '*' as i32 {
        check_num =
            (check_num as libc::c_int ^ *nmea_string.offset(i as isize) as libc::c_int) as uint8_t;
        i += 1i32;
        if i >= buff_size {
            return -1i32;
        }
    }
    /* Convert checksum value to 2 hexadecimal characters */
    *checksum.offset(0) = nibble_to_hexchar((check_num as libc::c_int / 16i32) as uint8_t); /* upper nibble */
    *checksum.offset(1) = nibble_to_hexchar((check_num as libc::c_int % 16i32) as uint8_t); /* lower nibble */
    return i + 1i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
unsafe extern "C" fn nibble_to_hexchar(mut a: uint8_t) -> libc::c_char {
    if (a as libc::c_int) < 10i32 {
        return ('0' as i32 + a as libc::c_int) as libc::c_char;
    } else if (a as libc::c_int) < 16i32 {
        return ('A' as i32 + (a as libc::c_int - 10i32)) as libc::c_char;
    } else {
        return '?' as i32 as libc::c_char;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*
Calculate the checksum of a NMEA frame and compare it to the checksum that is
present at the end of it.
Return true if it matches
*/
unsafe extern "C" fn validate_nmea_checksum(
    mut serial_buff: *const libc::c_char,
    mut buff_size: libc::c_int,
) -> bool {
    let mut checksum_index: libc::c_int = 0; /* 2 characters to calculate NMEA checksum */
    let mut checksum: [libc::c_char; 2] = [0; 2];
    checksum_index = nmea_checksum(serial_buff, buff_size, checksum.as_mut_ptr());
    /* could we calculate a verification checksum ? */
    if checksum_index < 0i32 {
        return 0i32 != 0;
    }
    /* check if there are enough char in the serial buffer to read checksum */
    if checksum_index >= buff_size - 2i32 {
        return 0i32 != 0;
    }
    /* check the checksum per se */
    if *serial_buff.offset(checksum_index as isize) as libc::c_int == checksum[0] as libc::c_int
        && *serial_buff.offset((checksum_index + 1i32) as isize) as libc::c_int
            == checksum[1] as libc::c_int
    {
        return 1i32 != 0;
    } else {
        return 0i32 != 0;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*
Return true if the "label" string (can contain wildcard characters) matches
the begining of the "s" string
*/
unsafe extern "C" fn match_label(
    mut s: *const libc::c_char,
    mut label: *mut libc::c_char,
    mut size: libc::c_int,
    mut wildcard: libc::c_char,
) -> bool {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < size {
        if !(*label.offset(i as isize) as libc::c_int == wildcard as libc::c_int) {
            if *label.offset(i as isize) as libc::c_int != *s.offset(i as isize) as libc::c_int {
                return 0i32 != 0;
            }
        }
        i += 1
    }
    return 1i32 != 0;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*
Chop a string into smaller strings
Replace every separator in the input character buffer by a null character so
that all s[index] are valid strings.
Populate an array of integer 'idx_ary' representing indexes of token in the
string.
buff_size and max_idx are there to prevent segfaults.
Return the number of token found (number of idx_ary filled).
*/
unsafe extern "C" fn str_chop(
    mut s: *mut libc::c_char,
    mut buff_size: libc::c_int,
    mut separator: libc::c_char,
    mut idx_ary: *mut libc::c_int,
    mut max_idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0i32; /* index in the string */
    let mut j: libc::c_int = 0i32; /* index in the result array */
    if s.is_null()
        || buff_size < 0i32
        || separator as libc::c_int == 0i32
        || idx_ary.is_null()
        || max_idx < 0i32
    {
        /* unsafe to do anything */
        return -1i32;
    }
    if buff_size == 0i32 || max_idx == 0i32 {
        /* nothing to do */
        return 0i32;
    } /* add string terminator at the end of the buffer, just to be sure */
    *s.offset((buff_size - 1i32) as isize) = 0i32 as libc::c_char;
    *idx_ary.offset(j as isize) = 0i32;
    j += 1i32;
    /* loop until string terminator is reached */
    while *s.offset(i as isize) as libc::c_int != 0i32 {
        if *s.offset(i as isize) as libc::c_int == separator as libc::c_int {
            *s.offset(i as isize) = 0i32 as libc::c_char; /* replace separator by string terminator */
            if j >= max_idx {
                /* no more room in the index array */
                return j;
            } /* next token start after replaced separator */
            *idx_ary.offset(j as isize) = i + 1i32;
            j += 1
        }
        i += 1
    }
    return j;
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Configure a GPS module

@param tty_path path to the TTY connected to the GPS
@param gps_familly parameter (eg. ubx6 for uBlox gen.6)
@param target_brate target baudrate for communication (0 keeps default target baudrate)
@param fd_ptr pointer to a variable to receive file descriptor on GPS tty
@return success if the function was able to connect and configure a GPS module
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn lgw_gps_enable(
    mut tty_path: *mut libc::c_char,
    mut gps_family: *mut libc::c_char,
    mut target_brate: speed_t,
    mut fd_ptr: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* serial port options */
    let mut ttyopt: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    }; /* file descriptor to the serial port of the GNSS module */
    let mut gps_tty_dev: libc::c_int = 0; /* Checksum */
    let mut ubx_cmd_timegps: [uint8_t; 16] = [
        0xb5i32 as uint8_t,
        0x62i32 as uint8_t,
        0x6i32 as uint8_t,
        0x1i32 as uint8_t,
        0x8i32 as uint8_t,
        0i32 as uint8_t,
        0x1i32 as uint8_t,
        0x20i32 as uint8_t,
        0i32 as uint8_t,
        0x1i32 as uint8_t,
        0x1i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0x32i32 as uint8_t,
        0x94i32 as uint8_t,
    ];
    let mut num_written: ssize_t = 0;
    /* check input parameters */
    if tty_path.is_null() {
        return -1i32;
    }
    if fd_ptr.is_null() {
        return -1i32;
    }
    /* open TTY device */
    gps_tty_dev = open(tty_path, 0o2i32 | 0o400i32);
    if gps_tty_dev <= 0i32 {
        return -1i32;
    }
    *fd_ptr = gps_tty_dev;
    /* manage the different GPS modules families */
    if !gps_family.is_null() {
        (strncmp(
            gps_family,
            b"ubx7\x00" as *const u8 as *const libc::c_char,
            4i32 as libc::c_ulong,
        )) != 0i32;
    }
    /* manage the target bitrate */
    (target_brate) != 0i32 as libc::c_uint;
    /* get actual serial port configuration */
    i = tcgetattr(gps_tty_dev, &mut ttyopt);
    if i != 0i32 {
        return -1i32;
    }
    /* Save current serial port configuration for restoring later */
    memcpy(
        &mut ttyopt_restore as *mut termios as *mut libc::c_void,
        &mut ttyopt as *mut termios as *const libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    /* update baudrates */
    cfsetispeed(&mut ttyopt, 0o15i32 as speed_t);
    cfsetospeed(&mut ttyopt, 0o15i32 as speed_t);
    /* update terminal parameters */
    /* The following configuration should allow to:
        - Get ASCII NMEA messages
        - Get UBX binary messages
        - Send UBX binary commands
    Note: as binary data have to be read/written, we need to disable
          various character processing to avoid loosing data */
    /* Control Modes */
    ttyopt.c_cflag |= 0o4000i32 as libc::c_uint; /* local connection, no modem control */
    ttyopt.c_cflag |= 0o200i32 as libc::c_uint; /* enable receiving characters */
    ttyopt.c_cflag |= 0o60i32 as libc::c_uint; /* 8 bit frames */
    ttyopt.c_cflag &= !0o400i32 as libc::c_uint; /* no parity */
    ttyopt.c_cflag &= !0o100i32 as libc::c_uint; /* one stop bit */
    /* Input Modes */
    ttyopt.c_iflag |= 0o4i32 as libc::c_uint; /* ignore bytes with parity errors */
    ttyopt.c_iflag &= !0o400i32 as libc::c_uint; /* do not map CR to NL on input*/
    ttyopt.c_iflag &= !0o200i32 as libc::c_uint; /* do not ignore carriage return on input */
    ttyopt.c_iflag &= !0o2000i32 as libc::c_uint; /* disable Start/Stop output control */
    ttyopt.c_iflag &= !0o10000i32 as libc::c_uint; /* do not send Start/Stop characters */
    /* Output Modes */
    ttyopt.c_oflag = 0i32 as tcflag_t; /* disable everything on output as we only write binary */
    /* Local Modes */
    ttyopt.c_lflag &= !0o2i32 as libc::c_uint; /* disable canonical input - cannot use with binary input */
    ttyopt.c_lflag &= !0o1i32 as libc::c_uint; /* disable check for INTR, QUIT, SUSP special characters */
    ttyopt.c_lflag &= !0o100000i32 as libc::c_uint; /* disable any special control character */
    ttyopt.c_lflag &= !0o10i32 as libc::c_uint; /* do not echo back every character typed */
    ttyopt.c_lflag &= !0o20i32 as libc::c_uint; /* does not erase the last character in current line */
    ttyopt.c_lflag &= !0o40i32 as libc::c_uint; /* do not echo NL after KILL character */
    /* settings for non-canonical mode
    read will block for until the lesser of VMIN or requested chars have been received */
    ttyopt.c_cc[6] = 8i32 as cc_t;
    ttyopt.c_cc[5] = 0i32 as cc_t;
    /* set new serial ports parameters */
    i = tcsetattr(gps_tty_dev, 0i32, &mut ttyopt);
    if i != 0i32 {
        return -1i32;
    }
    tcflush(gps_tty_dev, 2i32);
    /* Send UBX CFG NAV-TIMEGPS message to tell GPS module to output native GPS time */
    /* This is a binary message, serial port has to be properly configured to handle this */
    num_written = write(
        gps_tty_dev,
        ubx_cmd_timegps.as_mut_ptr() as *const libc::c_void,
        16i32 as size_t,
    );
    (num_written) != 16i32 as libc::c_long;
    /* get timezone info */
    tzset();
    /* initialize global variables */
    gps_time_ok = 0i32 != 0;
    gps_pos_ok = 0i32 != 0;
    gps_mod = 'N' as i32 as libc::c_char;
    return 0i32;
}
/* *
@brief Restore GPS serial configuration and close serial device

@param fd file descriptor on GPS tty
@return success if the function was able to complete
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_gps_disable(mut fd: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* restore serial ports parameters */
    i = tcsetattr(fd, 0i32, &mut ttyopt_restore);
    if i != 0i32 {
        return -1i32;
    }
    tcflush(fd, 2i32);
    i = close(fd);
    if i <= 0i32 {
        return -1i32;
    }
    return 0i32;
}
/* *
@brief Parse Ublox proprietary messages coming from the GPS system

@param serial_buff pointer to the string to be parsed
@param buff_size maximum string lengths for UBX parsing (incl. null char)
@param msg_size number of bytes parsed as UBX message if found
@return type of frame parsed

The RAW UBX sentences are parsed to a global set of variables shared with the
lgw_gps_get function.
If the lgw_parse_ubx and lgw_gps_get are used in different threads, a mutex
lock must be acquired before calling either function.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_parse_ubx(
    mut serial_buff: *const libc::c_char,
    mut buff_size: size_t,
    mut msg_size: *mut size_t,
) -> gps_msg {
    let mut valid: bool = 0i32 != 0; /* iTOW, fTOW and week validity */
    let mut payload_length: libc::c_uint = 0; /* ensure msg_size alway receives a value */
    let mut ck_a: uint8_t = 0;
    let mut ck_b: uint8_t = 0;
    let mut ck_a_rcv: uint8_t = 0;
    let mut ck_b_rcv: uint8_t = 0;
    let mut i: libc::c_uint = 0;
    *msg_size = 0i32 as size_t;
    /* check input parameters */
    if serial_buff.is_null() {
        return IGNORED;
    }
    if buff_size < 8i32 as libc::c_ulong {
        return IGNORED;
    }
    /* display received serial data and checksum */
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) < buff_size {
        i = i.wrapping_add(1)
    }
    /* Check for UBX sync chars 0xB5 0x62 */
    if *serial_buff.offset(0) as libc::c_int == 0xb5i32 as libc::c_char as libc::c_int
        && *serial_buff.offset(1) as libc::c_int == 0x62i32 as libc::c_char as libc::c_int
    {
        payload_length = *serial_buff.offset(4) as uint8_t as libc::c_uint; /* Not a UBX message */
        payload_length |=
            ((*serial_buff.offset(5) as uint8_t as libc::c_int) << 8i32) as libc::c_uint;
        /* Get payload length to compute message size */
        *msg_size = (6i32 as libc::c_uint)
            .wrapping_add(payload_length)
            .wrapping_add(2i32 as libc::c_uint) as size_t; /* header + payload + checksum */
        /* check for complete message in buffer */
        if *msg_size <= buff_size {
            ck_a_rcv = *serial_buff.offset((*msg_size).wrapping_sub(2i32 as libc::c_ulong) as isize)
                as uint8_t; /* message contains less bytes than indicated by header */
            /* Validate checksum of message */
            /* received checksum */
            ck_b_rcv = *serial_buff.offset((*msg_size).wrapping_sub(1i32 as libc::c_ulong) as isize)
                as uint8_t; /* received checksum */
            /* Use 8-bit Fletcher Algorithm to compute checksum of actual payload */
            ck_a = 0i32 as uint8_t;
            ck_b = 0i32 as uint8_t;
            i = 0i32 as libc::c_uint;
            while i < (4i32 as libc::c_uint).wrapping_add(payload_length) {
                ck_a = (ck_a as libc::c_int
                    + *serial_buff.offset(i.wrapping_add(2i32 as libc::c_uint) as isize)
                        as libc::c_int) as uint8_t;
                ck_b = (ck_b as libc::c_int + ck_a as libc::c_int) as uint8_t;
                i = i.wrapping_add(1)
            }
            /* Compare checksums and parse if OK */
            if ck_a as libc::c_int == ck_a_rcv as libc::c_int
                && ck_b as libc::c_int == ck_b_rcv as libc::c_int
            {
                if *serial_buff.offset(2) as libc::c_int == 0x1i32
                    && *serial_buff.offset(3) as libc::c_int == 0x20i32
                {
                    /* checksum failed */
                    /* Check for Class 0x01 (NAV) and ID 0x20 (NAV-TIMEGPS) */
                    /* Check validity of information */
                    valid = *serial_buff.offset(17) as libc::c_int & 0x3i32 != 0; /* towValid, weekValid */
                    if valid {
                        gps_iTOW = *serial_buff.offset(6) as uint8_t as uint32_t; /* valid */
                        gps_iTOW |= ((*serial_buff.offset(7) as uint8_t as libc::c_int) << 8i32)
                            as libc::c_uint;
                        gps_iTOW |= ((*serial_buff.offset(8) as uint8_t as libc::c_int) << 16i32)
                            as libc::c_uint;
                        /* Parse buffer to extract GPS time */
                        /* Warning: payload byte ordering is Little Endian */
                        gps_iTOW |= ((*serial_buff.offset(9) as uint8_t as libc::c_int) << 24i32)
                            as libc::c_uint; /* GPS time of week, in ms */
                        gps_fTOW = *serial_buff.offset(10) as uint8_t as int32_t; /* Fractional part of iTOW, in ns */
                        gps_fTOW |= (*serial_buff.offset(11) as uint8_t as libc::c_int) << 8i32; /* GPS week number */
                        gps_fTOW |= (*serial_buff.offset(12) as uint8_t as libc::c_int) << 16i32; /* not a supported message */
                        gps_fTOW |= (*serial_buff.offset(13) as uint8_t as libc::c_int) << 24i32;
                        gps_week = *serial_buff.offset(14) as uint8_t as int16_t;
                        gps_week = (gps_week as libc::c_int
                            | (*serial_buff.offset(15) as uint8_t as libc::c_int) << 8i32)
                            as int16_t;
                        gps_time_ok = 1i32 != 0
                    } else {
                        gps_time_ok = 0i32 != 0
                    }
                    return UBX_NAV_TIMEGPS;
                } else if *serial_buff.offset(2) as libc::c_int == 0x5i32
                    && *serial_buff.offset(3) as libc::c_int == 0i32
                {
                    return IGNORED;
                } else if *serial_buff.offset(2) as libc::c_int == 0x5i32
                    && *serial_buff.offset(3) as libc::c_int == 0x1i32
                {
                    return IGNORED;
                } else {
                    return IGNORED;
                }
            } else {
                return INVALID;
            }
        } else {
            return INCOMPLETE;
        }
    } else {
        /* Ignore messages which are not UBX ones for now */
        return IGNORED;
    };
}
/* *
@brief Parse messages coming from the GPS system (or other GNSS)

@param serial_buff pointer to the string to be parsed
@param buff_size maximum string lengths for NMEA parsing (incl. null char)
@return type of frame parsed

The RAW NMEA sentences are parsed to a global set of variables shared with the
lgw_gps_get function.
If the lgw_parse_nmea and lgw_gps_get are used in different threads, a mutex
lock must be acquired before calling either function.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_parse_nmea(
    mut serial_buff: *const libc::c_char,
    mut buff_size: libc::c_int,
) -> gps_msg {
    let mut i: libc::c_int = 0; /* string index from the string chopping */
    let mut j: libc::c_int = 0; /* number of strings detected by string chopping */
    let mut k: libc::c_int = 0; /* parsing modifies buffer so need a local copy */
    let mut str_index: [libc::c_int; 30] = [0; 30];
    let mut nb_fields: libc::c_int = 0;
    let mut parser_buf: [libc::c_char; 256] = [0; 256];
    /* check input parameters */
    if serial_buff.is_null() {
        return UNKNOWN;
    }
    if buff_size
        > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
    {
        return INVALID;
    }
    /* look for some NMEA sentences in particular */
    if buff_size < 8i32 {
        return UNKNOWN;
    } else if !validate_nmea_checksum(serial_buff, buff_size) {
        return INVALID;
    } else if match_label(
        serial_buff,
        b"$G?RMC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6i32,
        '?' as i32 as libc::c_char,
    ) {
        /*
        NMEA sentence format: $xxRMC,time,status,lat,NS,long,EW,spd,cog,date,mv,mvEW,posMode*cs<CR><LF>
        Valid fix: $GPRMC,083559.34,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A*00
        No fix: $GPRMC,,V,,,,,,,,,,N*00
        */
        memcpy(
            parser_buf.as_mut_ptr() as *mut libc::c_void,
            serial_buff as *const libc::c_void,
            buff_size as libc::c_ulong,
        );
        parser_buf[buff_size as usize] = '\u{0}' as i32 as libc::c_char;
        nb_fields = str_chop(
            parser_buf.as_mut_ptr(),
            buff_size,
            ',' as i32 as libc::c_char,
            str_index.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_int; 30]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        );
        if nb_fields != 13i32 {
            return IGNORED;
        }
        /* parse GPS status */
        gps_mod = *parser_buf.as_mut_ptr().offset(str_index[12] as isize); /* get first character, no need to bother with sscanf */
        if gps_mod as libc::c_int != 'N' as i32
            && gps_mod as libc::c_int != 'A' as i32
            && gps_mod as libc::c_int != 'D' as i32
        {
            gps_mod = 'N' as i32 as libc::c_char
        }
        /* parse complete time */
        i = sscanf(
            parser_buf.as_mut_ptr().offset(str_index[1] as isize),
            b"%2hd%2hd%2hd%4f\x00" as *const u8 as *const libc::c_char,
            &mut gps_hou as *mut libc::c_short,
            &mut gps_min as *mut libc::c_short,
            &mut gps_sec as *mut libc::c_short,
            &mut gps_fra as *mut libc::c_float,
        );
        j = sscanf(
            parser_buf.as_mut_ptr().offset(str_index[9] as isize),
            b"%2hd%2hd%2hd\x00" as *const u8 as *const libc::c_char,
            &mut gps_day as *mut libc::c_short,
            &mut gps_mon as *mut libc::c_short,
            &mut gps_yea as *mut libc::c_short,
        );
        if i == 4i32 && j == 3i32 {
            if gps_mod as libc::c_int == 'A' as i32 || gps_mod as libc::c_int == 'D' as i32 {
                gps_time_ok = 1i32 != 0
            } else {
                gps_time_ok = 0i32 != 0
            }
        } else {
            /* could not get a valid hour AND date */
            gps_time_ok = 0i32 != 0
        }
        return NMEA_RMC;
    } else if match_label(
        serial_buff,
        b"$G?GGA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6i32,
        '?' as i32 as libc::c_char,
    ) {
        /*
        NMEA sentence format: $xxGGA,time,lat,NS,long,EW,quality,numSV,HDOP,alt,M,sep,M,diffAge,diffStation*cs<CR><LF>
        Valid fix: $GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B
        */
        memcpy(
            parser_buf.as_mut_ptr() as *mut libc::c_void,
            serial_buff as *const libc::c_void,
            buff_size as libc::c_ulong,
        );
        parser_buf[buff_size as usize] = '\u{0}' as i32 as libc::c_char;
        nb_fields = str_chop(
            parser_buf.as_mut_ptr(),
            buff_size,
            ',' as i32 as libc::c_char,
            str_index.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_int; 30]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        );
        if nb_fields != 15i32 {
            return IGNORED;
        }
        /* parse number of satellites used for fix */
        sscanf(
            parser_buf.as_mut_ptr().offset(str_index[7] as isize),
            b"%hd\x00" as *const u8 as *const libc::c_char,
            &mut gps_sat as *mut libc::c_short,
        );
        /* parse 3D coordinates */
        i = sscanf(
            parser_buf.as_mut_ptr().offset(str_index[2] as isize),
            b"%2hd%10lf\x00" as *const u8 as *const libc::c_char,
            &mut gps_dla as *mut libc::c_short,
            &mut gps_mla as *mut libc::c_double,
        );
        gps_ola = *parser_buf.as_mut_ptr().offset(str_index[3] as isize);
        j = sscanf(
            parser_buf.as_mut_ptr().offset(str_index[4] as isize),
            b"%3hd%10lf\x00" as *const u8 as *const libc::c_char,
            &mut gps_dlo as *mut libc::c_short,
            &mut gps_mlo as *mut libc::c_double,
        );
        gps_olo = *parser_buf.as_mut_ptr().offset(str_index[5] as isize);
        k = sscanf(
            parser_buf.as_mut_ptr().offset(str_index[9] as isize),
            b"%hd\x00" as *const u8 as *const libc::c_char,
            &mut gps_alt as *mut libc::c_short,
        );
        if i == 2i32
            && j == 2i32
            && k == 1i32
            && (gps_ola as libc::c_int == 'N' as i32 || gps_ola as libc::c_int == 'S' as i32)
            && (gps_olo as libc::c_int == 'E' as i32 || gps_olo as libc::c_int == 'W' as i32)
        {
            gps_pos_ok = 1i32 != 0
        } else {
            /* could not get a valid latitude, longitude AND altitude */
            gps_pos_ok = 0i32 != 0
        }
        return NMEA_GGA;
    } else {
        /* quite verbose */
        return IGNORED;
    };
}
/* *
@brief Get the GPS solution (space & time) for the concentrator

@param utc pointer to store UTC time, with ns precision (NULL to ignore)
@param gps_time pointer to store GPS time, with ns precision (NULL to ignore)
@param loc pointer to store coordinates (NULL to ignore)
@param err pointer to store coordinates standard deviation (NULL to ignore)
@return success if the chosen elements could be returned

This function read the global variables generated by the NMEA/UBX parsing
functions lgw_parse_nmea/lgw_parse_ubx. It returns time and location data in a
format that is exploitable by other functions in that library sub-module.
If the lgw_parse_nmea/lgw_parse_ubx and lgw_gps_get are used in different
threads, a mutex lock must be acquired before calling either function.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_gps_get(
    mut utc: *mut timespec,
    mut gps_time: *mut timespec,
    mut loc: *mut coord_s,
    mut err: *mut coord_s,
) -> libc::c_int {
    let mut x: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut y: time_t = 0;
    let mut intpart: libc::c_double = 0.;
    let mut fractpart: libc::c_double = 0.;
    if !utc.is_null() {
        if !gps_time_ok {
            return -1i32;
        }
        memset(
            &mut x as *mut tm as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<tm>() as libc::c_ulong,
        );
        if (gps_yea as libc::c_int) < 100i32 {
            /* 2-digits year, 20xx */
            x.tm_year = gps_yea as libc::c_int + 100i32
        /* 100 years offset to 1900 */
        } else {
            x.tm_year = gps_yea as libc::c_int - 1900i32
        } /* tm_mon is [0,11], gps_mon is [1,12] */
        x.tm_mon = gps_mon as libc::c_int - 1i32; /* need to substract timezone bc mktime assumes time vector is local time */
        x.tm_mday = gps_day as libc::c_int;
        x.tm_hour = gps_hou as libc::c_int;
        x.tm_min = gps_min as libc::c_int;
        x.tm_sec = gps_sec as libc::c_int;
        y = mktime(&mut x) - timezone;
        if y == -1i32 as time_t {
            return -1i32;
        }
        (*utc).tv_sec = y;
        (*utc).tv_nsec = (gps_fra as libc::c_double * 1e9f64) as int32_t as __syscall_slong_t
    }
    if !gps_time.is_null() {
        if !gps_time_ok {
            return -1i32;
        }
        fractpart = modf(
            gps_iTOW as libc::c_double / 1E3f64 + gps_fTOW as libc::c_double / 1E9f64,
            &mut intpart,
        );
        /* Number of seconds since beginning on current GPS week */
        (*gps_time).tv_sec = intpart as time_t;
        /* Number of seconds since GPS epoch 06.Jan.1980 */
        (*gps_time).tv_sec += gps_week as time_t * 604800i32 as libc::c_long; /* day*hours*minutes*secondes: 7*24*60*60; */
        /* Fractional part in nanoseconds */
        (*gps_time).tv_nsec = (fractpart * 1E9f64) as libc::c_long
    }
    if !loc.is_null() {
        if !gps_pos_ok {
            return -1i32;
        }
        (*loc).lat = (gps_dla as libc::c_double + gps_mla / 60.0f64)
            * (if gps_ola as libc::c_int == 'N' as i32 {
                1.0f64
            } else {
                -1.0f64
            });
        (*loc).lon = (gps_dlo as libc::c_double + gps_mlo / 60.0f64)
            * (if gps_olo as libc::c_int == 'E' as i32 {
                1.0f64
            } else {
                -1.0f64
            });
        (*loc).alt = gps_alt
    }
    if !err.is_null() {
        (*err).lat = 0.0f64;
        (*err).lon = 0.0f64;
        (*err).alt = 0i32 as libc::c_short
    }
    return 0i32;
}
/* *
@brief Get time and position information from the serial GPS last message received
@param utc UTC time, with ns precision (leap seconds are ignored)
@param gps_time timestamp of last time pulse from the GPS module converted to the UNIX epoch
       (leap seconds are ignored)
@param loc location information
@param err location error estimate if supported
@return success if timestamp was read and time reference could be refreshed

Set systime to 0 in ref to trigger initial synchronization.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_gps_sync(
    mut ref_0: *mut tref,
    mut count_us: uint32_t,
    mut utc: timespec,
    mut gps_time: timespec,
) -> libc::c_int {
    let mut cnt_diff: libc::c_double = 0.; /* internal concentrator time difference (in seconds) */
    let mut utc_diff: libc::c_double = 0.; /* UTC time difference (in seconds) */
    let mut slope: libc::c_double = 0.; /* time slope between new reference and old reference (for sanity check) */
    let mut aber_n0: bool = false; /* is the update value for synchronization aberrant or not ? */
    static mut aber_min1: bool = 0i32 != 0; /* keep track of whether value at sync N-1 was aberrant or not  */
    static mut aber_min2: bool = 0i32 != 0; /* keep track of whether value at sync N-2 was aberrant or not  */
    if ref_0.is_null() {
        return -1i32;
    }
    /* calculate the slope */
    cnt_diff = count_us.wrapping_sub((*ref_0).count_us) as libc::c_double / 1E6f64; /* uncorrected by xtal_err */
    utc_diff = (utc.tv_sec - (*ref_0).utc.tv_sec) as libc::c_double
        + 1E-9f64 * (utc.tv_nsec - (*ref_0).utc.tv_nsec) as libc::c_double;
    /* detect aberrant points by measuring if slope limits are exceeded */
    if utc_diff != 0i32 as libc::c_double {
        // prevent divide by zero
        slope = cnt_diff / utc_diff;
        if slope > 1.00001f64 || slope < 0.99999f64 {
            aber_n0 = 1i32 != 0
        } else {
            aber_n0 = 0i32 != 0
        }
    } else {
        aber_n0 = 1i32 != 0
    }
    /* watch if the 3 latest sync point were aberrant or not */
    if aber_n0 as libc::c_int == 0i32 {
        /* value no aberrant -> sync with smoothed slope */
        (*ref_0).systime = time(0 as *mut time_t);
        (*ref_0).count_us = count_us;
        (*ref_0).utc.tv_sec = utc.tv_sec;
        (*ref_0).utc.tv_nsec = utc.tv_nsec;
        (*ref_0).gps.tv_sec = gps_time.tv_sec;
        (*ref_0).gps.tv_nsec = gps_time.tv_nsec;
        (*ref_0).xtal_err = slope;
        aber_min2 = aber_min1;
        aber_min1 = aber_n0;
        return 0i32;
    } else if aber_n0 as libc::c_int != 0
        && aber_min1 as libc::c_int != 0
        && aber_min2 as libc::c_int != 0
    {
        /* 3 successive aberrant values -> sync reset (keep xtal_err) */
        (*ref_0).systime = time(0 as *mut time_t);
        (*ref_0).count_us = count_us;
        (*ref_0).utc.tv_sec = utc.tv_sec;
        (*ref_0).utc.tv_nsec = utc.tv_nsec;
        (*ref_0).gps.tv_sec = gps_time.tv_sec;
        (*ref_0).gps.tv_nsec = gps_time.tv_nsec;
        /* reset xtal_err only if the present value is out of range */
        if (*ref_0).xtal_err > 1.00001f64 || (*ref_0).xtal_err < 0.99999f64 {
            (*ref_0).xtal_err = 1.0f64
        }
        aber_min2 = aber_min1;
        aber_min1 = aber_n0;
        return 0i32;
    } else {
        /* only 1 or 2 successive aberrant values -> ignore and return an error */
        aber_min2 = aber_min1;
        aber_min1 = aber_n0;
        return -1i32;
    };
}
/* *
@brief Convert concentrator timestamp counter value to UTC time

@param ref time reference structure required for time conversion
@param count_us internal timestamp counter of the LoRa concentrator
@param utc pointer to store UTC time, with ns precision (leap seconds ignored)
@return success if the function was able to convert timestamp to UTC

This function is typically used when a packet is received to transform the
internal counter-based timestamp in an absolute timestamp with an accuracy in
the order of a couple microseconds (ns resolution).
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_cnt2utc(
    mut ref_0: tref,
    mut count_us: uint32_t,
    mut utc: *mut timespec,
) -> libc::c_int {
    let mut delta_sec: libc::c_double = 0.;
    let mut intpart: libc::c_double = 0.;
    let mut fractpart: libc::c_double = 0.;
    let mut tmp: libc::c_long = 0;
    if utc.is_null() {
        return -1i32;
    }
    if ref_0.systime == 0i32 as libc::c_long
        || ref_0.xtal_err > 1.00001f64
        || ref_0.xtal_err < 0.99999f64
    {
        return -1i32;
    }
    /* calculate delta in seconds between reference count_us and target count_us */
    delta_sec = count_us.wrapping_sub(ref_0.count_us) as libc::c_double / (1E6f64 * ref_0.xtal_err);
    /* now add that delta to reference UTC time */
    fractpart = modf(delta_sec, &mut intpart); /* must carry one second */
    tmp = ref_0.utc.tv_nsec + (fractpart * 1E9f64) as libc::c_long;
    if tmp < 1E9f64 as libc::c_long {
        /* the nanosecond part doesn't overflow */
        (*utc).tv_sec = ref_0.utc.tv_sec + intpart as time_t;
        (*utc).tv_nsec = tmp
    } else {
        (*utc).tv_sec = ref_0.utc.tv_sec + intpart as time_t + 1i32 as libc::c_long;
        (*utc).tv_nsec = tmp - 1E9f64 as libc::c_long
    }
    return 0i32;
}
/* *
@brief Convert UTC time to concentrator timestamp counter value

@param ref time reference structure required for time conversion
@param utc UTC time, with ns precision (leap seconds are ignored)
@param count_us pointer to store internal timestamp counter of LoRa concentrator
@return success if the function was able to convert UTC to timestamp

This function is typically used when a packet must be sent at an accurate time
(eg. to send a piggy-back response after receiving a packet from a node) to
transform an absolute UTC time into a matching internal concentrator timestamp.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_utc2cnt(
    mut ref_0: tref,
    mut utc: timespec,
    mut count_us: *mut uint32_t,
) -> libc::c_int {
    let mut delta_sec: libc::c_double = 0.;
    if count_us.is_null() {
        return -1i32;
    }
    if ref_0.systime == 0i32 as libc::c_long
        || ref_0.xtal_err > 1.00001f64
        || ref_0.xtal_err < 0.99999f64
    {
        return -1i32;
    }
    /* calculate delta in seconds between reference utc and target utc */
    delta_sec = (utc.tv_sec - ref_0.utc.tv_sec) as libc::c_double;
    delta_sec += 1E-9f64 * (utc.tv_nsec - ref_0.utc.tv_nsec) as libc::c_double;
    /* now convert that to internal counter tics and add that to reference counter value */
    *count_us = ref_0
        .count_us
        .wrapping_add((delta_sec * 1E6f64 * ref_0.xtal_err) as uint32_t);
    return 0i32;
}
/* *
@brief Convert concentrator timestamp counter value to GPS time

@param ref time reference structure required for time conversion
@param count_us internal timestamp counter of the LoRa concentrator
@param gps_time pointer to store GPS time, with ns precision (leap seconds ignored)
@return success if the function was able to convert timestamp to GPS time

This function is typically used when a packet is received to transform the
internal counter-based timestamp in an absolute timestamp with an accuracy in
the order of a millisecond.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_cnt2gps(
    mut ref_0: tref,
    mut count_us: uint32_t,
    mut gps_time: *mut timespec,
) -> libc::c_int {
    let mut delta_sec: libc::c_double = 0.;
    let mut intpart: libc::c_double = 0.;
    let mut fractpart: libc::c_double = 0.;
    let mut tmp: libc::c_long = 0;
    if gps_time.is_null() {
        return -1i32;
    }
    if ref_0.systime == 0i32 as libc::c_long
        || ref_0.xtal_err > 1.00001f64
        || ref_0.xtal_err < 0.99999f64
    {
        return -1i32;
    }
    /* calculate delta in milliseconds between reference count_us and target count_us */
    delta_sec = count_us.wrapping_sub(ref_0.count_us) as libc::c_double / (1E6f64 * ref_0.xtal_err);
    /* now add that delta to reference GPS time */
    fractpart = modf(delta_sec, &mut intpart); /* must carry one second */
    tmp = ref_0.gps.tv_nsec + (fractpart * 1E9f64) as libc::c_long;
    if tmp < 1E9f64 as libc::c_long {
        /* the nanosecond part doesn't overflow */
        (*gps_time).tv_sec = ref_0.gps.tv_sec + intpart as time_t;
        (*gps_time).tv_nsec = tmp
    } else {
        (*gps_time).tv_sec = ref_0.gps.tv_sec + intpart as time_t + 1i32 as libc::c_long;
        (*gps_time).tv_nsec = tmp - 1E9f64 as libc::c_long
    }
    return 0i32;
}
/* *
@brief Convert GPS time to concentrator timestamp counter value

@param ref time reference structure required for time conversion
@param gps_time GPS time, with ns precision (leap seconds are ignored)
@param count_us pointer to store internal timestamp counter of LoRa concentrator
@return success if the function was able to convert GPS time to timestamp

This function is typically used when a packet must be sent at an accurate time
(eg. to send a piggy-back response after receiving a packet from a node) to
transform an absolute GPS time into a matching internal concentrator timestamp.
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_gps2cnt(
    mut ref_0: tref,
    mut gps_time: timespec,
    mut count_us: *mut uint32_t,
) -> libc::c_int {
    let mut delta_sec: libc::c_double = 0.;
    if count_us.is_null() {
        return -1i32;
    }
    if ref_0.systime == 0i32 as libc::c_long
        || ref_0.xtal_err > 1.00001f64
        || ref_0.xtal_err < 0.99999f64
    {
        return -1i32;
    }
    /* calculate delta in seconds between reference gps time and target gps time */
    delta_sec = (gps_time.tv_sec - ref_0.gps.tv_sec) as libc::c_double;
    delta_sec += 1E-9f64 * (gps_time.tv_nsec - ref_0.gps.tv_nsec) as libc::c_double;
    /* now convert that to internal counter tics and add that to reference counter value */
    *count_us = ref_0
        .count_us
        .wrapping_add((delta_sec * 1E6f64 * ref_0.xtal_err) as uint32_t);
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
