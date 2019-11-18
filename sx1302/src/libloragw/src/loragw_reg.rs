use libc;
extern "C" {
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief LoRa concentrator SPI setup (configure I/O and peripherals)
    @param spidev_path path to the SPI device to be used to connect to the SX1302
    @param spi_target_ptr pointer on a generic pointer to SPI target (implementation dependant)
    @return status of register operation (LGW_SPI_SUCCESS/LGW_SPI_ERROR)
    */
    #[no_mangle]
    fn lgw_spi_open(
        spidev_path: *const libc::c_char,
        spi_target_ptr: *mut *mut libc::c_void,
    ) -> libc::c_int;
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
        spi_mux_target: uint8_t,
        address: uint16_t,
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
        spi_mux_target: uint8_t,
        address: uint16_t,
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
        spi_mux_target: uint8_t,
        address: uint16_t,
        data: *const uint8_t,
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
        spi_mux_target: uint8_t,
        address: uint16_t,
        data: *mut uint8_t,
        size: uint16_t,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Functions used to handle a single LoRa SX1302 concentrator.
    Registers are addressed by name.
    Multi-bytes registers are handled automatically.
    Read-modify-write is handled automatically.

License: Revised BSD License, see LICENSE.TXT file include in the project
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
    pub addr: uint16_t,
    pub offs: uint8_t,
    pub sign: bool,
    pub leng: uint8_t,
    pub rdon: bool,
    pub chck: bool,
    pub dflt: int32_t,
    /* !< register default value */
}
#[no_mangle]
pub static mut loregs: [lgw_reg_s; 1045] = [
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 1i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 1i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 1i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 1i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 2i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 2i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 2i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 5i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 5i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 5i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 16i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5600i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 0i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 0i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 2i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 3i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 3i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 3i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 4i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 4i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 4i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 5i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 6i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5780i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x57c0i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x57c0i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x57c0i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x57c0i32 + 1i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 0i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 187i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 10i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 11i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 18i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 32i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 32i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 32i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 32i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 32i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 33i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 33i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 34i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 35i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 36i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 37i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 108i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 38i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 144i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 39i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 40i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 41i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 42i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 64i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 43i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 64i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 15i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 65i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 65i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 65i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 65i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 65i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 66i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 67i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 20i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 68i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 26i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 69i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 70i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 70i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 70i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 70i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 70i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 71i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 151i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 72i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 35i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 73i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 82i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 74i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 37i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 75i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 86i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 76i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 83i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 77i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 101i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 78i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 100i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 79i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 96i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 96i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 97i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 97i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 97i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 97i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 98i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 98i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 98i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 98i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 98i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 99i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 12i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 100i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 100i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 101i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 101i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 101i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 102i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 103i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 104i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 104i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 104i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 105i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 105i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 105i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 105i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 105i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 106i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 20i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 107i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 108i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 108i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 109i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 109i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 109i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 109i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 110i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 110i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 110i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 111i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 112i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 112i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 112i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5200i32 + 113i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 0i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 187i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 10i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 11i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 18i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 32i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 32i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 32i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 32i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 32i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 33i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 33i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 34i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 35i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 36i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 37i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 108i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 38i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 144i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 39i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 40i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 41i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 42i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 64i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 43i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 64i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 15i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 65i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 65i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 65i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 65i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 65i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 66i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 67i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 20i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 68i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 26i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 69i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 70i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 70i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 70i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 70i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 70i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 71i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 151i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 72i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 35i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 73i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 82i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 74i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 37i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 75i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 86i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 76i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 83i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 77i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 101i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 78i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 100i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 79i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 96i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 96i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 97i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 97i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 97i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 97i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 98i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 98i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 98i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 98i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 98i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 99i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 12i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 100i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 100i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 101i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 101i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 101i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 102i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 103i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 104i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 104i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 104i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 105i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 105i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 105i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 105i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 105i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 106i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 20i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 107i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 108i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 108i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 109i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 109i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 109i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 109i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 110i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 110i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 110i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 111i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 112i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 112i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 112i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5400i32 + 113i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 16i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 18i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5640i32 + 19i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6100i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 30i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 50i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 70i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 80i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 17i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 18i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 19i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 20i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 21i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 15i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 21i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 22i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 23i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 24i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 25i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 25i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 26i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 27i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 32i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 33i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 34i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 35i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 36i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 37i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 38i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 38i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 55i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 39i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 40i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 41i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 42i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 42i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 42i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 43i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 43i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 44i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 45i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 45i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 55i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 46i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 47i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 48i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 49i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 49i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 49i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 50i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 50i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 51i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 52i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 52i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 55i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 53i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 54i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 55i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 56i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 56i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 56i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 57i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 57i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 58i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 59i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 59i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 56i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 60i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 61i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 62i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 63i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 63i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 63i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 64i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 64i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 65i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 66i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 66i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 58i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 67i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 68i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 69i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 70i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 70i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 70i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 71i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 71i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 72i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 73i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 73i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 74i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 75i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 76i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 77i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 77i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 77i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 78i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 78i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 79i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 80i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 80i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 81i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 82i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 83i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 84i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 84i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 84i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 85i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 85i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 86i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 87i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 87i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 88i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 89i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 90i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 91i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 91i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 91i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 92i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 92i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 93i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 96i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 96i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 96i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 96i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 96i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 97i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 97i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 98i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 99i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 100i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 101i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 102i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 103i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 104i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 105i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 106i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 107i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 108i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 109i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 110i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 111i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 112i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 113i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 114i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 19i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 115i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 116i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 116i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 117i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 117i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 117i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 117i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 118i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 118i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 118i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 118i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 119i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 119i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 119i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 119i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 120i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 121i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 122i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 123i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 25i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 124i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 42i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 125i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 125i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 126i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 126i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 126i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 126i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 127i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 127i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 127i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 127i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 128i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 12i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 129i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 129i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 130i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 130i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 131i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 132i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 133i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 133i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 133i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 133i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 134i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 135i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 135i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 135i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 135i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 136i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 137i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 137i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 138i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 139i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 140i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 141i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 142i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 143i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 144i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 144i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 144i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 144i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 144i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 145i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 145i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 145i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 145i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 145i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 146i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 146i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 146i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 147i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 147i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 147i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 148i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 148i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 149i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 149i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 149i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 149i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 150i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 150i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 150i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 150i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 151i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 151i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 152i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 152i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 153i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 153i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 153i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 153i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 153i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 154i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 154i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 154i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 155i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 155i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 155i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 156i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 156i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 157i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 157i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 157i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 157i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 158i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 158i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 158i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 158i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 159i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 159i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 160i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 9i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 161i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 112i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 162i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 163i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 164i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 33i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 165i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 165i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 165i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 165i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 166i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 166i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 166i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 166i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 167i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 167i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 167i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 167i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 168i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 168i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 168i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 168i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 169i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 169i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 170i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 170i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 171i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 171i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 171i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 172i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 32i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 173i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 174i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 174i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 175i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 175i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 175i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 175i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 176i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 176i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 64i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 177i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 178i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 179i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 179i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 179i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 180i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 181i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 182i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 182i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 183i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 183i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 183i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 183i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 184i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 127i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 185i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 185i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 185i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 185i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 186i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 186i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 186i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 186i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 187i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 85i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 188i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 85i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 189i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 85i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 190i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 85i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 191i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 192i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 192i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 192i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 192i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 192i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 193i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 194i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 195i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 21i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 196i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 197i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 198i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 199i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 200i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 201i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 202i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 203i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 204i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 205i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 206i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 207i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 208i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 209i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 210i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 211i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 212i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 213i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 214i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 215i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 216i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 217i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 218i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5800i32 + 219i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 18i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 19i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 20i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 21i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 22i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 23i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 24i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 25i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 26i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 27i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 28i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 29i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 29i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 30i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 30i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 31i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 31i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 32i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 32i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6080i32 + 33i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 1i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 1i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 14i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 14i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 18i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 19i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 20i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 21i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 22i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 23i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 24i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 25i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 26i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 27i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 28i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 29i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 30i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5700i32 + 31i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 2i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 5i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 5i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 5i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 5i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 6i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 6i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 255i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6180i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 3i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 3i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 3i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 3i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 4i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 10i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 11i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 12i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 13i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 14i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 15i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 16i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 17i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 18i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: -3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 19i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 20i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 21i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 19i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 22i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 10i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 23i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 23i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 24i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 24i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 24i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 24i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 25i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 25i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 25i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 25i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 26i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 26i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 26i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 26i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 27i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 28i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 60i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 29i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 6i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 30i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 25i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 31i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 42i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 32i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 32i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 33i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 33i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 34i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 34i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 34i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 34i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 35i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 35i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 35i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 35i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 35i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 36i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 12i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 37i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 37i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 38i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 38i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 39i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 40i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 41i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 41i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 41i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 41i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 42i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 42i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 42i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 42i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 43i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 44i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 44i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 22i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 45i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 45i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 45i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 46i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 47i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 1i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 48i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 48i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 48i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 48i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 48i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 49i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 49i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 49i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 49i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 49i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 50i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 50i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 50i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 51i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 51i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 51i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 52i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 52i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 52i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 53i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 53i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 54i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 9i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 55i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 112i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 56i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 57i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 58i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 33i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 59i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 59i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 59i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 60i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 5i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 60i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 3i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 61i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 61i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 61i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 8i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 62i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 24i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 63i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 48i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 64i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 64i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 65i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 65i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 65i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 65i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 66i32) as uint16_t,
            offs: 7i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 66i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 7i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 55i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 67i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 68i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 11i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 69i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 69i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 69i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 7i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 70i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 70i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 70i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 70i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 71i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 80i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 81i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 128i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 82i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 82i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 82i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 82i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 83i32) as uint16_t,
            offs: 6i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 83i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 83i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 84i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 84i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 1i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 84i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 84i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 85i32) as uint16_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 85i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 86i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 87i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 88i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 89i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 90i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 91i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 92i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 93i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 94i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 95i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 96i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 97i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 98i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 99i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 100i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 101i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 4i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x5b00i32 + 102i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 0i32) as uint16_t,
            offs: 4i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 0i32) as uint16_t,
            offs: 3i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 0i32) as uint16_t,
            offs: 2i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 0i32) as uint16_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 0i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 1i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 2i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 5i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 3i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 4i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 5i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 6i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 7i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 8i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 2i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 1i32 != 0,
            dflt: 2i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: (0x6000i32 + 9i32) as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 1i32 as uint8_t,
            rdon: 1i32 != 0,
            chck: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: 0i32 as int8_t,
            addr: 0i32 as uint16_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 0i32 as uint8_t,
            rdon: 0i32 != 0,
            chck: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
];
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
#[no_mangle]
pub static mut lgw_spi_target: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED FUNCTIONS -------------------------------------------- */
/* ! generic pointer to the SPI device */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn reg_w_align32(
    mut spi_target: *mut libc::c_void,
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
        spi_stat += lgw_spi_w(spi_target, spi_mux_target, r.addr, reg_value as uint8_t)
    } else if r.offs as libc::c_int + r.leng as libc::c_int <= 8i32 {
        /* single-byte read-modify-write, offs:[0-7], leng:[1-7] */
        spi_stat += lgw_spi_r(
            spi_target,
            spi_mux_target,
            r.addr,
            &mut *buf.as_mut_ptr().offset(0),
        ); /* bit mask */
        buf[1] = ((1i32 << r.leng as libc::c_int) - 1i32 << r.offs as libc::c_int) as uint8_t; /* new data offsetted */
        buf[2] = ((reg_value as uint8_t as libc::c_int) << r.offs as libc::c_int) as uint8_t; /* mixing old & new data */
        buf[3] = (!(buf[1] as libc::c_int) & buf[0] as libc::c_int
            | buf[1] as libc::c_int & buf[2] as libc::c_int) as uint8_t;
        spi_stat += lgw_spi_w(spi_target, spi_mux_target, r.addr, buf[3])
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
/* --- PUBLIC MACROS -------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Connect LoRa concentrator by opening SPI link
@param spidev_path path to the SPI device to be used to connect to the SX1302
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
/* Concentrator connect */
#[no_mangle]
pub unsafe extern "C" fn lgw_connect(mut spidev_path: *const libc::c_char) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut u: uint8_t = 0i32 as uint8_t;
    /* check SPI link status */
    if spidev_path.is_null() {
        return -1i32;
    }
    if !lgw_spi_target.is_null() {
        lgw_spi_close(lgw_spi_target);
    }
    /* open the SPI link */
    spi_stat = lgw_spi_open(spidev_path, &mut lgw_spi_target);
    if spi_stat != 0i32 {
        return -1i32;
    }
    /* check SX1302 version */
    spi_stat = lgw_spi_r(lgw_spi_target, 0i32 as uint8_t, loregs[16].addr, &mut u);
    if spi_stat != 0i32 {
        return -1i32;
    }
    if u as libc::c_int != loregs[16].dflt {
        return -1i32;
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
        chck: false,
        dflt: 0,
    };
    /* check input parameters */
    if register_id as libc::c_int >= 1044i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    spi_stat += reg_w_align32(lgw_spi_target, 0i32 as uint8_t, r, reg_value);
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
        chck: false,
        dflt: 0,
    };
    /* check input parameters */
    if reg_value.is_null() {
        return -1i32;
    }
    if register_id as libc::c_int >= 1044i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    spi_stat += reg_r_align32(lgw_spi_target, 0i32 as uint8_t, r, reg_value);
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
        chck: false,
        dflt: 0,
    };
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    if register_id as libc::c_int >= 1044i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    /* do the burst write */
    spi_stat += lgw_spi_wb(lgw_spi_target, 0i32 as uint8_t, r.addr, data, size);
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
        chck: false,
        dflt: 0,
    };
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    if register_id as libc::c_int >= 1044i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = loregs[register_id as usize];
    /* do the burst read */
    spi_stat += lgw_spi_rb(lgw_spi_target, 0i32 as uint8_t, r.addr, data, size);
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_mem_wb(
    mut mem_addr: uint16_t,
    mut data: *const uint8_t,
    mut size: uint16_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut chunk_cnt: libc::c_int = 0i32;
    let mut addr: uint16_t = mem_addr;
    let mut sz_todo: uint16_t = size;
    let mut chunk_size: uint16_t = 0;
    let CHUNK_SIZE_MAX: uint16_t = 1024i32 as uint16_t;
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* write memory by chunks */
    while sz_todo as libc::c_int > 0i32 {
        /* full or partial chunk ? */
        chunk_size = if sz_todo as libc::c_int > CHUNK_SIZE_MAX as libc::c_int {
            CHUNK_SIZE_MAX as libc::c_int
        } else {
            sz_todo as libc::c_int
        } as uint16_t;
        /* do the burst write */
        spi_stat += lgw_spi_wb(
            lgw_spi_target,
            0i32 as uint8_t,
            addr,
            &*data.offset((chunk_cnt * CHUNK_SIZE_MAX as libc::c_int) as isize),
            chunk_size,
        );
        /* prepare for next write */
        addr = (addr as libc::c_int + chunk_size as libc::c_int) as uint16_t;
        sz_todo = (sz_todo as libc::c_int - chunk_size as libc::c_int) as uint16_t;
        chunk_cnt += 1i32
    }
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* *
TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_mem_rb(
    mut mem_addr: uint16_t,
    mut data: *mut uint8_t,
    mut size: uint16_t,
    mut fifo_mode: bool,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0i32;
    let mut chunk_cnt: libc::c_int = 0i32;
    let mut addr: uint16_t = mem_addr;
    let mut sz_todo: uint16_t = size;
    let mut chunk_size: uint16_t = 0;
    let CHUNK_SIZE_MAX: uint16_t = 1024i32 as uint16_t;
    /* check input parameters */
    if data.is_null() {
        return -1i32;
    }
    if size as libc::c_int == 0i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* read memory by chunks */
    while sz_todo as libc::c_int > 0i32 {
        /* full or partial chunk ? */
        chunk_size = if sz_todo as libc::c_int > CHUNK_SIZE_MAX as libc::c_int {
            CHUNK_SIZE_MAX as libc::c_int
        } else {
            sz_todo as libc::c_int
        } as uint16_t;
        /* do the burst read */
        spi_stat += lgw_spi_rb(
            lgw_spi_target,
            0i32 as uint8_t,
            addr,
            &mut *data.offset((chunk_cnt * CHUNK_SIZE_MAX as libc::c_int) as isize),
            chunk_size,
        );
        /* do not increment the address when the target memory is in FIFO mode (auto-increment) */
        if fifo_mode as libc::c_int == 0i32 {
            addr = (addr as libc::c_int + chunk_size as libc::c_int) as uint16_t
        }
        /* prepare for next read */
        sz_todo = (sz_todo as libc::c_int - chunk_size as libc::c_int) as uint16_t;
        chunk_cnt += 1i32
    }
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* --- EOF ------------------------------------------------------------------ */
