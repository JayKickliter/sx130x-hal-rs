use libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
/* FPGA, with spi mux header */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief LoRa concentrator SPI setup (configure I/O and peripherals)
@param spi_target_ptr pointer on a generic pointer to SPI target (implementation dependant)
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
//#define SPI_DEV_PATH    "/dev/spidev32766.0"
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
/* SPI initialization and configuration */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_open(mut spi_target_ptr: *mut *mut libc::c_void) -> libc::c_int {
    let mut spi_device: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dev: libc::c_int = 0;
    let mut a: libc::c_int = 0i32;
    let mut b: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    /* check input variables */
    if spi_target_ptr.is_null() {
        return -1i32;
    }
    /* cannot be null, must point on a void pointer (*spi_target_ptr can be null) */
    /* allocate memory for the device descriptor */
    spi_device = malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
    if spi_device.is_null() {
        return -1i32;
    }
    /* open SPI device */
    dev = open(
        b"/dev/spidev0.0\x00" as *const u8 as *const libc::c_char,
        0o2i32,
    );
    if dev < 0i32 {
        return -1i32;
    }
    /* setting SPI mode to 'mode 0' */
    i = 0i32 | 0i32;
    a = ioctl(
        dev,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    b = ioctl(
        dev,
        (2u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    if a < 0i32 || b < 0i32 {
        close(dev);
        free(spi_device as *mut libc::c_void);
        return -1i32;
    }
    /* setting SPI max clk (in Hz) */
    i = 8000000i32;
    a = ioctl(
        dev,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (4i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u32>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    b = ioctl(
        dev,
        (2u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (4i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u32>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    if a < 0i32 || b < 0i32 {
        close(dev);
        free(spi_device as *mut libc::c_void);
        return -1i32;
    }
    /* setting SPI to MSB first */
    i = 0i32;
    a = ioctl(
        dev,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (2i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    b = ioctl(
        dev,
        (2u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (2i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    if a < 0i32 || b < 0i32 {
        close(dev);
        free(spi_device as *mut libc::c_void);
        return -1i32;
    }
    /* setting SPI to 8 bits per word */
    i = 0i32;
    a = ioctl(
        dev,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (3i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    b = ioctl(
        dev,
        (2u32 << 0i32 + 8i32 + 8i32 + 14i32
            | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
            | (3i32 << 0i32) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u8>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut i as *mut libc::c_int,
    );
    if a < 0i32 || b < 0i32 {
        close(dev);
        return -1i32;
    }
    *spi_device = dev;
    *spi_target_ptr = spi_device as *mut libc::c_void;
    return 0i32;
}
/* *
@brief LoRa concentrator SPI close
@param spi_target generic pointer to SPI target (implementation dependant)
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* SPI release */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_close(mut spi_target: *mut libc::c_void) -> libc::c_int {
    let mut spi_device: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    /* check input variables */
    if spi_target.is_null() {
        return -1i32;
    }
    /* close file & deallocate file descriptor */
    spi_device = *(spi_target as *mut libc::c_int); /* must check that spi_target is not null beforehand */
    a = close(spi_device);
    free(spi_target);
    /* determine return code */
    if a < 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
@brief LoRa concentrator SPI single-byte write
@param spi_target generic pointer to SPI target (implementation dependant)
@param address 7-bit register address
@param data data byte to write
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Simple write */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_w(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut address: uint8_t,
    mut data: uint8_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0;
    let mut out_buf: [uint8_t; 3] = [0; 3];
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
    /* check input variables */
    if spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    (address as libc::c_int & 0x80i32) != 0i32;
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    if spi_mux_mode as libc::c_int == 0x1i32 {
        out_buf[0] = spi_mux_target;
        out_buf[1] = (0x80i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        out_buf[2] = data;
        command_size = 3i32 as uint8_t
    } else {
        out_buf[0] = (0x80i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        out_buf[1] = data;
        command_size = 2i32 as uint8_t
    }
    /* I/O transaction */
    memset(
        &mut k as *mut spi_ioc_transfer as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<spi_ioc_transfer>() as libc::c_ulong,
    ); /* clear k */
    k.tx_buf = out_buf.as_mut_ptr() as libc::c_ulong as __u64;
    k.len = command_size as __u32;
    k.speed_hz = 8000000i32 as __u32;
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
/* *
@brief LoRa concentrator SPI single-byte read
@param spi_target generic pointer to SPI target (implementation dependant)
@param address 7-bit register address
@param data data byte to write
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Simple read */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_r(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut address: uint8_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0;
    let mut out_buf: [uint8_t; 3] = [0; 3];
    let mut command_size: uint8_t = 0;
    let mut in_buf: [uint8_t; 3] = [0; 3];
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
    /* check input variables */
    if spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    (address as libc::c_int & 0x80i32) != 0i32;
    if data.is_null() {
        return -1i32;
    }
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    if spi_mux_mode as libc::c_int == 0x1i32 {
        out_buf[0] = spi_mux_target;
        out_buf[1] = (0i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        out_buf[2] = 0i32 as uint8_t;
        command_size = 3i32 as uint8_t
    } else {
        out_buf[0] = (0i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        out_buf[1] = 0i32 as uint8_t;
        command_size = 2i32 as uint8_t
    }
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
        *data = in_buf[(command_size as libc::c_int - 1i32) as usize];
        return 0i32;
    };
}
/* *
@brief LoRa concentrator SPI burst (multiple-byte) write
@param spi_target generic pointer to SPI target (implementation dependant)
@param address 7-bit register address
@param data pointer to byte array that will be sent to the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Burst (multiple-byte) write */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_wb(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut address: uint8_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0;
    let mut command: [uint8_t; 2] = [0; 2];
    let mut command_size: uint8_t = 0;
    let mut k: [spi_ioc_transfer; 2] = [spi_ioc_transfer {
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
    }; 2];
    let mut size_to_do: libc::c_int = 0;
    let mut chunk_size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut byte_transfered: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    /* check input parameters */
    if spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    (address as libc::c_int & 0x80i32) != 0i32;
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare command byte */
    if spi_mux_mode as libc::c_int == 0x1i32 {
        command[0] = spi_mux_target;
        command[1] = (0x80i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        command_size = 2i32 as uint8_t
    } else {
        command[0] = (0x80i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        command_size = 1i32 as uint8_t
    }
    size_to_do = size as libc::c_int;
    /* I/O transaction */
    memset(
        &mut k as *mut [spi_ioc_transfer; 2] as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[spi_ioc_transfer; 2]>() as libc::c_ulong,
    ); /* clear k */
    k[0].tx_buf = &mut *command.as_mut_ptr().offset(0) as *mut uint8_t as libc::c_ulong as __u64;
    k[0].len = command_size as __u32;
    k[0].cs_change = 0i32 as __u8;
    k[1].cs_change = 0i32 as __u8;
    i = 0i32;
    while size_to_do > 0i32 {
        chunk_size = if size_to_do < 1024i32 {
            size_to_do
        } else {
            1024i32
        };
        offset = i * 1024i32;
        k[1].tx_buf = data.offset(offset as isize) as libc::c_ulong as __u64;
        k[1].len = chunk_size as __u32;
        byte_transfered = (byte_transfered as libc::c_uint).wrapping_add(
            (ioctl(
                spi_device,
                (1u32 << 0i32 + 8i32 + 8i32 + 14i32
                    | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
                    | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
                    | (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        << 0i32 + 8i32 + 8i32,
                &mut k as *mut [spi_ioc_transfer; 2],
            ) as libc::c_uint)
                .wrapping_sub(k[0].len),
        ) as libc::c_int as libc::c_int;
        size_to_do -= chunk_size;
        i += 1
        /* subtract the quantity of data already transferred */
    }
    /* determine return code */
    if byte_transfered != size as libc::c_int {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
@brief LoRa concentrator SPI burst (multiple-byte) read
@param spi_target generic pointer to SPI target (implementation dependant)
@param address 7-bit register address
@param data pointer to byte array that will be written from the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Burst (multiple-byte) read */
#[no_mangle]
pub unsafe extern "C" fn lgw_spi_rb(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut address: uint8_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_device: libc::c_int = 0;
    let mut command: [uint8_t; 2] = [0; 2];
    let mut command_size: uint8_t = 0;
    let mut k: [spi_ioc_transfer; 2] = [spi_ioc_transfer {
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
    }; 2];
    let mut size_to_do: libc::c_int = 0;
    let mut chunk_size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut byte_transfered: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    /* check input parameters */
    if spi_target.is_null() {
        return -1i32;
    } /* must check that spi_target is not null beforehand */
    (address as libc::c_int & 0x80i32) != 0i32;
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare command byte */
    if spi_mux_mode as libc::c_int == 0x1i32 {
        command[0] = spi_mux_target;
        command[1] = (0i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        command_size = 2i32 as uint8_t
    } else {
        command[0] = (0i32 | address as libc::c_int & 0x7fi32) as uint8_t;
        command_size = 1i32 as uint8_t
    }
    size_to_do = size as libc::c_int;
    /* I/O transaction */
    memset(
        &mut k as *mut [spi_ioc_transfer; 2] as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[spi_ioc_transfer; 2]>() as libc::c_ulong,
    ); /* clear k */
    k[0].tx_buf = &mut *command.as_mut_ptr().offset(0) as *mut uint8_t as libc::c_ulong as __u64;
    k[0].len = command_size as __u32;
    k[0].cs_change = 0i32 as __u8;
    k[1].cs_change = 0i32 as __u8;
    i = 0i32;
    while size_to_do > 0i32 {
        chunk_size = if size_to_do < 1024i32 {
            size_to_do
        } else {
            1024i32
        };
        offset = i * 1024i32;
        k[1].rx_buf = data.offset(offset as isize) as libc::c_ulong as __u64;
        k[1].len = chunk_size as __u32;
        byte_transfered = (byte_transfered as libc::c_uint).wrapping_add(
            (ioctl(
                spi_device,
                (1u32 << 0i32 + 8i32 + 8i32 + 14i32
                    | (('k' as i32) << 0i32 + 8i32) as libc::c_uint
                    | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
                    | (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        << 0i32 + 8i32 + 8i32,
                &mut k as *mut [spi_ioc_transfer; 2],
            ) as libc::c_uint)
                .wrapping_sub(k[0].len),
        ) as libc::c_int as libc::c_int;
        size_to_do -= chunk_size;
        i += 1
        /* subtract the quantity of data already transferred */
    }
    /* determine return code */
    if byte_transfered != size as libc::c_int {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* --- EOF ------------------------------------------------------------------ */
