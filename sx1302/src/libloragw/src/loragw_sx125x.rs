use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    /* --- PRIVATE VARIABLES ---------------------------------------------------- */
    #[no_mangle]
    static mut lgw_spi_target: *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Functions used to handle LoRa concentrator SX1255/SX1257 radios.

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED TYPES ------------------------------------------------ */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct radio_reg_s {
    pub addr: uint8_t,
    pub offs: uint8_t,
    pub leng: uint8_t,
    /* number of bits in the register */
}
pub type radio_reg_t = libc::c_uint;
pub const SX125x_REG_SX1255_XOSC_TEST__GM_STARTUP: radio_reg_t = 50;
pub const SX125x_REG_SX1255_XOSC_TEST__DISABLE: radio_reg_t = 49;
pub const SX125x_REG_SX1255_XOSC_TEST: radio_reg_t = 48;
pub const SX125x_REG_SX1257_XOSC_TEST__GM_STARTUP: radio_reg_t = 47;
pub const SX125x_REG_SX1257_XOSC_TEST__DISABLE: radio_reg_t = 46;
pub const SX125x_REG_SX1257_XOSC_TEST: radio_reg_t = 45;
pub const SX125x_REG_LOW_BAT_THRESH: radio_reg_t = 44;
pub const SX125x_REG_MODE_STATUS__TX_PLL_LOCKED: radio_reg_t = 43;
pub const SX125x_REG_MODE_STATUS__RX_PLL_LOCKED: radio_reg_t = 42;
pub const SX125x_REG_MODE_STATUS__LOW_BAT_EN: radio_reg_t = 41;
pub const SX125x_REG_MODE_STATUS: radio_reg_t = 40;
pub const SX125x_REG_CLK_SELECT__DAC_CLK_SELECT: radio_reg_t = 39;
pub const SX125x_REG_CLK_SELECT__CLK_OUT: radio_reg_t = 38;
pub const SX125x_REG_CLK_SELECT__RF_LOOPBACK_EN: radio_reg_t = 37;
pub const SX125x_REG_CLK_SELECT__DIG_LOOPBACK_EN: radio_reg_t = 36;
pub const SX125x_REG_CLK_SELECT: radio_reg_t = 35;
pub const SX125x_REG_DIO_MAPPING__DIO_3_MAPPING: radio_reg_t = 34;
pub const SX125x_REG_DIO_MAPPING__DIO_2_MAPPING: radio_reg_t = 33;
pub const SX125x_REG_DIO_MAPPING__DIO_1_MAPPING: radio_reg_t = 32;
pub const SX125x_REG_DIO_MAPPING__DIO_0_MAPPING: radio_reg_t = 31;
pub const SX125x_REG_DIO_MAPPING: radio_reg_t = 30;
pub const SX125x_REG_RX_PLL_BW__ADC_TEMP_EN: radio_reg_t = 29;
pub const SX125x_REG_RX_PLL_BW__PLL_BW: radio_reg_t = 28;
pub const SX125x_REG_RX_PLL_BW: radio_reg_t = 27;
pub const SX125x_REG_RX_BW__BB_BW: radio_reg_t = 26;
pub const SX125x_REG_RX_BW__ADC_TRIM: radio_reg_t = 25;
pub const SX125x_REG_RX_BW__ADC_BW: radio_reg_t = 24;
pub const SX125x_REG_RX_BW: radio_reg_t = 23;
pub const SX125x_REG_RX_ANA_GAIN__LNA_ZIN: radio_reg_t = 22;
pub const SX125x_REG_RX_ANA_GAIN__BB_GAIN: radio_reg_t = 21;
pub const SX125x_REG_RX_ANA_GAIN__LNA_GAIN: radio_reg_t = 20;
pub const SX125x_REG_RX_ANA_GAIN: radio_reg_t = 19;
pub const SX125x_REG_TX_DAC_BW: radio_reg_t = 18;
pub const SX125x_REG_TX_BW__ANA_BW: radio_reg_t = 17;
pub const SX125x_REG_TX_BW__PLL_BW: radio_reg_t = 16;
pub const SX125x_REG_TX_BW: radio_reg_t = 15;
pub const SX125x_REG_TX_GAIN__MIX_GAIN: radio_reg_t = 14;
pub const SX125x_REG_TX_GAIN__DAC_GAIN: radio_reg_t = 13;
pub const SX125x_REG_TX_GAIN: radio_reg_t = 12;
pub const SX125x_REG_VERSION: radio_reg_t = 11;
pub const SX125x_REG_FRF_TX_LSB: radio_reg_t = 10;
pub const SX125x_REG_FRF_TX_MID: radio_reg_t = 9;
pub const SX125x_REG_FRF_TX_MSB: radio_reg_t = 8;
pub const SX125x_REG_FRF_RX_LSB: radio_reg_t = 7;
pub const SX125x_REG_FRF_RX_MID: radio_reg_t = 6;
pub const SX125x_REG_FRF_RX_MSB: radio_reg_t = 5;
pub const SX125x_REG_MODE__STANDBY_EN: radio_reg_t = 4;
pub const SX125x_REG_MODE__RX_EN: radio_reg_t = 3;
pub const SX125x_REG_MODE__TX_EN: radio_reg_t = 2;
pub const SX125x_REG_MODE__PA_DRIVER_EN: radio_reg_t = 1;
pub const SX125x_REG_MODE: radio_reg_t = 0;
pub const LGW_RADIO_TYPE_SX1257: C2RustUnnamed = 2;
pub const LGW_RADIO_TYPE_SX1255: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const LGW_RADIO_TYPE_SX1250: C2RustUnnamed = 5;
pub const LGW_RADIO_TYPE_SX1276: C2RustUnnamed = 4;
pub const LGW_RADIO_TYPE_SX1272: C2RustUnnamed = 3;
pub const LGW_RADIO_TYPE_NONE: C2RustUnnamed = 0;
static mut sx125x_regs: [radio_reg_s; 51] = [
    {
        let mut init = radio_reg_s {
            addr: 0i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 0i32 as uint8_t,
            offs: 3i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 0i32 as uint8_t,
            offs: 2i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 0i32 as uint8_t,
            offs: 1i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 0i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 1i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 2i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 3i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 4i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 5i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 6i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 7i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 8i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 8i32 as uint8_t,
            offs: 4i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 8i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 10i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 10i32 as uint8_t,
            offs: 5i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 10i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 5i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 11i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 12i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 12i32 as uint8_t,
            offs: 5i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 12i32 as uint8_t,
            offs: 1i32 as uint8_t,
            leng: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 12i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 13i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 13i32 as uint8_t,
            offs: 5i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 13i32 as uint8_t,
            offs: 2i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 13i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 14i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 14i32 as uint8_t,
            offs: 1i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 14i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 15i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 15i32 as uint8_t,
            offs: 6i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 15i32 as uint8_t,
            offs: 4i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 15i32 as uint8_t,
            offs: 2i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 15i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 16i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 16i32 as uint8_t,
            offs: 3i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 16i32 as uint8_t,
            offs: 2i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 16i32 as uint8_t,
            offs: 1i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 16i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 17i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 17i32 as uint8_t,
            offs: 2i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 17i32 as uint8_t,
            offs: 1i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 17i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 26i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 38i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 38i32 as uint8_t,
            offs: 4i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 38i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 40i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 8i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 40i32 as uint8_t,
            offs: 4i32 as uint8_t,
            leng: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = radio_reg_s {
            addr: 40i32 as uint8_t,
            offs: 0i32 as uint8_t,
            leng: 4i32 as uint8_t,
        };
        init
    },
];
/* ! generic pointer to the SPI device */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
/* Simple read */
#[no_mangle]
pub unsafe extern "C" fn sx125x_reg_r(
    mut spi_target: *mut libc::c_void,
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
    if data.is_null() {
        return -1i32;
    }
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    out_buf[0] = spi_mux_target;
    out_buf[1] = (0i32 | address as libc::c_int & 0x7fi32) as uint8_t;
    out_buf[2] = 0i32 as uint8_t;
    command_size = 3i32 as uint8_t;
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
        //DEBUG_MSG("Note: SPI read success\n");
        *data = in_buf[(command_size as libc::c_int - 1i32) as usize];
        return 0i32;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx125x_reg_w(
    mut spi_target: *mut libc::c_void,
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
    spi_device = *(spi_target as *mut libc::c_int);
    /* prepare frame to be sent */
    out_buf[0] = spi_mux_target;
    out_buf[1] = (0x80i32 | address as libc::c_int & 0x7fi32) as uint8_t;
    out_buf[2] = data;
    command_size = 3i32 as uint8_t;
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
        //DEBUG_MSG("Note: SPI write success\n");
        return 0i32;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_sx125x_reg_w(
    mut idx: radio_reg_t,
    mut data: uint8_t,
    mut rf_chain: uint8_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0;
    let mut reg: radio_reg_s = radio_reg_s {
        addr: 0,
        offs: 0,
        leng: 0,
    };
    let mut mask: uint8_t = 0;
    let mut r: uint8_t = 0;
    let mut w: uint8_t = 0;
    let mut val_check: uint8_t = 0;
    /* checking input parameters */
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    if idx as libc::c_uint >= 51i32 as libc::c_uint {
        return -1i32;
    }
    reg = sx125x_regs[idx as usize];
    if reg.leng as libc::c_int == 8i32 && reg.offs as libc::c_int == 0i32 {
        /* direct write */
        spi_stat = sx125x_reg_w(
            lgw_spi_target,
            if rf_chain as libc::c_int == 0i32 {
                0x1i32
            } else {
                0x2i32
            } as uint8_t,
            reg.addr,
            data,
        )
    } else {
        /* read-modify-write */
        spi_stat = sx125x_reg_r(
            lgw_spi_target,
            if rf_chain as libc::c_int == 0i32 {
                0x1i32
            } else {
                0x2i32
            } as uint8_t,
            reg.addr,
            &mut r,
        );
        mask = ((1i32 << reg.leng as libc::c_int) - 1i32 << reg.offs as libc::c_int) as uint8_t;
        w = (r as libc::c_int & !(mask as libc::c_int)
            | (data as libc::c_int) << reg.offs as libc::c_int & mask as libc::c_int)
            as uint8_t;
        spi_stat |= sx125x_reg_w(
            lgw_spi_target,
            if rf_chain as libc::c_int == 0i32 {
                0x1i32
            } else {
                0x2i32
            } as uint8_t,
            reg.addr,
            w,
        )
    }
    /* Check that we can read what we have written */
    lgw_sx125x_reg_r(idx, &mut val_check, rf_chain);
    if val_check as libc::c_int != data as libc::c_int {
        printf(
            b"ERROR: sx125x register %d write failed (w:%u r:%u)!!\n\x00" as *const u8
                as *const libc::c_char,
            idx as libc::c_uint,
            data as libc::c_int,
            val_check as libc::c_int,
        );
        spi_stat = -1i32
    }
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_sx125x_reg_r(
    mut idx: radio_reg_t,
    mut data: *mut uint8_t,
    mut rf_chain: uint8_t,
) -> libc::c_int {
    let mut spi_stat: libc::c_int = 0;
    let mut reg: radio_reg_s = radio_reg_s {
        addr: 0,
        offs: 0,
        leng: 0,
    };
    let mut mask: uint8_t = 0;
    let mut r: uint8_t = 0;
    /* checking input parameters */
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    if idx as libc::c_uint >= 51i32 as libc::c_uint {
        return -1i32;
    }
    reg = sx125x_regs[idx as usize];
    spi_stat = sx125x_reg_r(
        lgw_spi_target,
        if rf_chain as libc::c_int == 0i32 {
            0x1i32
        } else {
            0x2i32
        } as uint8_t,
        reg.addr,
        &mut r,
    );
    mask = ((1i32 << reg.leng as libc::c_int) - 1i32 << reg.offs as libc::c_int) as uint8_t;
    *data = ((r as libc::c_int & mask as libc::c_int) >> reg.offs as libc::c_int) as uint8_t;
    if spi_stat != 0i32 {
        return -1i32;
    } else {
        return 0i32;
    };
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
/*

SX1257 frequency setting :
F_register(24bit) = F_rf (Hz) / F_step(Hz)
                  = F_rf (Hz) * 2^19 / F_xtal(Hz)
                  = F_rf (Hz) * 2^19 / 32e6
                  = F_rf (Hz) * 256/15625

SX1255 frequency setting :
F_register(24bit) = F_rf (Hz) / F_step(Hz)
                  = F_rf (Hz) * 2^20 / F_xtal(Hz)
                  = F_rf (Hz) * 2^20 / 32e6
                  = F_rf (Hz) * 512/15625
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn sx125x_setup(
    mut rf_chain: uint8_t,
    mut rf_clkout: uint8_t,
    mut rf_enable: bool,
    mut rf_radio_type: uint8_t,
    mut freq_hz: uint32_t,
) -> libc::c_int {
    let mut part_int: uint32_t = 0i32 as uint32_t;
    let mut part_frac: uint32_t = 0i32 as uint32_t;
    let mut cpt_attempts: libc::c_int = 0i32;
    let mut val: uint8_t = 0;
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    /* Get version to identify SX1255/57 silicon revision */
    lgw_sx125x_reg_r(SX125x_REG_VERSION, &mut val, rf_chain);
    /* General radio setup */
    if rf_clkout as libc::c_int == rf_chain as libc::c_int {
        lgw_sx125x_reg_w(SX125x_REG_CLK_SELECT, (0i32 + 2i32) as uint8_t, rf_chain);
    } else {
        lgw_sx125x_reg_w(SX125x_REG_CLK_SELECT, 0i32 as uint8_t, rf_chain);
    }
    match rf_radio_type as libc::c_int {
        1 => {
            lgw_sx125x_reg_w(
                SX125x_REG_SX1255_XOSC_TEST__GM_STARTUP,
                13i32 as uint8_t,
                rf_chain,
            );
            lgw_sx125x_reg_w(
                SX125x_REG_SX1255_XOSC_TEST__DISABLE,
                2i32 as uint8_t,
                rf_chain,
            );
        }
        2 => {
            lgw_sx125x_reg_w(
                SX125x_REG_SX1257_XOSC_TEST__GM_STARTUP,
                13i32 as uint8_t,
                rf_chain,
            );
            lgw_sx125x_reg_w(
                SX125x_REG_SX1257_XOSC_TEST__DISABLE,
                2i32 as uint8_t,
                rf_chain,
            );
        }
        _ => {}
    }
    if rf_enable as libc::c_int == 1i32 {
        /* Tx gain and trim */
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__MIX_GAIN, 14i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__DAC_GAIN, 2i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_TX_BW__ANA_BW, 0i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_TX_BW__PLL_BW, 1i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_TX_DAC_BW, 5i32 as uint8_t, rf_chain);
        /* Rx gain and trim */
        lgw_sx125x_reg_w(SX125x_REG_RX_ANA_GAIN__LNA_ZIN, 0i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_ANA_GAIN__BB_GAIN, 15i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_ANA_GAIN__LNA_GAIN, 1i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_BW__BB_BW, 0i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_BW__ADC_TRIM, 6i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_BW__ADC_BW, 7i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_PLL_BW__ADC_TEMP_EN, 0i32 as uint8_t, rf_chain);
        lgw_sx125x_reg_w(SX125x_REG_RX_PLL_BW__PLL_BW, 0i32 as uint8_t, rf_chain);
        /* set RX PLL frequency */
        match rf_radio_type as libc::c_int {
            1 => {
                part_int = freq_hz.wrapping_div((15625i32 << 7i32) as libc::c_uint); /* integer part, gives the MSB */
                part_frac = (freq_hz.wrapping_rem((15625i32 << 7i32) as libc::c_uint) << 9i32)
                    .wrapping_div(15625i32 as libc::c_uint)
            }
            2 => {
                /* fractional part, gives middle part and LSB */
                part_int = freq_hz.wrapping_div((15625i32 << 8i32) as libc::c_uint); /* integer part, gives the MSB */
                part_frac = (freq_hz.wrapping_rem((15625i32 << 8i32) as libc::c_uint) << 8i32)
                    .wrapping_div(15625i32 as libc::c_uint)
            }
            _ => {}
        } /* fractional part, gives middle part and LSB */
        lgw_sx125x_reg_w(
            SX125x_REG_FRF_RX_MSB,
            (0xffi32 as libc::c_uint & part_int) as uint8_t,
            rf_chain,
        );
        lgw_sx125x_reg_w(
            SX125x_REG_FRF_RX_MID,
            (0xffi32 as libc::c_uint & part_frac >> 8i32) as uint8_t,
            rf_chain,
        );
        lgw_sx125x_reg_w(
            SX125x_REG_FRF_RX_LSB,
            (0xffi32 as libc::c_uint & part_frac) as uint8_t,
            rf_chain,
        );
        loop
        /* start and PLL lock */
        {
            if cpt_attempts >= 5i32 {
                return -1i32;
            }
            lgw_sx125x_reg_w(SX125x_REG_MODE, 1i32 as uint8_t, rf_chain);
            lgw_sx125x_reg_w(SX125x_REG_MODE, 3i32 as uint8_t, rf_chain);
            cpt_attempts += 1;
            wait_ms(1i32 as libc::c_ulong);
            lgw_sx125x_reg_r(SX125x_REG_MODE_STATUS, &mut val, rf_chain);
            if !(val as libc::c_int & 0x2i32 == 0i32) {
                break;
            }
        }
    }
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
