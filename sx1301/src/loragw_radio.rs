use libc;
extern "C" {
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
    /*
     / _____)             _              | |
    ( (____  _____ ____ _| |_ _____  ____| |__
     \____ \| ___ |    (_   _) ___ |/ ___)  _ \
     _____) ) ____| | | || |_| ____( (___| | | |
    (______/|_____)_|_|_| \__)_____)\____)_| |_|
      (C)2013 Semtech-Cycleo

    Description:
        LoRa concentrator HAL common auxiliary functions

    License: Revised BSD License, see LICENSE.TXT file include in the project
    Maintainer: Sylvain Miermont
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
    /* *
    @brief LoRa concentrator register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
    /* *
    @brief LoRa concentrator register read
    @param register_id register number in the data structure describing registers
    @param reg_value pointer to a variable where to write register read value
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_r(register_id: uint16_t, reg_value: *mut int32_t) -> libc::c_int;
    /* *
    @brief LoRa concentrator FPGA register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_fpga_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
    /* -------------------------------------------------------------------------- */
    /* --- PRIVATE VARIABLES ---------------------------------------------------- */
    #[no_mangle]
    static mut lgw_spi_target: *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
/* values available for the 'bandwidth' parameters (LoRa & FSK) */
/* NOTE: directly encode FSK RX bandwidth, do not change */
/* values available for the 'datarate' parameters */
/* NOTE: LoRa values used directly to code SF bitmask in 'multi' modem, do not change */
/* NOTE: for FSK directly use baudrate between 500 bauds and 250 kbauds */
/* values available for the 'coderate' parameters (LoRa only) */
/* NOTE: arbitrary values */
/* values available for the 'status' parameter */
/* NOTE: values according to hardware specification */
/* values available for the 'tx_mode' parameter */
//#define ON_EVENT      3
//#define GPS_DELAYED   4
//#define EVENT_DELAYED 5
/* values available for 'select' in the status function */
/* status code for TX_STATUS */
/* NOTE: arbitrary values */
/* TX modem disabled, it will ignore commands */
/* TX modem is free, ready to receive a command */
/* TX modem is loaded, ready to send the packet after an event and/or delay */
/* TX modem is emitting */
/* status code for RX_STATUS */
/* NOTE: arbitrary values */
/* RX modem is disabled, it will ignore commands  */
/* RX modem is receiving */
/* RX is suspended while a TX is ongoing */
/* Maximum size of Tx gain LUT */
/* LBT constants */
/* Number of LBT channels */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC TYPES --------------------------------------------------------- */
/* *
@enum lgw_radio_type_e
@brief Radio types that can be found on the LoRa Gateway
*/
pub type lgw_radio_type_e = libc::c_uint;
pub const LGW_RADIO_TYPE_SX1276: lgw_radio_type_e = 4;
pub const LGW_RADIO_TYPE_SX1272: lgw_radio_type_e = 3;
pub const LGW_RADIO_TYPE_SX1257: lgw_radio_type_e = 2;
pub const LGW_RADIO_TYPE_SX1255: lgw_radio_type_e = 1;
pub const LGW_RADIO_TYPE_NONE: lgw_radio_type_e = 0;
/* irreductible fraction for PLL register caculation */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
pub type lgw_sx127x_rxbw_e = libc::c_uint;
pub const LGW_SX127X_RXBW_250K_HZ: lgw_sx127x_rxbw_e = 20;
pub const LGW_SX127X_RXBW_200K_HZ: lgw_sx127x_rxbw_e = 19;
pub const LGW_SX127X_RXBW_166K7_HZ: lgw_sx127x_rxbw_e = 18;
pub const LGW_SX127X_RXBW_125K_HZ: lgw_sx127x_rxbw_e = 17;
pub const LGW_SX127X_RXBW_100K_HZ: lgw_sx127x_rxbw_e = 16;
pub const LGW_SX127X_RXBW_83K3_HZ: lgw_sx127x_rxbw_e = 15;
pub const LGW_SX127X_RXBW_62K5_HZ: lgw_sx127x_rxbw_e = 14;
pub const LGW_SX127X_RXBW_50K_HZ: lgw_sx127x_rxbw_e = 13;
pub const LGW_SX127X_RXBW_41K7_HZ: lgw_sx127x_rxbw_e = 12;
pub const LGW_SX127X_RXBW_31K3_HZ: lgw_sx127x_rxbw_e = 11;
pub const LGW_SX127X_RXBW_25K_HZ: lgw_sx127x_rxbw_e = 10;
pub const LGW_SX127X_RXBW_20K8_HZ: lgw_sx127x_rxbw_e = 9;
pub const LGW_SX127X_RXBW_15K6_HZ: lgw_sx127x_rxbw_e = 8;
pub const LGW_SX127X_RXBW_12K5_HZ: lgw_sx127x_rxbw_e = 7;
pub const LGW_SX127X_RXBW_10K4_HZ: lgw_sx127x_rxbw_e = 6;
pub const LGW_SX127X_RXBW_7K8_HZ: lgw_sx127x_rxbw_e = 5;
pub const LGW_SX127X_RXBW_6K3_HZ: lgw_sx127x_rxbw_e = 4;
pub const LGW_SX127X_RXBW_5K2_HZ: lgw_sx127x_rxbw_e = 3;
pub const LGW_SX127X_RXBW_3K9_HZ: lgw_sx127x_rxbw_e = 2;
pub const LGW_SX127X_RXBW_3K1_HZ: lgw_sx127x_rxbw_e = 1;
pub const LGW_SX127X_RXBW_2K6_HZ: lgw_sx127x_rxbw_e = 0;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2013 Semtech-Cycleo

Description:
    Functions used to handle LoRa concentrator radios.

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
/* --- PRIVATE TYPES -------------------------------------------------------- */
/* *
@struct lgw_radio_FSK_bandwidth_s
@brief Associate a bandwidth in kHz with its corresponding register values
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_sx127x_FSK_bandwidth_s {
    pub RxBwKHz: uint32_t,
    pub RxBwMant: uint8_t,
    pub RxBwExp: uint8_t,
}
/* *
@struct lgw_radio_type_version_s
@brief Associate a radio type with its corresponding expected version value
        read in the radio version register.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_radio_type_version_s {
    pub type_0: lgw_radio_type_e,
    pub reg_version: uint8_t,
}
#[no_mangle]
pub static mut sx127x_FskBandwidths: [lgw_sx127x_FSK_bandwidth_s; 21] = [
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 2600i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 7i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 3100i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 7i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 3900i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 7i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 5200i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 6i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 6300i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 6i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 7800i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 6i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 10400i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 5i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 12500i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 5i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 15600i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 5i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 20800i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 25000i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 31300i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 4i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 41700i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 50000i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 62500i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 3i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 83333i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 100000i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 125000i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 2i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 166700i32 as uint32_t,
            RxBwMant: 2i32 as uint8_t,
            RxBwExp: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 200000i32 as uint32_t,
            RxBwMant: 1i32 as uint8_t,
            RxBwExp: 1i32 as uint8_t,
        };
        init
    },
    {
        let mut init = lgw_sx127x_FSK_bandwidth_s {
            RxBwKHz: 250000i32 as uint32_t,
            RxBwMant: 0i32 as uint8_t,
            RxBwExp: 1i32 as uint8_t,
        };
        init
    },
];
/* ! generic pointer to the SPI device */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn sx125x_write(mut channel: uint8_t, mut addr: uint8_t, mut data: uint8_t) {
    let mut reg_add: libc::c_int = 0;
    let mut reg_dat: libc::c_int = 0;
    let mut reg_cs: libc::c_int = 0;
    /* checking input parameters */
    if channel as libc::c_int >= 2i32 {
        return;
    }
    if addr as libc::c_int >= 0x7fi32 {
        return;
    }
    /* selecting the target radio */
    match channel as libc::c_int {
        0 => {
            reg_add = 251i32;
            reg_dat = 249i32;
            reg_cs = 252i32
        }
        1 => {
            reg_add = 255i32;
            reg_dat = 253i32;
            reg_cs = 256i32
        }
        _ => return,
    }
    /* SPI master data write procedure */
    lgw_reg_w(reg_cs as uint16_t, 0i32); /* MSB at 1 for write operation */
    lgw_reg_w(reg_add as uint16_t, 0x80i32 | addr as libc::c_int);
    lgw_reg_w(reg_dat as uint16_t, data as int32_t);
    lgw_reg_w(reg_cs as uint16_t, 1i32);
    lgw_reg_w(reg_cs as uint16_t, 0i32);
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx125x_read(mut channel: uint8_t, mut addr: uint8_t) -> uint8_t {
    let mut reg_add: libc::c_int = 0;
    let mut reg_dat: libc::c_int = 0;
    let mut reg_cs: libc::c_int = 0;
    let mut reg_rb: libc::c_int = 0;
    let mut read_value: int32_t = 0;
    /* checking input parameters */
    if channel as libc::c_int >= 2i32 {
        return 0i32 as uint8_t;
    }
    if addr as libc::c_int >= 0x7fi32 {
        return 0i32 as uint8_t;
    }
    /* selecting the target radio */
    match channel as libc::c_int {
        0 => {
            reg_add = 251i32;
            reg_dat = 249i32;
            reg_cs = 252i32;
            reg_rb = 250i32
        }
        1 => {
            reg_add = 255i32;
            reg_dat = 253i32;
            reg_cs = 256i32;
            reg_rb = 254i32
        }
        _ => return 0i32 as uint8_t,
    }
    /* SPI master data read procedure */
    lgw_reg_w(reg_cs as uint16_t, 0i32); /* MSB at 0 for read operation */
    lgw_reg_w(reg_add as uint16_t, addr as int32_t);
    lgw_reg_w(reg_dat as uint16_t, 0i32);
    lgw_reg_w(reg_cs as uint16_t, 1i32);
    lgw_reg_w(reg_cs as uint16_t, 0i32);
    lgw_reg_r(reg_rb as uint16_t, &mut read_value);
    return read_value as uint8_t;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn setup_sx1272_FSK(
    mut frequency: uint32_t,
    mut rxbw_khz: lgw_sx127x_rxbw_e,
    mut rssi_offset: int8_t,
) -> libc::c_int {
    let mut freq_reg: uint64_t = 0;
    let mut ModulationShaping: uint8_t = 0i32 as uint8_t;
    let mut PllHop: uint8_t = 1i32 as uint8_t;
    let mut LnaGain: uint8_t = 1i32 as uint8_t;
    let mut LnaBoost: uint8_t = 3i32 as uint8_t;
    let mut AdcBwAuto: uint8_t = 0i32 as uint8_t;
    let mut AdcBw: uint8_t = 7i32 as uint8_t;
    let mut AdcLowPwr: uint8_t = 0i32 as uint8_t;
    let mut AdcTrim: uint8_t = 6i32 as uint8_t;
    let mut AdcTest: uint8_t = 0i32 as uint8_t;
    let mut RxBwExp: uint8_t = sx127x_FskBandwidths[rxbw_khz as usize].RxBwExp;
    let mut RxBwMant: uint8_t = sx127x_FskBandwidths[rxbw_khz as usize].RxBwMant;
    let mut RssiSmoothing: uint8_t = 5i32 as uint8_t;
    let mut RssiOffsetReg: uint8_t = 0;
    let mut reg_val: uint8_t = 0;
    let mut x: libc::c_int = 0;
    /* Set in FSK mode */
    x = lgw_sx127x_reg_w(0x1i32 as uint8_t, 0i32 as uint8_t); /* Sleep mode, no FSK shaping */
    wait_ms(100i32 as libc::c_ulong); /* Standby mode, no FSK shaping */
    x |= lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (0i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    );
    wait_ms(100i32 as libc::c_ulong);
    x |= lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (1i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    );
    wait_ms(100i32 as libc::c_ulong);
    /* Set RF carrier frequency */
    x |= lgw_sx127x_reg_w(
        0x4bi32 as uint8_t,
        ((PllHop as libc::c_int) << 7i32) as uint8_t,
    );
    freq_reg = ((frequency as uint64_t) << 19i32).wrapping_div(32000000i32 as uint64_t);
    x |= lgw_sx127x_reg_w(
        0x6i32 as uint8_t,
        (freq_reg >> 16i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x7i32 as uint8_t,
        (freq_reg >> 8i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x8i32 as uint8_t,
        (freq_reg >> 0i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    /* Config */
    x |= lgw_sx127x_reg_w(
        0xci32 as uint8_t,
        (LnaBoost as libc::c_int | (LnaGain as libc::c_int) << 5i32) as uint8_t,
    ); /* Improved sensitivity, highest gain */
    x |= lgw_sx127x_reg_w(
        0x68i32 as uint8_t,
        (AdcBw as libc::c_int | (AdcBwAuto as libc::c_int) << 3i32) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x69i32 as uint8_t,
        (AdcTest as libc::c_int
            | (AdcTrim as libc::c_int) << 4i32
            | (AdcLowPwr as libc::c_int) << 7i32) as uint8_t,
    );
    /* set BR and FDEV for 200 kHz bandwidth*/
    x |= lgw_sx127x_reg_w(0x2i32 as uint8_t, 125i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x3i32 as uint8_t, 0i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x4i32 as uint8_t, 2i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x5i32 as uint8_t, 225i32 as uint8_t);
    /* Config continues... */
    x |= lgw_sx127x_reg_w(0xdi32 as uint8_t, 0i32 as uint8_t); /* Disable AGC */
    RssiOffsetReg = if rssi_offset as libc::c_int >= 0i32 {
        rssi_offset as uint8_t as libc::c_int
    } else {
        (!-(rssi_offset as libc::c_int) + 1i32) as uint8_t as libc::c_int
    } as uint8_t; /* 2's complement */
    x |= lgw_sx127x_reg_w(
        0xei32 as uint8_t,
        (RssiSmoothing as libc::c_int | (RssiOffsetReg as libc::c_int) << 3i32) as uint8_t,
    ); /* Set RSSI smoothing to 64 samples, RSSI offset to given value */
    x |= lgw_sx127x_reg_w(
        0x12i32 as uint8_t,
        (RxBwExp as libc::c_int | (RxBwMant as libc::c_int) << 3i32) as uint8_t,
    ); /* PLL BW set to 75 KHz */
    x |= lgw_sx127x_reg_w(0x23i32 as uint8_t, 2i32 as uint8_t); /* optimize PLL start-up time */
    x |= lgw_sx127x_reg_w(0x5ci32 as uint8_t, 0x10i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x47i32 as uint8_t, 1i32 as uint8_t);
    if x != 0i32 {
        return x;
    }
    /* set Rx continuous mode */
    x = lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (5i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    ); /* Receiver Mode, no FSK shaping */
    wait_ms(500i32 as libc::c_ulong);
    x |= lgw_sx127x_reg_r(0x3ei32 as uint8_t, &mut reg_val);
    /* Check if RxReady and ModeReady */
    if reg_val as libc::c_int >> 6i32 & (1i32 << 1i32) - 1i32 == 0i32
        || reg_val as libc::c_int >> 7i32 & (1i32 << 1i32) - 1i32 == 0i32
        || x != 0i32
    {
        return -1i32;
    }
    wait_ms(500i32 as libc::c_ulong);
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn setup_sx1276_FSK(
    mut frequency: uint32_t,
    mut rxbw_khz: lgw_sx127x_rxbw_e,
    mut rssi_offset: int8_t,
) -> libc::c_int {
    let mut freq_reg: uint64_t = 0;
    let mut ModulationShaping: uint8_t = 0i32 as uint8_t;
    let mut PllHop: uint8_t = 1i32 as uint8_t;
    let mut LnaGain: uint8_t = 1i32 as uint8_t;
    let mut LnaBoost: uint8_t = 3i32 as uint8_t;
    let mut AdcBwAuto: uint8_t = 0i32 as uint8_t;
    let mut AdcBw: uint8_t = 7i32 as uint8_t;
    let mut AdcLowPwr: uint8_t = 0i32 as uint8_t;
    let mut AdcTrim: uint8_t = 6i32 as uint8_t;
    let mut AdcTest: uint8_t = 0i32 as uint8_t;
    let mut RxBwExp: uint8_t = sx127x_FskBandwidths[rxbw_khz as usize].RxBwExp;
    let mut RxBwMant: uint8_t = sx127x_FskBandwidths[rxbw_khz as usize].RxBwMant;
    let mut RssiSmoothing: uint8_t = 5i32 as uint8_t;
    let mut RssiOffsetReg: uint8_t = 0;
    let mut reg_val: uint8_t = 0;
    let mut x: libc::c_int = 0;
    /* Set in FSK mode */
    x = lgw_sx127x_reg_w(0x1i32 as uint8_t, 0i32 as uint8_t); /* Sleep mode, no FSK shaping */
    wait_ms(100i32 as libc::c_ulong); /* Standby mode, no FSK shaping */
    x |= lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (0i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    );
    wait_ms(100i32 as libc::c_ulong);
    x |= lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (1i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    );
    wait_ms(100i32 as libc::c_ulong);
    /* Set RF carrier frequency */
    x |= lgw_sx127x_reg_w(
        0x44i32 as uint8_t,
        ((PllHop as libc::c_int) << 7i32) as uint8_t,
    );
    freq_reg = ((frequency as uint64_t) << 19i32).wrapping_div(32000000i32 as uint64_t);
    x |= lgw_sx127x_reg_w(
        0x6i32 as uint8_t,
        (freq_reg >> 16i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x7i32 as uint8_t,
        (freq_reg >> 8i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x8i32 as uint8_t,
        (freq_reg >> 0i32 & 0xffi32 as libc::c_ulong) as uint8_t,
    );
    /* Config */
    x |= lgw_sx127x_reg_w(
        0xci32 as uint8_t,
        (LnaBoost as libc::c_int | (LnaGain as libc::c_int) << 5i32) as uint8_t,
    ); /* Improved sensitivity, highest gain */
    x |= lgw_sx127x_reg_w(
        0x57i32 as uint8_t,
        (AdcBw as libc::c_int | (AdcBwAuto as libc::c_int) << 3i32) as uint8_t,
    );
    x |= lgw_sx127x_reg_w(
        0x58i32 as uint8_t,
        (AdcTest as libc::c_int
            | (AdcTrim as libc::c_int) << 4i32
            | (AdcLowPwr as libc::c_int) << 7i32) as uint8_t,
    );
    /* set BR and FDEV for 200 kHz bandwidth*/
    x |= lgw_sx127x_reg_w(0x2i32 as uint8_t, 125i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x3i32 as uint8_t, 0i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x4i32 as uint8_t, 2i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x5i32 as uint8_t, 225i32 as uint8_t);
    /* Config continues... */
    x |= lgw_sx127x_reg_w(0xdi32 as uint8_t, 0i32 as uint8_t); /* Disable AGC */
    RssiOffsetReg = if rssi_offset as libc::c_int >= 0i32 {
        rssi_offset as uint8_t as libc::c_int
    } else {
        (!-(rssi_offset as libc::c_int) + 1i32) as uint8_t as libc::c_int
    } as uint8_t; /* 2's complement */
    x |= lgw_sx127x_reg_w(
        0xei32 as uint8_t,
        (RssiSmoothing as libc::c_int | (RssiOffsetReg as libc::c_int) << 3i32) as uint8_t,
    ); /* Set RSSI smoothing to 64 samples, RSSI offset 3dB */
    x |= lgw_sx127x_reg_w(
        0x12i32 as uint8_t,
        (RxBwExp as libc::c_int | (RxBwMant as libc::c_int) << 3i32) as uint8_t,
    ); /* PLL BW set to 75 KHz */
    x |= lgw_sx127x_reg_w(0x23i32 as uint8_t, 2i32 as uint8_t); /* optimize PLL start-up time */
    x |= lgw_sx127x_reg_w(0x70i32 as uint8_t, 0x10i32 as uint8_t);
    x |= lgw_sx127x_reg_w(0x43i32 as uint8_t, 1i32 as uint8_t);
    if x != 0i32 {
        return x;
    }
    /* set Rx continuous mode */
    x = lgw_sx127x_reg_w(
        0x1i32 as uint8_t,
        (5i32 | (ModulationShaping as libc::c_int) << 3i32) as uint8_t,
    ); /* Receiver Mode, no FSK shaping */
    wait_ms(500i32 as libc::c_ulong);
    x |= lgw_sx127x_reg_r(0x3ei32 as uint8_t, &mut reg_val);
    /* Check if RxReady and ModeReady */
    if reg_val as libc::c_int >> 6i32 & (1i32 << 1i32) - 1i32 == 0i32
        || reg_val as libc::c_int >> 7i32 & (1i32 << 1i32) - 1i32 == 0i32
        || x != 0i32
    {
        return -1i32;
    }
    wait_ms(500i32 as libc::c_ulong);
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn reset_sx127x(mut radio_type: lgw_radio_type_e) -> libc::c_int {
    let mut x: libc::c_int = 0;
    match radio_type as libc::c_uint {
        4 => {
            x = lgw_fpga_reg_w(6i32 as uint16_t, 0i32);
            x |= lgw_fpga_reg_w(6i32 as uint16_t, 1i32);
            if x != 0i32 {
                return x;
            }
        }
        3 => {
            x = lgw_fpga_reg_w(6i32 as uint16_t, 1i32);
            x |= lgw_fpga_reg_w(6i32 as uint16_t, 0i32);
            if x != 0i32 {
                return x;
            }
        }
        _ => return -1i32,
    }
    return 0i32;
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn lgw_setup_sx125x(
    mut rf_chain: uint8_t,
    mut rf_clkout: uint8_t,
    mut rf_enable: bool,
    mut rf_radio_type: uint8_t,
    mut freq_hz: uint32_t,
) -> libc::c_int {
    let mut part_int: uint32_t = 0i32 as uint32_t;
    let mut part_frac: uint32_t = 0i32 as uint32_t;
    let mut cpt_attempts: libc::c_int = 0i32;
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    /* Get version to identify SX1255/57 silicon revision */
    /* General radio setup */
    if rf_clkout as libc::c_int == rf_chain as libc::c_int {
        sx125x_write(rf_chain, 0x10i32 as uint8_t, (1i32 + 2i32) as uint8_t);
    } else {
        sx125x_write(rf_chain, 0x10i32 as uint8_t, 1i32 as uint8_t);
    }
    match rf_radio_type as libc::c_int {
        1 => {
            sx125x_write(
                rf_chain,
                0x28i32 as uint8_t,
                (13i32 + 2i32 * 16i32) as uint8_t,
            );
        }
        2 => {
            sx125x_write(
                rf_chain,
                0x26i32 as uint8_t,
                (13i32 + 2i32 * 16i32) as uint8_t,
            );
        }
        _ => {}
    }
    if rf_enable as libc::c_int == 1i32 {
        /* Tx gain and trim */
        sx125x_write(
            rf_chain,
            0x8i32 as uint8_t,
            (14i32 + 2i32 * 16i32) as uint8_t,
        );
        sx125x_write(
            rf_chain,
            0xai32 as uint8_t,
            (0i32 + 1i32 * 32i32) as uint8_t,
        );
        sx125x_write(rf_chain, 0xbi32 as uint8_t, 5i32 as uint8_t);
        /* Rx gain and trim */
        sx125x_write(
            rf_chain,
            0xci32 as uint8_t,
            (1i32 + 12i32 * 2i32 + 1i32 * 32i32) as uint8_t,
        );
        sx125x_write(
            rf_chain,
            0xdi32 as uint8_t,
            (0i32 + 6i32 * 4i32 + 7i32 * 32i32) as uint8_t,
        );
        sx125x_write(rf_chain, 0xei32 as uint8_t, (0i32 + 0i32 * 2i32) as uint8_t);
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
        sx125x_write(
            rf_chain,
            0x1i32 as uint8_t,
            (0xffi32 as libc::c_uint & part_int) as uint8_t,
        ); /* Most Significant Byte */
        sx125x_write(
            rf_chain,
            0x2i32 as uint8_t,
            (0xffi32 as libc::c_uint & part_frac >> 8i32) as uint8_t,
        ); /* middle byte */
        sx125x_write(
            rf_chain,
            0x3i32 as uint8_t,
            (0xffi32 as libc::c_uint & part_frac) as uint8_t,
        ); /* Least Significant Byte */
        loop
        /* start and PLL lock */
        {
            if cpt_attempts >= 5i32 {
                return -1i32;
            } /* enable Xtal oscillator */
            sx125x_write(rf_chain, 0i32 as uint8_t, 1i32 as uint8_t); /* Enable RX (PLL+FE) */
            sx125x_write(rf_chain, 0i32 as uint8_t, 3i32 as uint8_t);
            cpt_attempts += 1;
            wait_ms(1i32 as libc::c_ulong);
            if !(sx125x_read(rf_chain, 0x11i32 as uint8_t) as libc::c_int & 0x2i32 == 0i32) {
                break;
            }
        }
    }
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_sx127x_reg_w(
    mut address: uint8_t,
    mut reg_value: uint8_t,
) -> libc::c_int {
    return lgw_spi_w(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x3i32 as uint8_t,
        address,
        reg_value,
    );
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_sx127x_reg_r(
    mut address: uint8_t,
    mut reg_value: *mut uint8_t,
) -> libc::c_int {
    return lgw_spi_r(
        lgw_spi_target,
        0x1i32 as uint8_t,
        0x3i32 as uint8_t,
        address,
        reg_value,
    );
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lgw_setup_sx127x(
    mut frequency: uint32_t,
    mut modulation: uint8_t,
    mut rxbw_khz: lgw_sx127x_rxbw_e,
    mut rssi_offset: int8_t,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut version: uint8_t = 0;
    let mut radio_type: lgw_radio_type_e = LGW_RADIO_TYPE_NONE;
    let mut supported_radio_type: [lgw_radio_type_version_s; 2] = [
        {
            let mut init = lgw_radio_type_version_s {
                type_0: LGW_RADIO_TYPE_SX1272,
                reg_version: 0x22i32 as uint8_t,
            };
            init
        },
        {
            let mut init = lgw_radio_type_version_s {
                type_0: LGW_RADIO_TYPE_SX1276,
                reg_version: 0x12i32 as uint8_t,
            };
            init
        },
    ];
    /* Check parameters */
    if modulation as libc::c_int != 0x20i32 {
        return -1i32;
    }
    if rxbw_khz as libc::c_uint > LGW_SX127X_RXBW_250K_HZ as libc::c_int as libc::c_uint {
        return -1i32;
    }
    /* Probing radio type */
    i = 0i32;
    while i < ::std::mem::size_of::<[lgw_radio_type_version_s; 2]>() as libc::c_ulong as libc::c_int
    {
        /* Reset the radio */
        x = reset_sx127x(supported_radio_type[i as usize].type_0);
        if x != 0i32 {
            return x;
        }
        /* Read version register */
        x = lgw_sx127x_reg_r(0x42i32 as uint8_t, &mut version);
        if x != 0i32 {
            return x;
        }
        /* Check if we got the expected version */
        if version as libc::c_int != supported_radio_type[i as usize].reg_version as libc::c_int {
            i += 1
        } else {
            radio_type = supported_radio_type[i as usize].type_0;
            break;
        }
    }
    if radio_type as libc::c_uint == LGW_RADIO_TYPE_NONE as libc::c_int as libc::c_uint {
        return -1i32;
    }
    /* Setup the radio */
    match modulation as libc::c_int {
        32 => {
            if radio_type as libc::c_uint == LGW_RADIO_TYPE_SX1272 as libc::c_int as libc::c_uint {
                x = setup_sx1272_FSK(frequency, rxbw_khz, rssi_offset)
            } else {
                x = setup_sx1276_FSK(frequency, rxbw_khz, rssi_offset)
            }
        }
        _ => {}
    }
    if x != 0i32 {
        return x;
    }
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
