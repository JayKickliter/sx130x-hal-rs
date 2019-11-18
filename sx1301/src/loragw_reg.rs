use libc;
extern "C" {
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief LoRa concentrator SPI setup (configure I/O and peripherals)
    @param spi_target_ptr pointer on a generic pointer to SPI target (implementation dependant)
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_open(spi_target_ptr: *mut *mut libc::c_void) -> libc::c_int;
    /* *
    @brief LoRa concentrator SPI close
    @param spi_target generic pointer to SPI target (implementation dependant)
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_close(spi_target: *mut libc::c_void) -> libc::c_int;
    /* *
    @brief LoRa concentrator SPI single-byte write
    @param spi_target generic pointer to SPI target (implementation dependant)
    @param address 7-bit register address
    @param data data byte to write
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_w(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        address: uint8_t,
        data: uint8_t,
    ) -> libc::c_int;
    /* *
    @brief LoRa concentrator SPI single-byte read
    @param spi_target generic pointer to SPI target (implementation dependant)
    @param address 7-bit register address
    @param data data byte to write
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_r(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        address: uint8_t,
        data: *mut uint8_t,
    ) -> libc::c_int;
    /* *
    @brief LoRa concentrator SPI burst (multiple-byte) write
    @param spi_target generic pointer to SPI target (implementation dependant)
    @param address 7-bit register address
    @param data pointer to byte array that will be sent to the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_wb(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        address: uint8_t,
        data: *mut uint8_t,
        size: uint16_t,
    ) -> libc::c_int;
    /* *
    @brief LoRa concentrator SPI burst (multiple-byte) read
    @param spi_target generic pointer to SPI target (implementation dependant)
    @param address 7-bit register address
    @param data pointer to byte array that will be written from the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_rb(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        address: uint8_t,
        data: *mut uint8_t,
        size: uint16_t,
    ) -> libc::c_int;
    /* *
    @brief LoRa concentrator FPGA configuration
    @param tx_notch_freq TX notch filter frequency, in Hertz
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_fpga_configure(tx_notch_freq: uint32_t) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
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
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2013 Semtech-Cycleo

Description:
    Functions used to handle a single LoRa concentrator.
    Registers are addressed by name.
    Multi-bytes registers are handled automatically.
    Read-modify-write is handled automatically.

License: Revised BSD License, see LICENSE.TXT file include in the project
Maintainer: Sylvain Miermont
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* library configuration options (dynamically generated) */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED TYPES ------------------------------------------------ */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_reg_s {
    pub page: int8_t,
    pub addr: uint8_t,
    pub offs: uint8_t,
    pub sign: bool,
    pub leng: uint8_t,
    pub rdon: bool,
    pub dflt: int32_t,
    /* !< register default value */
}
#[no_mangle]
pub static mut FPGA_VERSION: [uint8_t; 2] = [31i32 as uint8_t, 33i32 as uint8_t];
/* several versions could be supported */
/*
auto generated register mapping for C code : 11-Jul-2013 13:20:40
this file contains autogenerated C struct used to access the LoRa register from the Primer firmware
this file is autogenerated from registers description
293 registers are defined
*/
#[no_mangle]
pub static mut loregs: [lgw_reg_s; 326] = [
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 0i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 0i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 1i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 103i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 2i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 4i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 5i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 6i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 7i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 8i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 9i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 10i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 11i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 12i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 14i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 15i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 16i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 16i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 16i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 16i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 17i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 17i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 18i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 18i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 18i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 18i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 19i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 19i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 20i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 20i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 20i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 20i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 20i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 21i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 22i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 23i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 24i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 25i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 25i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 25i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 26i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 27i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 28i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 29i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 30i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 31i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 32i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 125i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 126i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 127i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 34i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 35i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 240i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 36i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: -384i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 38i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: -128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 40i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 384i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 44i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: -384i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 46i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: -128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 48i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 50i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 384i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 52i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 54i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 13i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 65i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 66i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 67i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 68i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 69i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 70i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 71i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 72i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 73i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 74i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 74i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 75i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 75i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 76i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 76i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 78i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 78i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 81i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 12i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 83i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 12i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4092i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 85i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 86i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 87i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 88i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 89i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 89i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 90i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 90i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 91i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 92i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 29i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 93i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 9i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 94i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 94i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 94i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 95i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 95i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 96i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 98i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 98i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 99i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 99i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 100i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 101i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 102i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 103i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 104i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 104i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 105i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 105i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 105i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 106i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 107i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 108i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 109i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 100i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 110i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 100i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 111i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 112i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 113i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 114i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 115i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 116i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 116i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 117i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 34i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 36i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 36i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 37i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 39i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 40i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 41i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 41i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 41i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 41i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 44i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 45i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 46i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 46i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 47i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 49i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 49i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 50i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 51i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 52i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 29i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 53i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 36i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 54i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 12i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 56i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 56i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 56i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 57i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 58i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 58i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 58i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 59i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 59i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 59i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 60i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 60i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 61i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 62i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 63i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 63i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 63i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 63i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 65i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 65i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 66i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 68i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 32i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 72i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 32i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 76i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 80i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 81i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 82i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 83i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 10i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 34i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 35i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 37i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 38i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 39i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 40i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 42i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 43i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 44i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 45i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 45i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 46i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 46i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 47i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 47i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 47i32 as uint8_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 47i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 48i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 51i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 52i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 54i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 56i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 57i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 58i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 59i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 60i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 61i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 62i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 63i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 64i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 65i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 66i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 68i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 70i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 32i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 74i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 74i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 75i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 75i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 76i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 76i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 77i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 78i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 79i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 79i32 as uint8_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 80i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 81i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 82i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 82i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 82i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 83i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 84i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 84i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 84i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 84i32 as uint8_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 84i32 as uint8_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 85i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 86i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 87i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 88i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 89i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 89i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 90i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 91i32 as uint8_t,
            offs: 2i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 92i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 94i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 95i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 96i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 2i32 as int8_t,
            addr: 97i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 1i32 as int8_t,
            addr: 33i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
];
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
static mut lgw_regpage: libc::c_int = -1i32;
/* ! keep the value of the register page selected */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
#[no_mangle]
pub static mut lgw_spi_target: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
/* ! generic pointer to the SPI device */
#[no_mangle]
pub static mut lgw_spi_mux_mode: uint8_t = 0i32 as uint8_t;
/* ! current SPI mux mode used */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn page_switch(mut target: uint8_t) -> libc::c_int {
    lgw_regpage = 0x3i32 & target as libc::c_int;
    lgw_spi_w(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        0i32 as uint8_t,
        lgw_regpage as uint8_t,
    );
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn check_fpga_version(mut version: uint8_t) -> bool {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < ::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong as libc::c_int {
        if FPGA_VERSION[i as usize] as libc::c_int == version as libc::c_int {
            return 1i32 != 0;
        }
        i += 1
    }
    return 0i32 != 0;
}
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED FUNCTIONS -------------------------------------------- */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn reg_w_align32(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut r: lgw_reg_s,
    mut reg_value: int32_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut size_byte: libc::c_int = 0;
    let mut buf: [uint8_t; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"\x00\x00\x00\x00");
    if r.leng as libc::c_int == 8i32 && r.offs as libc::c_int == 0i32 {
        /* direct write */
        spi_stat += lgw_spi_w(
            spi_target,
            spi_mux_mode,
            spi_mux_target,
            r.addr,
            reg_value as uint8_t,
        )
    } else if r.offs as libc::c_int + r.leng as libc::c_int <= 8i32 {
        /* single-byte read-modify-write, offs:[0-7], leng:[1-7] */
        spi_stat += lgw_spi_r(
            spi_target,
            spi_mux_mode,
            spi_mux_target,
            r.addr,
            &mut *buf.as_mut_ptr().offset(0),
        ); /* bit mask */
        buf[1] = ((1i32 << r.leng as libc::c_int) - 1i32 << r.offs as libc::c_int) as uint8_t; /* new data offsetted */
        buf[2] = ((reg_value as uint8_t as libc::c_int) << r.offs as libc::c_int) as uint8_t; /* mixing old & new data */
        buf[3] = (!(buf[1] as libc::c_int) & buf[0] as libc::c_int
            | buf[1] as libc::c_int & buf[2] as libc::c_int) as uint8_t;
        spi_stat += lgw_spi_w(spi_target, spi_mux_mode, spi_mux_target, r.addr, buf[3])
    } else if r.offs as libc::c_int == 0i32
        && r.leng as libc::c_int > 0i32
        && r.leng as libc::c_int <= 32i32
    {
        /* multi-byte direct write routine */
        size_byte = (r.leng as libc::c_int + 7i32) / 8i32;
        i = 0i32;
        while i < size_byte {
            /* add a byte if it's not an exact multiple of 8 */
            /* write the register in one burst */
            /* big endian register file for a file on N bytes
            Least significant byte is stored in buf[0], most one in buf[N-1] */
            buf[i as usize] = (0xffi32 & reg_value) as uint8_t;
            reg_value = reg_value >> 8i32;
            i += 1
        }
        spi_stat += lgw_spi_wb(
            spi_target,
            spi_mux_mode,
            spi_mux_target,
            r.addr,
            buf.as_mut_ptr(),
            size_byte as uint16_t,
        )
    } else {
        /* register spanning multiple memory bytes but with an offset */
        return -1i32;
    }
    return spi_stat;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn reg_r_align32(
    mut spi_target: *mut libc::c_void,
    mut spi_mux_mode: uint8_t,
    mut spi_mux_target: uint8_t,
    mut r: lgw_reg_s,
    mut reg_value: *mut int32_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut bufu: [uint8_t; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"\x00\x00\x00\x00");
    let mut bufs: *mut int8_t = bufu.as_mut_ptr() as *mut int8_t;
    let mut i: libc::c_int = 0;
    let mut size_byte: libc::c_int = 0;
    let mut u: uint32_t = 0i32 as uint32_t;
    if r.offs as libc::c_int + r.leng as libc::c_int <= 8i32 {
        /* read one byte, then shift and mask bits to get reg value with sign extension if needed */
        spi_stat += lgw_spi_r(
            spi_target,
            spi_mux_mode,
            spi_mux_target,
            r.addr,
            &mut *bufu.as_mut_ptr().offset(0),
        ); /* left-align the data */
        bufu[1] = ((bufu[0] as libc::c_int) << 8i32 - r.leng as libc::c_int - r.offs as libc::c_int)
            as uint8_t; /* right align the data with sign extension (ARITHMETIC right shift) */
        if r.sign as libc::c_int == 1i32 {
            *bufs.offset(2) =
                (*bufs.offset(1) as libc::c_int >> 8i32 - r.leng as libc::c_int) as int8_t;
            *reg_value = *bufs.offset(2) as int32_t
        /* signed pointer -> 32b sign extension */
        } else {
            bufu[2] = (bufu[1] as libc::c_int >> 8i32 - r.leng as libc::c_int) as uint8_t;
            *reg_value = bufu[2] as int32_t /* right align the data, no sign extension */
            /* unsigned pointer -> no sign extension */
        }
    } else if r.offs as libc::c_int == 0i32
        && r.leng as libc::c_int > 0i32
        && r.leng as libc::c_int <= 32i32
    {
        size_byte = (r.leng as libc::c_int + 7i32) / 8i32; /* add a byte if it's not an exact multiple of 8 */
        spi_stat += lgw_spi_rb(
            spi_target,
            spi_mux_mode,
            spi_mux_target,
            r.addr,
            bufu.as_mut_ptr(),
            size_byte as uint16_t,
        );
        u = 0i32 as uint32_t;
        i = size_byte - 1i32;
        while i >= 0i32 {
            u = (bufu[i as usize] as uint32_t).wrapping_add(u << 8i32);
            i -= 1
            /* transform a 4-byte array into a 32 bit word */
        } /* left-align the data */
        if r.sign as libc::c_int == 1i32 {
            u = u << 32i32 - r.leng as libc::c_int;
            *reg_value = u as int32_t >> 32i32 - r.leng as libc::c_int
        /* right-align the data with sign extension (ARITHMETIC right shift) */
        } else {
            *reg_value = u as int32_t
            /* unsigned value -> return 'as is' */
        }
    } else {
        /* register spanning multiple memory bytes but with an offset */
        return -1i32;
    }
    return spi_stat;
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Connect LoRa concentrator by opening SPI link
@param spi_only indicates if we only want to create the SPI connexion to the
concentrator, or if we also want to reset it and configure the FPGA (if present)
@param tx_notch_filter TX notch filter frequency to be set in the FPGA (only
used with SX1301AP2 reference design).
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
/* Concentrator connect */
#[no_mangle]
pub unsafe extern "C" fn lgw_connect(
    mut spi_only: bool,
    mut tx_notch_freq: uint32_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut u: uint8_t = 0i32 as uint8_t;
    let mut x: libc::c_int = 0;
    /* check SPI link status */
    if !lgw_spi_target.is_null() {
        lgw_spi_close(lgw_spi_target);
    }
    /* open the SPI link */
    spi_stat = lgw_spi_open(&mut lgw_spi_target);
    if spi_stat != 0i32 {
        return -1i32;
    }
    if spi_only as libc::c_int == 0i32 {
        /* Detect if the gateway has an FPGA with SPI mux header support */
        /* First, we assume there is an FPGA, and try to read its version */
        spi_stat = lgw_spi_r(
            lgw_spi_target,
            0x1i32 as uint8_t,
            0x1i32 as uint8_t,
            loregs[2].addr,
            &mut u,
        );
        if spi_stat != 0i32 {
            return -1i32;
        }
        if check_fpga_version(u) as libc::c_int != 1i32 {
            /* We failed to read expected FPGA version, so let's assume there is no FPGA */
            lgw_spi_mux_mode = 0i32 as uint8_t
        } else {
            lgw_spi_mux_mode = 0x1i32 as uint8_t;
            /* FPGA Soft Reset */
            lgw_spi_w(
                lgw_spi_target,
                lgw_spi_mux_mode,
                0x1i32 as uint8_t,
                0i32 as uint8_t,
                1i32 as uint8_t,
            );
            lgw_spi_w(
                lgw_spi_target,
                lgw_spi_mux_mode,
                0x1i32 as uint8_t,
                0i32 as uint8_t,
                0i32 as uint8_t,
            );
            /* FPGA configure */
            x = lgw_fpga_configure(tx_notch_freq);
            if x != 0i32 {
                return -1i32;
            }
        }
        /* check SX1301 version */
        spi_stat = lgw_spi_r(
            lgw_spi_target,
            lgw_spi_mux_mode,
            0i32 as uint8_t,
            loregs[2].addr,
            &mut u,
        );
        if spi_stat != 0i32 {
            return -1i32;
        }
        if u as libc::c_int != loregs[2].dflt {
            return -1i32;
        }
        /* write 0 to the page/reset register */
        spi_stat = lgw_spi_w(
            lgw_spi_target,
            lgw_spi_mux_mode,
            0i32 as uint8_t,
            loregs[0].addr,
            0i32 as uint8_t,
        );
        if spi_stat != 0i32 {
            return -1i32;
        } else {
            lgw_regpage = 0i32
        }
    }
    return 0i32;
}
/* *
@brief Disconnect LoRa concentrator by closing SPI link
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Concentrator disconnect */
#[no_mangle]
pub unsafe extern "C" fn lgw_disconnect() -> libc::c_int {
    if !lgw_spi_target.is_null() {
        lgw_spi_close(lgw_spi_target);
        lgw_spi_target = 0 as *mut libc::c_void;
        return 0i32;
    } else {
        return -1i32;
    };
}
/* *
@brief Use the soft-reset register to put the concentrator in initial state
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* soft-reset function */
#[no_mangle]
pub unsafe extern "C" fn lgw_soft_reset() -> libc::c_int {
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        return -1i32;
    } /* 1 -> SOFT_RESET bit */
    lgw_spi_w(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0x80i32 as uint8_t,
    ); /* reset the paging static variable */
    lgw_regpage = 0i32;
    return 0i32;
}
/* *
@brief Check if the registers are ok, send diagnostics to stdio/stderr/file
@param f file descriptor to to which the check result will be written
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* register verification */
#[no_mangle]
pub unsafe extern "C" fn lgw_reg_check(mut f: *mut FILE) -> libc::c_int {
    let mut r: lgw_reg_s = lgw_reg_s {
        page: 0,
        addr: 0,
        offs: 0,
        sign: false,
        leng: 0,
        rdon: false,
        dflt: 0,
    };
    let mut read_value: int32_t = 0;
    let mut ok_msg: [libc::c_char; 12] =
        *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"+++MATCH+++\x00");
    let mut notok_msg: [libc::c_char; 15] =
        *::std::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"###MISMATCH###\x00");
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        fprintf(
            f,
            b"ERROR: CONCENTRATOR UNCONNECTED\n\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    fprintf(
        f,
        b"Start of register verification\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0i32;
    while i < 326i32 {
        r = loregs[i as usize];
        lgw_reg_r(i as uint16_t, &mut read_value);
        ptr = if read_value == r.dflt {
            ok_msg.as_mut_ptr()
        } else {
            notok_msg.as_mut_ptr()
        };
        if r.sign as libc::c_int == 1i32 {
            fprintf(
                f,
                b"%s reg number %d read: %d (%x) default: %d (%x)\n\x00" as *const u8
                    as *const libc::c_char,
                ptr,
                i,
                read_value,
                read_value,
                r.dflt,
                r.dflt,
            );
        } else {
            fprintf(
                f,
                b"%s reg number %d read: %u (%x) default: %u (%x)\n\x00" as *const u8
                    as *const libc::c_char,
                ptr,
                i,
                read_value,
                read_value,
                r.dflt,
                r.dflt,
            );
        }
        i += 1
    }
    fprintf(
        f,
        b"End of register verification\n\x00" as *const u8 as *const libc::c_char,
    );
    return 0i32;
}
/* *
@brief LoRa concentrator register write
@param register_id register number in the data structure describing registers
@param reg_value signed value to write to the register (for u32, use cast)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Write to a register addressed by name */
#[no_mangle]
pub unsafe extern "C" fn lgw_reg_w(
    mut register_id: uint16_t,
    mut reg_value: int32_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut r: lgw_reg_s = lgw_reg_s {
        page: 0,
        addr: 0,
        offs: 0,
        sign: false,
        leng: 0,
        rdon: false,
        dflt: 0,
    };
    /* check input parameters */
    if register_id as libc::c_int >= 326i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        return -1i32;
    }
    /* intercept direct access to PAGE_REG & SOFT_RESET */
    if register_id as libc::c_int == 0i32 {
        page_switch(reg_value as uint8_t);
        return 0i32;
    } else {
        if register_id as libc::c_int == 1i32 {
            /* only reset if lsb is 1 */
            if reg_value & 0x1i32 != 0i32 {
                lgw_soft_reset();
            }
            return 0i32;
        }
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    /* select proper register page if needed */
    if r.page as libc::c_int != -1i32 && r.page as libc::c_int != lgw_regpage {
        spi_stat += page_switch(r.page as uint8_t)
    }
    spi_stat += reg_w_align32(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        r,
        reg_value,
    );
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
@brief LoRa concentrator register read
@param register_id register number in the data structure describing registers
@param reg_value pointer to a variable where to write register read value
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Read to a register addressed by name */
#[no_mangle]
pub unsafe extern "C" fn lgw_reg_r(
    mut register_id: uint16_t,
    mut reg_value: *mut int32_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut r: lgw_reg_s = lgw_reg_s {
        page: 0,
        addr: 0,
        offs: 0,
        sign: false,
        leng: 0,
        rdon: false,
        dflt: 0,
    };
    /* check input parameters */
    if reg_value.is_null() {
        return -1i32;
    }
    if register_id as libc::c_int >= 326i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* select proper register page if needed */
    if r.page as libc::c_int != -1i32 && r.page as libc::c_int != lgw_regpage {
        spi_stat += page_switch(r.page as uint8_t)
    }
    spi_stat += reg_r_align32(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        r,
        reg_value,
    );
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
@brief LoRa concentrator register burst write
@param register_id register number in the data structure describing registers
@param data pointer to byte array that will be sent to the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Point to a register by name and do a burst write */
#[no_mangle]
pub unsafe extern "C" fn lgw_reg_wb(
    mut register_id: uint16_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut r: lgw_reg_s = lgw_reg_s {
        page: 0,
        addr: 0,
        offs: 0,
        sign: false,
        leng: 0,
        rdon: false,
        dflt: 0,
    };
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    if register_id as libc::c_int >= 326i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    /* select proper register page if needed */
    if r.page as libc::c_int != -1i32 && r.page as libc::c_int != lgw_regpage {
        spi_stat += page_switch(r.page as uint8_t)
    }
    /* do the burst write */
    spi_stat += lgw_spi_wb(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        r.addr,
        data,
        size,
    );
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
@brief LoRa concentrator register burst read
@param register_id register number in the data structure describing registers
@param data pointer to byte array that will be written from the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Point to a register by name and do a burst read */
#[no_mangle]
pub unsafe extern "C" fn lgw_reg_rb(
    mut register_id: uint16_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut r: lgw_reg_s = lgw_reg_s {
        page: 0,
        addr: 0,
        offs: 0,
        sign: false,
        leng: 0,
        rdon: false,
        dflt: 0,
    };
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    if register_id as libc::c_int >= 326i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() || lgw_regpage < 0i32 {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* select proper register page if needed */
    if r.page as libc::c_int != -1i32 && r.page as libc::c_int != lgw_regpage {
        spi_stat += page_switch(r.page as uint8_t)
    }
    /* do the burst read */
    spi_stat += lgw_spi_rb(
        lgw_spi_target,
        lgw_spi_mux_mode,
        0i32 as uint8_t,
        r.addr,
        data,
        size,
    );
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* --- EOF ------------------------------------------------------------------ */
