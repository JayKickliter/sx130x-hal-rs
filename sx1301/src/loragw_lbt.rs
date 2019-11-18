use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn lgw_setup_sx127x(
        frequency: uint32_t,
        modulation: uint8_t,
        rxbw_khz: lgw_sx127x_rxbw_e,
        rssi_offset: int8_t,
    ) -> libc::c_int;
    /* *
    @brief Abort a currently scheduled or ongoing TX
    @return LGW_HAL_ERROR id the operation failed, LGW_HAL_SUCCESS else
    */
    /* *
    @brief Return value of internal counter when latest event (eg GPS pulse) was captured
    @param trig_cnt_us pointer to receive timestamp value
    @return LGW_HAL_ERROR id the operation failed, LGW_HAL_SUCCESS else
    */
    /* *
    @brief Allow user to check the version/options of the library once compiled
    @return pointer on a human-readable null terminated string
    */
    /* *
    @brief Return time on air of given packet, in milliseconds
    @param packet is a pointer to the packet structure
    @return the packet time on air in milliseconds
    */
    #[no_mangle]
    fn lgw_time_on_air(packet: *mut lgw_pkt_tx_s) -> uint32_t;
    #[no_mangle]
    fn lgw_get_trigcnt(trig_cnt_us: *mut uint32_t) -> libc::c_int;
    /* *
    @brief LoRa concentrator FPGA register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_fpga_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
    /* *
    @brief LoRa concentrator FPGA register read
    @param register_id register number in the data structure describing registers
    @param reg_value pointer to a variable where to write register read value
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_fpga_reg_r(register_id: uint16_t, reg_value: *mut int32_t) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
    Functions used to handle LoRa concentrator radios.

License: Revised BSD License, see LICENSE.TXT file include in the project
Maintainer: Michael Coracin
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
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
/* *
@struct lgw_conf_lbt_chan_s
@brief Configuration structure for LBT channels
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_lbt_chan_s {
    pub freq_hz: uint32_t,
    pub scan_time_us: uint16_t,
}
/* *
@struct lgw_conf_lbt_s
@brief Configuration structure for LBT specificities
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_lbt_s {
    pub enable: bool,
    pub rssi_target: int8_t,
    pub nb_channel: uint8_t,
    pub channels: [lgw_conf_lbt_chan_s; 8],
    pub rssi_offset: int8_t,
    /* !> RSSI offset to be applied to SX127x RSSI values */
}
/* *
@struct lgw_pkt_tx_s
@brief Structure containing the configuration of a packet to send and a pointer to the payload
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_pkt_tx_s {
    pub freq_hz: uint32_t,
    pub tx_mode: uint8_t,
    pub count_us: uint32_t,
    pub rf_chain: uint8_t,
    pub rf_power: int8_t,
    pub modulation: uint8_t,
    pub bandwidth: uint8_t,
    pub datarate: uint32_t,
    pub coderate: uint8_t,
    pub invert_pol: bool,
    pub f_dev: uint8_t,
    pub preamble: uint16_t,
    pub no_crc: bool,
    pub no_header: bool,
    pub size: uint16_t,
    pub payload: [uint8_t; 256],
    /* !> buffer containing the payload */
}
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
static mut lbt_enable: bool = false;
static mut lbt_nb_active_channel: uint8_t = 0;
static mut lbt_rssi_target_dBm: int8_t = 0;
static mut lbt_rssi_offset_dB: int8_t = 0;
static mut lbt_start_freq: uint32_t = 0;
static mut lbt_channel_cfg: [lgw_conf_lbt_chan_s; 8] = [lgw_conf_lbt_chan_s {
    freq_hz: 0,
    scan_time_us: 0,
}; 8];
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Set the configuration parameters for LBT feature
@param conf structure containing the configuration parameters
@return LGW_LBT_ERROR id the operation failed, LGW_LBT_SUCCESS else
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn lbt_setconf(mut conf: *mut lgw_conf_lbt_s) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Check input parameters */
    if conf.is_null() {
        return -1i32;
    }
    if ((*conf).nb_channel as libc::c_int) < 1i32 || (*conf).nb_channel as libc::c_int > 8i32 {
        return -1i32;
    }
    /* Initialize LBT channels configuration */
    memset(
        lbt_channel_cfg.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[lgw_conf_lbt_chan_s; 8]>() as libc::c_ulong,
    );
    /* Set internal LBT config according to parameters */
    lbt_enable = (*conf).enable;
    lbt_nb_active_channel = (*conf).nb_channel;
    lbt_rssi_target_dBm = (*conf).rssi_target;
    lbt_rssi_offset_dB = (*conf).rssi_offset;
    i = 0i32;
    while i < lbt_nb_active_channel as libc::c_int {
        lbt_channel_cfg[i as usize].freq_hz = (*conf).channels[i as usize].freq_hz;
        lbt_channel_cfg[i as usize].scan_time_us = (*conf).channels[i as usize].scan_time_us;
        i += 1
    }
    return 0i32;
}
/* *
@brief Configure the concentrator for LBT feature
@return LGW_LBT_ERROR id the operation failed, LGW_LBT_SUCCESS else
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lbt_setup() -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: int32_t = 0;
    let mut freq_offset: uint32_t = 0;
    /* Check if LBT feature is supported by FPGA */
    x = lgw_fpga_reg_r(1i32 as uint16_t, &mut val);
    if x != 0i32 {
        return -1i32;
    }
    if val as uint8_t as libc::c_int >> 2i32 & (1i32 << 1i32) - 1i32 != 1i32 {
        return -1i32;
    }
    /* Get FPGA lowest frequency for LBT channels */
    x = lgw_fpga_reg_r(2i32 as uint16_t, &mut val);
    if x != 0i32 {
        return -1i32;
    }
    match val {
        0 => lbt_start_freq = 915000000i32 as uint32_t,
        1 => lbt_start_freq = 863000000i32 as uint32_t,
        _ => return -1i32,
    }
    /* Configure SX127x for FSK */
    x = lgw_setup_sx127x(
        lbt_start_freq,
        0x20i32 as uint8_t,
        LGW_SX127X_RXBW_100K_HZ,
        lbt_rssi_offset_dB,
    ); /* 200KHz LBT channels */
    if x != 0i32 {
        return -1i32;
    }
    /* Configure FPGA for LBT */
    val = -2i32 * lbt_rssi_target_dBm as libc::c_int; /* Convert RSSI target in dBm to FPGA register format */
    x = lgw_fpga_reg_w(35i32 as uint16_t, val);
    if x != 0i32 {
        return -1i32;
    }
    /* Set default values for non-active LBT channels */
    i = lbt_nb_active_channel as libc::c_int;
    while i < 8i32 {
        lbt_channel_cfg[i as usize].freq_hz = lbt_start_freq;
        lbt_channel_cfg[i as usize].scan_time_us = 128i32 as uint16_t;
        i += 1
        /* fastest scan for non-active channels */
    }
    /* Configure FPGA for both active and non-active LBT channels */
    i = 0i32;
    while i < 8i32 {
        /* Check input parameters */
        if lbt_channel_cfg[i as usize].freq_hz < lbt_start_freq {
            return -1i32;
        }
        if lbt_channel_cfg[i as usize].scan_time_us as libc::c_int != 128i32
            && lbt_channel_cfg[i as usize].scan_time_us as libc::c_int != 5000i32
        {
            return -1i32;
        }
        /* Configure */
        freq_offset = (lbt_channel_cfg[i as usize]
            .freq_hz
            .wrapping_sub(lbt_start_freq) as libc::c_double
            / 100E3f64) as uint32_t; /* 100kHz unit */
        x = lgw_fpga_reg_w((18i32 + i) as uint16_t, freq_offset as int32_t);
        if x != 0i32 {
            return -1i32;
        }
        if lbt_channel_cfg[i as usize].scan_time_us as libc::c_int == 5000i32 {
            /* configured to 128 by default */
            x = lgw_fpga_reg_w((27i32 + i) as uint16_t, 1i32);
            if x != 0i32 {
                return -1i32;
            }
        }
        i += 1
    }
    i = 0i32;
    while i < 8i32 {
        i += 1
    }
    return 0i32;
}
/* *
@brief Start the LBT FSM
@return LGW_LBT_ERROR id the operation failed, LGW_LBT_SUCCESS else
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lbt_start() -> libc::c_int {
    let mut x: libc::c_int = 0;
    x = lgw_fpga_reg_w(5i32 as uint16_t, 1i32);
    if x != 0i32 {
        return -1i32;
    }
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lbt_is_channel_free(
    mut pkt_data: *mut lgw_pkt_tx_s,
    mut tx_start_delay: uint16_t,
    mut tx_allowed: *mut bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: int32_t = 0;
    let mut tx_start_time: uint32_t = 0i32 as uint32_t;
    let mut tx_end_time: uint32_t = 0i32 as uint32_t;
    let mut delta_time: uint32_t = 0i32 as uint32_t;
    let mut sx1301_time: uint32_t = 0i32 as uint32_t;
    let mut lbt_time: uint32_t = 0i32 as uint32_t;
    let mut lbt_time1: uint32_t = 0i32 as uint32_t;
    let mut lbt_time2: uint32_t = 0i32 as uint32_t;
    let mut tx_max_time: uint32_t = 0i32 as uint32_t;
    let mut lbt_channel_decod_1: libc::c_int = -1i32;
    let mut lbt_channel_decod_2: libc::c_int = -1i32;
    let mut packet_duration: uint32_t = 0i32 as uint32_t;
    /* Check input parameters */
    if pkt_data.is_null() || tx_allowed.is_null() {
        return -1i32;
    }
    /* Check if TX is allowed */
    if lbt_enable as libc::c_int == 1i32 {
        /* TX allowed for LoRa only */
        if (*pkt_data).modulation as libc::c_int != 0x10i32 {
            *tx_allowed = 0i32 != 0;
            return 0i32;
        }
        /* Get SX1301 time at last PPS */
        lgw_get_trigcnt(&mut sx1301_time);
        match (*pkt_data).tx_mode as libc::c_int {
            1 => tx_start_time = (*pkt_data).count_us & 0x7ff000i32 as libc::c_uint,
            2 => {
                tx_start_time = sx1301_time
                    .wrapping_add(tx_start_delay as uint32_t)
                    .wrapping_add(1000000i32 as libc::c_uint)
                    & 0x7ff000i32 as libc::c_uint
            }
            0 | _ => {
                /* FALLTHROUGH  */
                return -1i32;
            }
        }
        /* Select LBT Channel corresponding to required TX frequency */
        lbt_channel_decod_1 = -1i32;
        lbt_channel_decod_2 = -1i32;
        if (*pkt_data).bandwidth as libc::c_int == 0x3i32 {
            i = 0i32;
            while i < lbt_nb_active_channel as libc::c_int {
                if is_equal_freq((*pkt_data).freq_hz, lbt_channel_cfg[i as usize].freq_hz)
                    as libc::c_int
                    == 1i32
                {
                    lbt_channel_decod_1 = i;
                    lbt_channel_decod_2 = i;
                    if lbt_channel_cfg[i as usize].scan_time_us as libc::c_int == 5000i32 {
                        tx_max_time = 4000000i32 as uint32_t
                    /* 4 seconds */
                    } else {
                        tx_max_time = 400000i32 as uint32_t
                        /* 400 milliseconds */
                    }
                    break;
                } else {
                    i += 1
                }
            }
        } else if (*pkt_data).bandwidth as libc::c_int == 0x2i32 {
            /* In case of 250KHz, the TX freq has to be in between 2 consecutive channels of 200KHz BW.
            The TX can only be over 2 channels, not more */
            i = 0i32;
            while i < lbt_nb_active_channel as libc::c_int - 1i32 {
                if is_equal_freq(
                    (*pkt_data).freq_hz,
                    lbt_channel_cfg[i as usize]
                        .freq_hz
                        .wrapping_add(lbt_channel_cfg[(i + 1i32) as usize].freq_hz)
                        .wrapping_div(2i32 as libc::c_uint),
                ) as libc::c_int
                    == 1i32
                    && lbt_channel_cfg[(i + 1i32) as usize]
                        .freq_hz
                        .wrapping_sub(lbt_channel_cfg[i as usize].freq_hz)
                        as libc::c_double
                        == 200E3f64
                {
                    lbt_channel_decod_1 = i;
                    lbt_channel_decod_2 = i + 1i32;
                    if lbt_channel_cfg[i as usize].scan_time_us as libc::c_int == 5000i32 {
                        tx_max_time = 4000000i32 as uint32_t
                    /* 4 seconds */
                    } else {
                        tx_max_time = 200000i32 as uint32_t
                        /* 200 milliseconds */
                    }
                    break;
                } else {
                    i += 1
                }
            }
        }
        /* Get last time when selected channel was free */
        if lbt_channel_decod_1 >= 0i32 && lbt_channel_decod_2 >= 0i32 {
            lgw_fpga_reg_w(17i32 as uint16_t, lbt_channel_decod_1); /* 16bits (1LSB = 256µs) */
            lgw_fpga_reg_r(16i32 as uint16_t, &mut val); /* 16bits (1LSB = 256µs) */
            lbt_time1 = ((val & 0xffffi32) as uint32_t).wrapping_mul(256i32 as libc::c_uint);
            lbt_time = lbt_time1;
            if lbt_channel_decod_1 != lbt_channel_decod_2 {
                lgw_fpga_reg_w(17i32 as uint16_t, lbt_channel_decod_2);
                lgw_fpga_reg_r(16i32 as uint16_t, &mut val);
                lbt_time2 = ((val & 0xffffi32) as uint32_t).wrapping_mul(256i32 as libc::c_uint);
                if lbt_time2 < lbt_time1 {
                    lbt_time = lbt_time2
                }
            }
        } else {
            lbt_time = 0i32 as uint32_t
        }
        packet_duration =
            (lgw_time_on_air(pkt_data) as libc::c_ulong).wrapping_mul(1000u64) as uint32_t;
        tx_end_time = tx_start_time.wrapping_add(packet_duration) & 0x7ff000i32 as libc::c_uint;
        if lbt_time < tx_end_time {
            delta_time = tx_end_time.wrapping_sub(lbt_time)
        } else {
            /* It means LBT counter has wrapped */
            printf(b"LBT: lbt counter has wrapped\n\x00" as *const u8 as *const libc::c_char);
            delta_time = (0x7ff000i32 as libc::c_uint)
                .wrapping_sub(lbt_time)
                .wrapping_add(tx_end_time)
        }
        /* send data if allowed */
        /* lbt_time: last time when channel was free */
        /* tx_max_time: maximum time allowed to send packet since last free time */
        /* 2048: some margin */
        if delta_time < tx_max_time.wrapping_sub(2048i32 as libc::c_uint)
            && lbt_time != 0i32 as libc::c_uint
        {
            *tx_allowed = 1i32 != 0
        } else {
            *tx_allowed = 0i32 != 0
        }
    } else {
        /* Always allow if LBT is disabled */
        *tx_allowed = 1i32 != 0
    }
    return 0i32;
}
/* *
@brief Configure the concentrator for LBT feature
@param pkt_data pointer to downlink packet to be trabsmitted
@param tx_allowed pointer to receive permission for transmission
@return LGW_LBT_ERROR id the operation failed, LGW_LBT_SUCCESS else
*/
/* *
@brief Check if LBT is enabled
@return true if enabled, false otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lbt_is_enabled() -> bool {
    return lbt_enable;
}
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* As given frequencies have been converted from float to integer, some aliasing
issues can appear, so we can't simply check for equality, but have to take some
margin */
#[no_mangle]
pub unsafe extern "C" fn is_equal_freq(mut a: uint32_t, mut b: uint32_t) -> bool {
    let mut diff: int64_t = 0;
    let mut a64: int64_t = a as int64_t;
    let mut b64: int64_t = b as int64_t;
    /* Calculate the difference */
    diff = llabs((a64 - b64) as libc::c_longlong) as int64_t;
    /* Check for acceptable diff range */
    if diff <= 10000i32 as libc::c_long {
        return 1i32 != 0;
    }
    return 0i32 != 0;
}
/* --- EOF ------------------------------------------------------------------ */
