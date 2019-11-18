use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    /* -------------------------------------------------------------------------- */
    /* --- INTERNAL SHARED FUNCTIONS -------------------------------------------- */
    #[no_mangle]
    fn reg_w_align32(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        r: lgw_reg_s,
        reg_value: int32_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn reg_r_align32(
        spi_target: *mut libc::c_void,
        spi_mux_mode: uint8_t,
        spi_mux_target: uint8_t,
        r: lgw_reg_s,
        reg_value: *mut int32_t,
    ) -> libc::c_int;
    /* -------------------------------------------------------------------------- */
    /* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
    #[no_mangle]
    static mut lgw_spi_target: *mut libc::c_void;
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
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2013 Semtech-Cycleo

Description:
    Functions used to handle FPGA register access for LoRa concentrator.
    Registers are addressed by name.
    Multi-bytes registers are handled automatically.
    Read-modify-write is handled automatically.

License: Revised BSD License, see LICENSE.TXT file include in the project
Maintainer: Michael Coracin
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* printf fprintf */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS ---------------------------------------------------- */
/*
auto generated register mapping for C code : 11-Jul-2013 13:20:40
this file contains autogenerated C struct used to access the LoRa register from the Primer firmware
this file is autogenerated from registers description
293 registers are defined
*/
#[no_mangle]
pub static mut fpga_regs: [lgw_reg_s; 38] = [
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 0i32 as uint8_t,
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
            addr: 0i32 as uint8_t,
            offs: 1i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 4i32 as uint8_t,
            rdon: 1i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 0i32 as uint8_t,
            offs: 5i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 3i32 as uint8_t,
            rdon: 1i32 != 0,
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
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 2i32 as uint8_t,
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
            addr: 3i32 as uint8_t,
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
            addr: 3i32 as uint8_t,
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
            addr: 3i32 as uint8_t,
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
            addr: 3i32 as uint8_t,
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
            addr: 3i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 3i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 3i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 3i32 as uint8_t,
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
            rdon: 1i32 != 0,
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
            leng: 16i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 1000i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 14i32 as uint8_t,
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
            addr: 17i32 as uint8_t,
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
            addr: 18i32 as uint8_t,
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
            addr: 19i32 as uint8_t,
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
            addr: 20i32 as uint8_t,
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
            addr: 21i32 as uint8_t,
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
            addr: 22i32 as uint8_t,
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
            addr: 23i32 as uint8_t,
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
            addr: 24i32 as uint8_t,
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
            addr: 25i32 as uint8_t,
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
            addr: 26i32 as uint8_t,
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
            addr: 28i32 as uint8_t,
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
            addr: 28i32 as uint8_t,
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
            addr: 28i32 as uint8_t,
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
            addr: 28i32 as uint8_t,
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
            addr: 28i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 28i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 28i32 as uint8_t,
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
            page: -1i32 as int8_t,
            addr: 28i32 as uint8_t,
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
            addr: 30i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 8i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 160i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 31i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 24i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
    {
        let mut init = lgw_reg_s {
            page: -1i32 as int8_t,
            addr: 34i32 as uint8_t,
            offs: 0i32 as uint8_t,
            sign: 0i32 != 0,
            leng: 6i32 as uint8_t,
            rdon: 0i32 != 0,
            dflt: 0i32,
        };
        init
    },
];
/* ! current SPI mux mode used */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
static mut tx_notch_support: bool = 0i32 != 0;
static mut tx_notch_offset: uint8_t = 0;
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief LoRa concentrator TX notch filter delay
@return delay in microseconds introduced by TX notch filter
*/
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_get_tx_notch_delay() -> libc::c_float {
    let mut tx_notch_delay: libc::c_float = 0.;
    if tx_notch_support as libc::c_int == 0i32 {
        return 0i32 as libc::c_float;
    }
    /* Notch filtering performed by FPGA adds a constant delay (group delay) that we need to compensate */
    tx_notch_delay = (31.25f64
        * ((64i32 + tx_notch_offset as libc::c_int) / 2i32) as libc::c_double
        / 1E3f64) as libc::c_float; /* 32MHz => 31.25ns */
    return tx_notch_delay;
}
/* *
@brief LoRa concentrator FPGA configuration
@param tx_notch_freq TX notch filter frequency, in Hertz
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_configure(mut tx_notch_freq: uint32_t) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut val: int32_t = 0;
    let mut spectral_scan_support: bool = false;
    let mut lbt_support: bool = false;
    /* Check input parameters */
    if tx_notch_freq < 126000u32 || tx_notch_freq > 250000u32 {
        tx_notch_freq = 129000u32
    }
    /* Get supported FPGA features */
    printf(b"INFO: FPGA supported features:\x00" as *const u8 as *const libc::c_char);
    lgw_fpga_reg_r(1i32 as uint16_t, &mut val);
    tx_notch_support = val as uint8_t as libc::c_int >> 0i32 & (1i32 << 1i32) - 1i32 != 0;
    if tx_notch_support as libc::c_int == 1i32 {
        printf(b" [TX filter] \x00" as *const u8 as *const libc::c_char);
    }
    spectral_scan_support = val as uint8_t as libc::c_int >> 1i32 & (1i32 << 1i32) - 1i32 != 0;
    if spectral_scan_support as libc::c_int == 1i32 {
        printf(b" [Spectral Scan] \x00" as *const u8 as *const libc::c_char);
    }
    lbt_support = val as uint8_t as libc::c_int >> 2i32 & (1i32 << 1i32) - 1i32 != 0;
    if lbt_support as libc::c_int == 1i32 {
        printf(b" [LBT] \x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    x = lgw_fpga_reg_w(7i32 as uint16_t, 1i32);
    x |= lgw_fpga_reg_w(8i32 as uint16_t, 1i32);
    x |= lgw_fpga_reg_w(9i32 as uint16_t, 0i32);
    if x != 0i32 {
        return -1i32;
    }
    /* Required for Semtech AP2 reference design */
    x = lgw_fpga_reg_w(10i32 as uint16_t, 1i32);
    if x != 0i32 {
        return -1i32;
    }
    /* Configure TX notch filter */
    if tx_notch_support as libc::c_int == 1i32 {
        tx_notch_offset = (32E6f64
            / (2i32 as libc::c_uint).wrapping_mul(tx_notch_freq) as libc::c_double
            - 64i32 as libc::c_double) as uint8_t;
        x = lgw_fpga_reg_w(37i32 as uint16_t, tx_notch_offset as int32_t);
        if x != 0i32 {
            return -1i32;
        }
        /* Readback to check that notch frequency is programmable */
        x = lgw_fpga_reg_r(37i32 as uint16_t, &mut val);
        if x != 0i32 {
            return -1i32;
        }
        (val) != tx_notch_offset as libc::c_int;
    }
    return 0i32;
}
/* *
@brief LoRa concentrator FPGA register write
@param register_id register number in the data structure describing registers
@param reg_value signed value to write to the register (for u32, use cast)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Write to a register addressed by name */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_reg_w(
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
    if register_id as libc::c_int >= 38i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = fpga_regs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    spi_stat += reg_w_align32(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x1i32 as uint8_t,
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
@brief LoRa concentrator FPGA register read
@param register_id register number in the data structure describing registers
@param reg_value pointer to a variable where to write register read value
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Read to a register addressed by name */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_reg_r(
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
    if register_id as libc::c_int >= 38i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = fpga_regs[register_id as usize];
    spi_stat += reg_r_align32(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x1i32 as uint8_t,
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
@brief LoRa concentrator FPGA register burst write
@param register_id register number in the data structure describing registers
@param data pointer to byte array that will be sent to the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Point to a register by name and do a burst write */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_reg_wb(
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
    if register_id as libc::c_int >= 38i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = fpga_regs[register_id as usize];
    /* reject write to read-only registers */
    if r.rdon as libc::c_int == 1i32 {
        return -1i32;
    }
    /* do the burst write */
    spi_stat += lgw_spi_wb(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x1i32 as uint8_t,
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
@brief LoRa concentrator FPGA register burst read
@param register_id register number in the data structure describing registers
@param data pointer to byte array that will be written from the LoRa concentrator
@param size size of the transfer, in byte(s)
@return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/* Point to a register by name and do a burst read */
#[no_mangle]
pub unsafe extern "C" fn lgw_fpga_reg_rb(
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
    if register_id as libc::c_int >= 38i32 {
        return -1i32;
    }
    /* check if SPI is initialised */
    if lgw_spi_target.is_null() {
        return -1i32;
    }
    /* get register struct from the struct array */
    r = fpga_regs[register_id as usize];
    /* do the burst read */
    spi_stat += lgw_spi_rb(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x1i32 as uint8_t,
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
