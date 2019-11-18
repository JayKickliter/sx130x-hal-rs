use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    /*
     / _____)             _              | |
    ( (____  _____ ____ _| |_ _____  ____| |__
     \____ \| ___ |    (_   _) ___ |/ ___)  _ \
     _____) ) ____| | | || |_| ____( (___| | | |
    (______/|_____)_|_|_| \__)_____)\____)_| |_|
      (C)2019 Semtech

    Description:
        LoRa concentrator HAL common auxiliary functions

    License: Revised BSD License, see LICENSE.TXT file include in the project
    */
    /* -------------------------------------------------------------------------- */
    /* --- DEPENDANCIES --------------------------------------------------------- */
    /* library configuration options (dynamically generated) */
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC MACROS -------------------------------------------------------- */
    /* *
    @brief Get a particular bit value from a byte
    @param b [in]   Any byte from which we want a bit value
    @param p [in]   Position of the bit in the byte [0..7]
    @param n [in]   Number of bits we want to get
    @return The value corresponding the requested bits
    */
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief Wait for a certain time (millisecond accuracy)
    @param t number of milliseconds to wait.
    */
    #[no_mangle]
    fn wait_ms(t: libc::c_ulong);
    /* -------------------------------------------------------------------------- */
    /* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
    #[no_mangle]
    static mut lgw_spi_target: *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: __u64,
    pub rx_buf: __u64,
    pub len: __u32,
    pub speed_hz: __u32,
    pub delay_usecs: __u16,
    pub bits_per_word: __u8,
    pub cs_change: __u8,
    pub tx_nbits: __u8,
    pub rx_nbits: __u8,
    pub pad: __u16,
}
pub type sx1250_op_code_t = libc::c_uint;
pub const GET_DEVICE_ERRORS: sx1250_op_code_t = 23;
pub const SET_FS: sx1250_op_code_t = 193;
pub const SET_REGULATORMODE: sx1250_op_code_t = 150;
pub const GET_STATUS: sx1250_op_code_t = 192;
pub const SET_TXCONTINUOUSPREAMBLE: sx1250_op_code_t = 210;
pub const SET_TXCONTINUOUSWAVE: sx1250_op_code_t = 209;
pub const WRITE_REGISTER: sx1250_op_code_t = 13;
pub const WRITE_BUFFER: sx1250_op_code_t = 14;
pub const SET_TX_PARAMS: sx1250_op_code_t = 142;
pub const SET_TX: sx1250_op_code_t = 131;
pub const SET_RX: sx1250_op_code_t = 130;
pub const SET_STANDBY: sx1250_op_code_t = 128;
pub const SET_SLEEP: sx1250_op_code_t = 132;
pub const SET_BUFFER_BASE_ADDRESS: sx1250_op_code_t = 143;
pub const SET_RF_FREQUENCY: sx1250_op_code_t = 134;
pub const SET_PACKET_TYPE: sx1250_op_code_t = 138;
pub const SET_PACKET_PARAMS: sx1250_op_code_t = 140;
pub const SET_PA_CONFIG: sx1250_op_code_t = 149;
pub const SET_MODULATION_PARAMS: sx1250_op_code_t = 139;
pub const SET_DIO_IRQ_PARAMS: sx1250_op_code_t = 8;
pub const READ_REGISTER: sx1250_op_code_t = 29;
pub const READ_BUFFER: sx1250_op_code_t = 30;
pub const GET_PACKET_STATUS: sx1250_op_code_t = 20;
pub const GET_RX_BUFFER_STATUS: sx1250_op_code_t = 19;
pub const GET_IRQ_STATUS: sx1250_op_code_t = 18;
pub const SET_RFSWITCHMODE: sx1250_op_code_t = 157;
pub const STOP_TIMER_ON_PREAMBLE: sx1250_op_code_t = 159;
pub const CLR_IRQ_STATUS: sx1250_op_code_t = 2;
pub const CALIBRATE_IMAGE: sx1250_op_code_t = 152;
pub const CALIBRATE: sx1250_op_code_t = 137;
pub type C2RustUnnamed = libc::c_uint;
pub const STDBY_XOSC: C2RustUnnamed = 1;
pub const STDBY_RC: C2RustUnnamed = 0;
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* ! generic pointer to the SPI device */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn sx1250_write_command(
    mut rf_chain: uint8_t,
    mut op_code: sx1250_op_code_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0; /* header + op_code */
    let mut cmd_size: libc::c_int = 2i32;
    let vla = (cmd_size + size as libc::c_int) as usize;
    let mut out_buf: Vec<uint8_t> = ::std::vec::from_elem(0, vla);
    let mut command_size: uint8_t = 0;
    let mut k: spi_ioc_transfer = spi_ioc_transfer {
        tx_buf: 0,
        rx_buf: 0,
        len: 0,
        speed_hz: 0,
        delay_usecs: 0,
        bits_per_word: 0,
        cs_change: 0,
        tx_nbits: 0,
        rx_nbits: 0,
        pad: 0,
    };
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* wait BUSY */
    wait_ms(1i32 as libc::c_ulong);
    /* check input variables */
    if lgw_spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    spi_device = *(lgw_spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    *out_buf.as_mut_ptr().offset(0) = if rf_chain as libc::c_int == 0i32 {
        0x1i32
    } else {
        0x2i32
    } as uint8_t;
    *out_buf.as_mut_ptr().offset(1) = op_code as uint8_t;
    i = 0i32;
    while i < size as libc::c_int {
        *out_buf.as_mut_ptr().offset((cmd_size + i) as isize) = *data.offset(i as isize);
        i += 1
    }
    command_size = (cmd_size + size as libc::c_int) as uint8_t;
    /* I/O transaction */
    memset(
        &mut k as *mut spi_ioc_transfer as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<spi_ioc_transfer>() as libc::c_ulong,
    ); /* clear k */
    k.tx_buf = out_buf.as_mut_ptr() as libc::c_ulong as __u64;
    k.len = command_size as __u32;
    k.speed_hz = 2000000i32 as __u32;
    k.cs_change = 0i32 as __u8;
    k.bits_per_word = 8i32 as __u8;
    a = ioctl(
        spi_device,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut k as *mut spi_ioc_transfer,
    );
    /* determine return code */
    if a != k.len as libc::c_int {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1250_read_command(
    mut rf_chain: uint8_t,
    mut op_code: sx1250_op_code_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0; /* header + op_code + NOP */
    let mut cmd_size: libc::c_int = 2i32;
    let vla = (cmd_size + size as libc::c_int) as usize;
    let mut out_buf: Vec<uint8_t> = ::std::vec::from_elem(0, vla);
    let mut command_size: uint8_t = 0;
    let vla_0 = ((vla * ::std::mem::size_of::<uint8_t>()) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uint8_t>() as libc::c_ulong) as usize;
    let mut in_buf: Vec<uint8_t> = ::std::vec::from_elem(0, vla_0);
    let mut k: spi_ioc_transfer = spi_ioc_transfer {
        tx_buf: 0,
        rx_buf: 0,
        len: 0,
        speed_hz: 0,
        delay_usecs: 0,
        bits_per_word: 0,
        cs_change: 0,
        tx_nbits: 0,
        rx_nbits: 0,
        pad: 0,
    };
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* wait BUSY */
    wait_ms(1i32 as libc::c_ulong);
    /* check input variables */
    if lgw_spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    if data.is_null() {
        return -1i32;
    }
    spi_device = *(lgw_spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    *out_buf.as_mut_ptr().offset(0) = if rf_chain as libc::c_int == 0i32 {
        0x1i32
    } else {
        0x2i32
    } as uint8_t;
    *out_buf.as_mut_ptr().offset(1) = op_code as uint8_t;
    i = 0i32;
    while i < size as libc::c_int {
        *out_buf.as_mut_ptr().offset((cmd_size + i) as isize) = *data.offset(i as isize);
        i += 1
    }
    command_size = (cmd_size + size as libc::c_int) as uint8_t;
    /* I/O transaction */
    memset(
        &mut k as *mut spi_ioc_transfer as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<spi_ioc_transfer>() as libc::c_ulong,
    ); /* clear k */
    k.tx_buf = out_buf.as_mut_ptr() as libc::c_ulong as __u64;
    k.rx_buf = in_buf.as_mut_ptr() as libc::c_ulong as __u64;
    k.len = command_size as __u32;
    k.cs_change = 0i32 as __u8;
    a = ioctl(
        spi_device,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut k as *mut spi_ioc_transfer,
    );
    /* determine return code */
    if a != k.len as libc::c_int {
        return -1i32;
    } else {
        //*data = in_buf[command_size - 1];
        memcpy(
            data as *mut libc::c_void,
            in_buf.as_mut_ptr().offset(cmd_size as isize) as *const libc::c_void,
            size as libc::c_ulong,
        );
        return 0i32;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1250_calibrate(
    mut rf_chain: uint8_t,
    mut freq_hz: uint32_t,
) -> libc::c_int {
    let mut buff: [uint8_t; 16] = [0; 16];
    buff[0] = 0i32 as uint8_t;
    sx1250_read_command(rf_chain, GET_STATUS, buff.as_mut_ptr(), 1i32 as uint16_t);
    /* Run calibration */
    if freq_hz as libc::c_double > 430E6f64 && (freq_hz as libc::c_double) < 440E6f64 {
        buff[0] = 0x6bi32 as uint8_t;
        buff[1] = 0x6fi32 as uint8_t
    } else if freq_hz as libc::c_double > 470E6f64 && (freq_hz as libc::c_double) < 510E6f64 {
        buff[0] = 0x75i32 as uint8_t;
        buff[1] = 0x81i32 as uint8_t
    } else if freq_hz as libc::c_double > 779E6f64 && (freq_hz as libc::c_double) < 787E6f64 {
        buff[0] = 0xc1i32 as uint8_t;
        buff[1] = 0xc5i32 as uint8_t
    } else if freq_hz as libc::c_double > 863E6f64 && (freq_hz as libc::c_double) < 870E6f64 {
        buff[0] = 0xd7i32 as uint8_t;
        buff[1] = 0xdbi32 as uint8_t
    } else if freq_hz as libc::c_double > 902E6f64 && (freq_hz as libc::c_double) < 928E6f64 {
        buff[0] = 0xe1i32 as uint8_t;
        buff[1] = 0xe9i32 as uint8_t
    } else {
        printf(
            b"ERROR: failed to calibrate sx1250 radio, frequency range not supported (%u)\n\x00"
                as *const u8 as *const libc::c_char,
            freq_hz,
        );
        return -1i32;
    }
    sx1250_write_command(
        rf_chain,
        CALIBRATE_IMAGE,
        buff.as_mut_ptr(),
        2i32 as uint16_t,
    );
    /* Wait for calibration to complete */
    wait_ms(10i32 as libc::c_ulong);
    buff[0] = 0i32 as uint8_t;
    buff[1] = 0i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_read_command(
        rf_chain,
        GET_DEVICE_ERRORS,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    if buff[2] as libc::c_int >> 4i32 & (1i32 << 1i32) - 1i32 != 0i32 {
        printf(b"ERROR: sx1250 Image Calibration Error\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1250_setup(mut rf_chain: uint8_t, mut freq_hz: uint32_t) -> libc::c_int {
    let mut freq_reg: int32_t = 0;
    let mut buff: [uint8_t; 16] = [0; 16];
    /* Set Radio in Standby for calibrations */
    buff[0] = STDBY_RC as libc::c_int as uint8_t;
    sx1250_write_command(rf_chain, SET_STANDBY, buff.as_mut_ptr(), 1i32 as uint16_t);
    wait_ms(10i32 as libc::c_ulong);
    /* Get status to check Standby mode has been properly set */
    buff[0] = 0i32 as uint8_t;
    sx1250_read_command(rf_chain, GET_STATUS, buff.as_mut_ptr(), 1i32 as uint16_t);
    if (buff[0] as libc::c_int >> 4i32 & (1i32 << 3i32) - 1i32) as uint8_t as libc::c_int != 0x2i32
    {
        printf(
            b"ERROR: Failed to set SX1250_%u in STANDBY_RC mode\n\x00" as *const u8
                as *const libc::c_char,
            rf_chain as libc::c_int,
        );
        return -1i32;
    }
    /* Run all calibrations (TCXO) */
    buff[0] = 0x7fi32 as uint8_t;
    sx1250_write_command(rf_chain, CALIBRATE, buff.as_mut_ptr(), 1i32 as uint16_t);
    wait_ms(10i32 as libc::c_ulong);
    /* Set Radio in Standby with XOSC ON */
    buff[0] = STDBY_XOSC as libc::c_int as uint8_t;
    sx1250_write_command(rf_chain, SET_STANDBY, buff.as_mut_ptr(), 1i32 as uint16_t);
    wait_ms(10i32 as libc::c_ulong);
    /* Get status to check Standby mode has been properly set */
    buff[0] = 0i32 as uint8_t;
    sx1250_read_command(rf_chain, GET_STATUS, buff.as_mut_ptr(), 1i32 as uint16_t);
    if (buff[0] as libc::c_int >> 4i32 & (1i32 << 3i32) - 1i32) as uint8_t as libc::c_int != 0x3i32
    {
        printf(
            b"ERROR: Failed to set SX1250_%u in STANDBY_XOSC mode\n\x00" as *const u8
                as *const libc::c_char,
            rf_chain as libc::c_int,
        );
        return -1i32;
    }
    /* Set Bitrate to maximum (to lower TX to FS switch time) */
    buff[0] = 0x6i32 as uint8_t;
    buff[1] = 0xa1i32 as uint8_t;
    buff[2] = 0x1i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    buff[0] = 0x6i32 as uint8_t;
    buff[1] = 0xa2i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    buff[0] = 0x6i32 as uint8_t;
    buff[1] = 0xa3i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    /* Configure DIO for Rx */
    buff[0] = 0x5i32 as uint8_t; /* Drive strength to min */
    buff[1] = 0x82i32 as uint8_t; /* Input enable, all disabled */
    buff[2] = 0i32 as uint8_t; /* No pull up */
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    ); /* No pull down */
    buff[0] = 0x5i32 as uint8_t; /* Output enable, all enabled */
    buff[1] = 0x83i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    buff[0] = 0x5i32 as uint8_t;
    buff[1] = 0x84i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    buff[0] = 0x5i32 as uint8_t;
    buff[1] = 0x85i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    buff[0] = 0x5i32 as uint8_t;
    buff[1] = 0x80i32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    /* Set fix gain (??) */
    buff[0] = 0x8i32 as uint8_t;
    buff[1] = 0xb6i32 as uint8_t;
    buff[2] = 0x2ai32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    /* Set frequency */
    freq_reg = (freq_hz as uint64_t)
        .wrapping_mul((1i32 << 25i32) as libc::c_ulong)
        .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t as int32_t;
    buff[0] = (freq_reg >> 24i32) as uint8_t;
    buff[1] = (freq_reg >> 16i32) as uint8_t;
    buff[2] = (freq_reg >> 8i32) as uint8_t;
    buff[3] = (freq_reg >> 0i32) as uint8_t;
    sx1250_write_command(
        rf_chain,
        SET_RF_FREQUENCY,
        buff.as_mut_ptr(),
        4i32 as uint16_t,
    );
    /* Set frequency offset to 0 */
    buff[0] = 0x8i32 as uint8_t;
    buff[1] = 0x8fi32 as uint8_t;
    buff[2] = 0i32 as uint8_t;
    buff[3] = 0i32 as uint8_t;
    buff[4] = 0i32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        5i32 as uint16_t,
    );
    /* Set Radio in Rx mode, necessary to give a clock to SX1302 */
    buff[0] = 0xffi32 as uint8_t; /* Rx Continuous */
    buff[1] = 0xffi32 as uint8_t; /* FPGA_MODE_RX */
    buff[2] = 0xffi32 as uint8_t;
    sx1250_write_command(rf_chain, SET_RX, buff.as_mut_ptr(), 3i32 as uint16_t);
    buff[0] = 0x5i32 as uint8_t;
    buff[1] = 0x87i32 as uint8_t;
    buff[2] = 0xbi32 as uint8_t;
    sx1250_write_command(
        rf_chain,
        WRITE_REGISTER,
        buff.as_mut_ptr(),
        3i32 as uint16_t,
    );
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
