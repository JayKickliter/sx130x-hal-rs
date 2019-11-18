use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    /* *
    @brief LoRa concentrator register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
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
    /* *
    @brief TODO
    @param TODO
    @return TODO
    */
    #[no_mangle]
    fn sx1302_agc_wait_status(status: uint8_t) -> libc::c_int;
    /* *
    @brief TODO
    @param TODO
    @return TODO
    */
    #[no_mangle]
    fn sx1302_agc_mailbox_read(mailbox: uint8_t, value: *mut uint8_t) -> libc::c_int;
    /* *
    @brief TODO
    @param TODO
    @return TODO
    */
    #[no_mangle]
    fn sx1302_agc_mailbox_write(mailbox: uint8_t, value: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn lgw_sx125x_reg_w(idx: radio_reg_t, data: uint8_t, rf_chain: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn lgw_sx125x_reg_r(idx: radio_reg_t, data: *mut uint8_t, rf_chain: uint8_t) -> libc::c_int;
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
pub type lgw_radio_type_t = libc::c_uint;
pub const LGW_RADIO_TYPE_SX1250: lgw_radio_type_t = 5;
pub const LGW_RADIO_TYPE_SX1276: lgw_radio_type_t = 4;
pub const LGW_RADIO_TYPE_SX1272: lgw_radio_type_t = 3;
pub const LGW_RADIO_TYPE_SX1257: lgw_radio_type_t = 2;
pub const LGW_RADIO_TYPE_SX1255: lgw_radio_type_t = 1;
pub const LGW_RADIO_TYPE_NONE: lgw_radio_type_t = 0;
/* *
@struct lgw_rssi_tcomp_s
@brief Structure containing all coefficients necessary to compute the offset to be applied on RSSI for current temperature
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_rssi_tcomp_s {
    pub coeff_a: libc::c_float,
    pub coeff_b: libc::c_float,
    pub coeff_c: libc::c_float,
    pub coeff_d: libc::c_float,
    pub coeff_e: libc::c_float,
}
/* *
@struct lgw_conf_rxrf_s
@brief Configuration structure for a RF chain
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_rxrf_s {
    pub enable: bool,
    pub freq_hz: uint32_t,
    pub rssi_offset: libc::c_float,
    pub rssi_tcomp: lgw_rssi_tcomp_s,
    pub type_0: lgw_radio_type_t,
    pub tx_enable: bool,
    /* !> enable or disable TX on that RF chain */
}
/* *
@struct lgw_tx_gain_s
@brief Structure containing all gains of Tx chain
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_tx_gain_s {
    pub rf_power: int8_t,
    pub dig_gain: uint8_t,
    pub pa_gain: uint8_t,
    pub dac_gain: uint8_t,
    pub mix_gain: uint8_t,
    pub offset_i: int8_t,
    pub offset_q: int8_t,
    pub pwr_idx: uint8_t,
    /* !> (sx1250) 6 bits: control the radio power index to be used for configuration */
}
/* *
@struct lgw_tx_gain_lut_s
@brief Structure defining the Tx gain LUT
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_tx_gain_lut_s {
    pub lut: [lgw_tx_gain_s; 16],
    pub size: uint8_t,
    /* !> Number of LUT indexes */
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
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    LoRa concentrator radio calibration functions

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types*/
/* library configuration options (dynamically generated) */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC MACROS -------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC TYPES --------------------------------------------------------- */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_sx125x_cal_rx_result_s {
    pub amp: int8_t,
    pub phi: int8_t,
    pub rej: uint16_t,
    pub rej_init: uint16_t,
    pub snr: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_sx125x_cal_tx_result_s {
    pub dac_gain: uint8_t,
    pub mix_gain: uint8_t,
    pub offset_i: int8_t,
    pub offset_q: int8_t,
    pub rej: uint16_t,
    pub sig: uint16_t,
}
/* 0:1ms, 1:2ms, 2:4ms, 3:8ms */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
/* Record Rx IQ mismatch corrections from calibration */
static mut rf_rx_image_amp: [int8_t; 2] = [0i32 as int8_t, 0i32 as int8_t];
static mut rf_rx_image_phi: [int8_t; 2] = [0i32 as int8_t, 0i32 as int8_t];
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_cal_start(
    mut version: uint8_t,
    mut rf_chain_cfg: *mut lgw_conf_rxrf_s,
    mut txgain_lut: *mut lgw_tx_gain_lut_s,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut val: uint8_t = 0;
    let mut cal_status: bool = 0i32 != 0;
    let mut x_max: uint8_t = 0;
    let mut x_max_idx: libc::c_int = 0;
    let mut dac_gain: [[uint8_t; 16]; 2] = [[0; 16]; 2];
    let mut mix_gain: [[uint8_t; 16]; 2] = [[0; 16]; 2];
    let mut offset_i: [[int8_t; 16]; 2] = [[0; 16]; 2];
    let mut offset_q: [[int8_t; 16]; 2] = [[0; 16]; 2];
    let mut nb_gains: [uint8_t; 2] = [0; 2];
    let mut unique_gains: bool = false;
    let mut cal_rx: [lgw_sx125x_cal_rx_result_s; 3] = [lgw_sx125x_cal_rx_result_s {
        amp: 0,
        phi: 0,
        rej: 0,
        rej_init: 0,
        snr: 0,
    }; 3];
    let mut cal_rx_min: lgw_sx125x_cal_rx_result_s = lgw_sx125x_cal_rx_result_s {
        amp: 0,
        phi: 0,
        rej: 0,
        rej_init: 0,
        snr: 0,
    };
    let mut cal_rx_max: lgw_sx125x_cal_rx_result_s = lgw_sx125x_cal_rx_result_s {
        amp: 0,
        phi: 0,
        rej: 0,
        rej_init: 0,
        snr: 0,
    };
    let mut cal_tx: [lgw_sx125x_cal_tx_result_s; 3] = [lgw_sx125x_cal_tx_result_s {
        dac_gain: 0,
        mix_gain: 0,
        offset_i: 0,
        offset_q: 0,
        rej: 0,
        sig: 0,
    }; 3];
    let mut cal_tx_min: lgw_sx125x_cal_tx_result_s = lgw_sx125x_cal_tx_result_s {
        dac_gain: 0,
        mix_gain: 0,
        offset_i: 0,
        offset_q: 0,
        rej: 0,
        sig: 0,
    };
    let mut cal_tx_max: lgw_sx125x_cal_tx_result_s = lgw_sx125x_cal_tx_result_s {
        dac_gain: 0,
        mix_gain: 0,
        offset_i: 0,
        offset_q: 0,
        rej: 0,
        sig: 0,
    };
    /* Wait for AGC fw to be started, and VERSION available in mailbox */
    sx1302_agc_wait_status(0x1i32 as uint8_t); /* fw has started, VERSION is ready in mailbox */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != version as libc::c_int {
        printf(
            b"ERROR: wrong CAL fw version (%d)\n\x00" as *const u8 as *const libc::c_char,
            val as libc::c_int,
        );
        return -1i32;
    }
    printf(
        b"CAL FW VERSION: %d\n\x00" as *const u8 as *const libc::c_char,
        val as libc::c_int,
    );
    /* notify CAL that it can resume */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0xffi32 as uint8_t);
    /* Wait for AGC to acknoledge */
    sx1302_agc_wait_status(0i32 as uint8_t);
    printf(b"CAL: started\n\x00" as *const u8 as *const libc::c_char);
    /* Run Rx image calibration */
    i = 0i32;
    while i < 2i32 {
        if (*rf_chain_cfg.offset(i as isize)).enable {
            /* Calibration using the other radio for Tx */
            if (*rf_chain_cfg.offset(0)).type_0 as libc::c_uint
                == (*rf_chain_cfg.offset(1)).type_0 as libc::c_uint
            {
                cal_rx_result_init(&mut cal_rx_min, &mut cal_rx_max);
                j = 0i32;
                while j < 3i32 {
                    sx125x_cal_rx_image(
                        i as uint8_t,
                        (*rf_chain_cfg.offset(i as isize)).freq_hz,
                        0i32 != 0,
                        (*rf_chain_cfg.offset(i as isize)).type_0 as uint8_t,
                        &mut *cal_rx.as_mut_ptr().offset(j as isize),
                    );
                    cal_rx_result_sort(
                        &mut *cal_rx.as_mut_ptr().offset(j as isize),
                        &mut cal_rx_min,
                        &mut cal_rx_max,
                    );
                    j += 1
                }
                cal_status = cal_rx_result_assert(&mut cal_rx_min, &mut cal_rx_max)
            }
            /* If failed or different radios, run calibration using RF loopback (assuming that it is better than no calibration) */
            if cal_status as libc::c_int == 0i32
                || (*rf_chain_cfg.offset(0)).type_0 as libc::c_uint
                    != (*rf_chain_cfg.offset(1)).type_0 as libc::c_uint
            {
                cal_rx_result_init(&mut cal_rx_min, &mut cal_rx_max);
                j = 0i32;
                while j < 3i32 {
                    sx125x_cal_rx_image(
                        i as uint8_t,
                        (*rf_chain_cfg.offset(i as isize)).freq_hz,
                        1i32 != 0,
                        (*rf_chain_cfg.offset(i as isize)).type_0 as uint8_t,
                        &mut *cal_rx.as_mut_ptr().offset(j as isize),
                    );
                    cal_rx_result_sort(
                        &mut *cal_rx.as_mut_ptr().offset(j as isize),
                        &mut cal_rx_min,
                        &mut cal_rx_max,
                    );
                    j += 1
                }
                cal_status = cal_rx_result_assert(&mut cal_rx_min, &mut cal_rx_max)
            }
            if cal_status as libc::c_int == 0i32 {
                return -1i32;
            }
            /* Use the results of the best iteration */
            x_max = 0i32 as uint8_t;
            x_max_idx = 0i32;
            j = 0i32;
            while j < 3i32 {
                if cal_rx[j as usize].rej as libc::c_int > x_max as libc::c_int {
                    x_max = cal_rx[j as usize].rej as uint8_t;
                    x_max_idx = j
                }
                j += 1
            }
            rf_rx_image_amp[i as usize] = cal_rx[x_max_idx as usize].amp;
            rf_rx_image_phi[i as usize] = cal_rx[x_max_idx as usize].phi
        } else {
            rf_rx_image_amp[i as usize] = 0i32 as int8_t;
            rf_rx_image_phi[i as usize] = 0i32 as int8_t
        }
        i += 1
    }
    /* Apply calibrated IQ mismatch compensation */
    lgw_reg_w(795i32 as uint16_t, rf_rx_image_amp[0] as int32_t);
    lgw_reg_w(796i32 as uint16_t, rf_rx_image_phi[0] as int32_t);
    lgw_reg_w(810i32 as uint16_t, rf_rx_image_amp[1] as int32_t);
    lgw_reg_w(811i32 as uint16_t, rf_rx_image_phi[1] as int32_t);
    /* Get List of unique combinations of DAC and mixer gains */
    k = 0i32;
    while k < 2i32 {
        nb_gains[k as usize] = 0i32 as uint8_t;
        i = 0i32;
        while i < (*txgain_lut.offset(k as isize)).size as libc::c_int {
            unique_gains = 1i32 != 0;
            j = 0i32;
            while j < nb_gains[k as usize] as libc::c_int {
                if (*txgain_lut.offset(k as isize)).lut[i as usize].dac_gain as libc::c_int
                    == dac_gain[k as usize][j as usize] as libc::c_int
                    && (*txgain_lut.offset(k as isize)).lut[i as usize].mix_gain as libc::c_int
                        == mix_gain[k as usize][j as usize] as libc::c_int
                {
                    unique_gains = 0i32 != 0
                }
                j += 1
            }
            if unique_gains {
                dac_gain[k as usize][nb_gains[k as usize] as usize] =
                    (*txgain_lut.offset(k as isize)).lut[i as usize].dac_gain;
                mix_gain[k as usize][nb_gains[k as usize] as usize] =
                    (*txgain_lut.offset(k as isize)).lut[i as usize].mix_gain;
                nb_gains[k as usize] = (nb_gains[k as usize] as libc::c_int + 1i32) as uint8_t
            }
            i += 1
        }
        k += 1
    }
    /* Run Tx image calibration */
    i = 0i32;
    while i < 2i32 {
        if (*rf_chain_cfg.offset(i as isize)).tx_enable {
            j = 0i32;
            while j < nb_gains[i as usize] as libc::c_int {
                cal_tx_result_init(&mut cal_tx_min, &mut cal_tx_max);
                k = 0i32;
                while k < 3i32 {
                    sx125x_cal_tx_dc_offset(
                        i as uint8_t,
                        (*rf_chain_cfg.offset(i as isize)).freq_hz,
                        dac_gain[i as usize][j as usize],
                        mix_gain[i as usize][j as usize],
                        (*rf_chain_cfg.offset(i as isize)).type_0 as uint8_t,
                        &mut *cal_tx.as_mut_ptr().offset(k as isize),
                    );
                    cal_tx_result_sort(
                        &mut *cal_tx.as_mut_ptr().offset(k as isize),
                        &mut cal_tx_min,
                        &mut cal_tx_max,
                    );
                    k += 1
                }
                cal_status = cal_tx_result_assert(&mut cal_tx_min, &mut cal_tx_max);
                if cal_status as libc::c_int == 0i32 {
                    return -1i32;
                }
                /* Use the results of the best iteration */
                x_max = 0i32 as uint8_t;
                x_max_idx = 0i32;
                k = 0i32;
                while k < 3i32 {
                    if cal_tx[k as usize].rej as libc::c_int > x_max as libc::c_int {
                        x_max = cal_tx[k as usize].rej as uint8_t;
                        x_max_idx = k
                    }
                    k += 1
                }
                offset_i[i as usize][j as usize] = cal_tx[x_max_idx as usize].offset_i;
                offset_q[i as usize][j as usize] = cal_tx[x_max_idx as usize].offset_q;
                j += 1
            }
        }
        i += 1
    }
    /* Fill DC offsets in Tx LUT */
    k = 0i32;
    while k < 2i32 {
        i = 0i32;
        while i < (*txgain_lut.offset(k as isize)).size as libc::c_int {
            j = 0i32;
            while j < nb_gains[k as usize] as libc::c_int {
                if (*txgain_lut.offset(k as isize)).lut[i as usize].dac_gain as libc::c_int
                    == dac_gain[k as usize][j as usize] as libc::c_int
                    && (*txgain_lut.offset(k as isize)).lut[i as usize].mix_gain as libc::c_int
                        == mix_gain[k as usize][j as usize] as libc::c_int
                {
                    break;
                }
                j += 1
            }
            (*txgain_lut.offset(k as isize)).lut[i as usize].offset_i =
                offset_i[k as usize][j as usize];
            (*txgain_lut.offset(k as isize)).lut[i as usize].offset_q =
                offset_q[k as usize][j as usize];
            i += 1
        }
        k += 1
    }
    printf(
        b"-------------------------------------------------------------------\n\x00" as *const u8
            as *const libc::c_char,
    );
    printf(b"Radio calibration completed:\n\x00" as *const u8 as *const libc::c_char);
    printf(
        b"  RadioA: amp:%d phi:%d\n\x00" as *const u8 as *const libc::c_char,
        rf_rx_image_amp[0] as libc::c_int,
        rf_rx_image_phi[0] as libc::c_int,
    );
    printf(
        b"  RadioB: amp:%d phi:%d\n\x00" as *const u8 as *const libc::c_char,
        rf_rx_image_amp[1] as libc::c_int,
        rf_rx_image_phi[1] as libc::c_int,
    );
    k = 0i32;
    while k < 2i32 {
        printf(
            b"  TX calibration params for rf_chain %d:\n\x00" as *const u8 as *const libc::c_char,
            k,
        );
        i = 0i32;
        while i < (*txgain_lut.offset(k as isize)).size as libc::c_int {
            printf(
                b"  -- power:%d\tdac:%u\tmix:%u\toffset_i:%d\toffset_q:%d\n\x00" as *const u8
                    as *const libc::c_char,
                (*txgain_lut.offset(k as isize)).lut[i as usize].rf_power as libc::c_int,
                (*txgain_lut.offset(k as isize)).lut[i as usize].dac_gain as libc::c_int,
                (*txgain_lut.offset(k as isize)).lut[i as usize].mix_gain as libc::c_int,
                (*txgain_lut.offset(k as isize)).lut[i as usize].offset_i as libc::c_int,
                (*txgain_lut.offset(k as isize)).lut[i as usize].offset_q as libc::c_int,
            );
            i += 1
        }
        k += 1
    }
    printf(
        b"-------------------------------------------------------------------\n\x00" as *const u8
            as *const libc::c_char,
    );
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx125x_cal_rx_image(
    mut rf_chain: uint8_t,
    mut freq_hz: uint32_t,
    mut use_loopback: bool,
    mut radio_type: uint8_t,
    mut res: *mut lgw_sx125x_cal_rx_result_s,
) -> libc::c_int {
    let mut rx: uint8_t = 0; /* Used by AGC to set decimation gain to increase signal and its image: value is MSB => x * 256 */
    let mut tx: uint8_t = 0;
    let mut rx_freq_hz: uint32_t = 0;
    let mut tx_freq_hz: uint32_t = 0;
    let mut rx_freq_int: uint32_t = 0;
    let mut rx_freq_frac: uint32_t = 0;
    let mut tx_freq_int: uint32_t = 0;
    let mut tx_freq_frac: uint32_t = 0;
    let mut rx_pll_locked: uint8_t = 0;
    let mut tx_pll_locked: uint8_t = 0;
    let mut rx_threshold: uint8_t = 8i32 as uint8_t;
    printf(
        b"\n%s: rf_chain:%u, freq_hz:%u, loopback:%d, radio_type:%d\n\x00" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sx125x_cal_rx_image\x00"))
            .as_ptr(),
        rf_chain as libc::c_int,
        freq_hz,
        use_loopback as libc::c_int,
        radio_type as libc::c_int,
    );
    /* Indentify which radio is transmitting the test tone */
    rx = rf_chain;
    if use_loopback as libc::c_int == 1i32 {
        tx = rf_chain
    } else {
        tx = (1i32 - rf_chain as libc::c_int) as uint8_t
    }
    /* Set PLL frequencies */
    rx_freq_hz = freq_hz; /* integer part, gives the MSB */
    tx_freq_hz = freq_hz.wrapping_add(250000i32 as libc::c_uint); /* fractional part, gives middle part and LSB */
    match radio_type as libc::c_int {
        1 => {
            rx_freq_int = rx_freq_hz.wrapping_div((15625i32 << 7i32) as libc::c_uint); /* integer part, gives the MSB */
            rx_freq_frac = (rx_freq_hz.wrapping_rem((15625i32 << 7i32) as libc::c_uint) << 9i32)
                .wrapping_div(15625i32 as libc::c_uint); /* fractional part, gives middle part and LSB */
            tx_freq_int = tx_freq_hz.wrapping_div((15625i32 << 7i32) as libc::c_uint); /* integer part, gives the MSB */
            tx_freq_frac = (tx_freq_hz.wrapping_rem((15625i32 << 7i32) as libc::c_uint) << 9i32)
                .wrapping_div(15625i32 as libc::c_uint)
        }
        2 => {
            rx_freq_int = rx_freq_hz.wrapping_div((15625i32 << 8i32) as libc::c_uint); /* fractional part, gives middle part and LSB */
            rx_freq_frac = (rx_freq_hz.wrapping_rem((15625i32 << 8i32) as libc::c_uint) << 8i32)
                .wrapping_div(15625i32 as libc::c_uint); /* integer part, gives the MSB */
            tx_freq_int = tx_freq_hz.wrapping_div((15625i32 << 8i32) as libc::c_uint); /* fractional part, gives middle part and LSB */
            tx_freq_frac = (tx_freq_hz.wrapping_rem((15625i32 << 8i32) as libc::c_uint) << 8i32)
                .wrapping_div(15625i32 as libc::c_uint)
        }
        _ => return -1i32,
    }
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_MSB,
        (0xffi32 as libc::c_uint & rx_freq_int) as uint8_t,
        rx,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_MID,
        (0xffi32 as libc::c_uint & rx_freq_frac >> 8i32) as uint8_t,
        rx,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_LSB,
        (0xffi32 as libc::c_uint & rx_freq_frac) as uint8_t,
        rx,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_MSB,
        (0xffi32 as libc::c_uint & tx_freq_int) as uint8_t,
        tx,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_MID,
        (0xffi32 as libc::c_uint & tx_freq_frac >> 8i32) as uint8_t,
        tx,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_LSB,
        (0xffi32 as libc::c_uint & tx_freq_frac) as uint8_t,
        tx,
    );
    /* Radio settings for calibration */
    //sx125x_reg_w(SX125x_RX_ANA_GAIN__LNA_ZIN, 1, rx); /* Default: 1 */
    //sx125x_reg_w(SX125x_RX_ANA_GAIN__BB_GAIN, 15, rx); /* Default: 15 */
    //sx125x_reg_w(SX125x_RX_ANA_GAIN__LNA_GAIN, 1, rx); /* Default: 1 */
    lgw_sx125x_reg_w(SX125x_REG_RX_BW__BB_BW, 0i32 as uint8_t, rx);
    lgw_sx125x_reg_w(SX125x_REG_RX_BW__ADC_TRIM, 6i32 as uint8_t, rx);
    //sx125x_reg_w(SX125x_RX_BW__ADC_BW, 7, rx);  /* Default: 7 */
    lgw_sx125x_reg_w(SX125x_REG_RX_PLL_BW__PLL_BW, 0i32 as uint8_t, rx);
    lgw_sx125x_reg_w(SX125x_REG_TX_BW__PLL_BW, 0i32 as uint8_t, tx);
    //sx125x_reg_w(SX125x_TX_BW__ANA_BW, 0, tx); /* Default: 0 */
    lgw_sx125x_reg_w(SX125x_REG_TX_DAC_BW, 5i32 as uint8_t, tx);
    //sx125x_reg_w(SX125x_CLK_SELECT__DAC_CLK_SELECT, 0, tx); /* Use internal clock, in case no Tx connection from SX1302, Default: 0  */
    if use_loopback as libc::c_int == 1i32 {
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__DAC_GAIN, 3i32 as uint8_t, tx); //8
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__MIX_GAIN, 10i32 as uint8_t, tx);
        lgw_sx125x_reg_w(SX125x_REG_CLK_SELECT__RF_LOOPBACK_EN, 1i32 as uint8_t, tx);
        lgw_sx125x_reg_w(SX125x_REG_MODE, 15i32 as uint8_t, tx);
    } else {
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__DAC_GAIN, 3i32 as uint8_t, tx);
        lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__MIX_GAIN, 15i32 as uint8_t, tx);
        lgw_sx125x_reg_w(SX125x_REG_MODE, 3i32 as uint8_t, rx);
        lgw_sx125x_reg_w(SX125x_REG_MODE, 13i32 as uint8_t, tx);
    }
    wait_ms(10i32 as libc::c_ulong);
    lgw_sx125x_reg_r(
        SX125x_REG_MODE_STATUS__RX_PLL_LOCKED,
        &mut rx_pll_locked,
        rx,
    );
    lgw_sx125x_reg_r(
        SX125x_REG_MODE_STATUS__TX_PLL_LOCKED,
        &mut tx_pll_locked,
        tx,
    );
    if rx_pll_locked as libc::c_int == 0i32 || tx_pll_locked as libc::c_int == 0i32 {
        return -1i32;
    }
    /* Trig calibration */
    /* Select radio to be connected to the Signal Analyzer (warning: RadioA:1, RadioB:0) */
    lgw_reg_w(
        820i32 as uint16_t,
        if rf_chain as libc::c_int == 0i32 {
            1i32
        } else {
            0i32
        },
    );
    /* Set calibration parameters */
    sx1302_agc_mailbox_write(2i32 as uint8_t, rf_chain); /* Set RX test config: radioA:0 radioB:1 */
    sx1302_agc_mailbox_write(
        1i32 as uint8_t,
        (250000i32 as libc::c_double * 64e-6f64) as uint8_t,
    ); /* Set frequency */
    sx1302_agc_mailbox_write(0i32 as uint8_t, 0i32 as uint8_t); /* dec_gain (not used) */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0i32 as uint8_t);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x1i32 as uint8_t);
    sx1302_agc_wait_status(0x1i32 as uint8_t);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x2i32 as uint8_t);
    sx1302_agc_wait_status(0x2i32 as uint8_t);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x3i32 as uint8_t);
    sx1302_agc_wait_status(0x3i32 as uint8_t);
    sx1302_agc_mailbox_write(2i32 as uint8_t, 0i32 as uint8_t);
    sx1302_agc_mailbox_write(1i32 as uint8_t, rx_threshold);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x4i32 as uint8_t);
    /* Get calibration results */
    sx1302_agc_wait_status(0x6i32 as uint8_t);
    let mut threshold: uint8_t = 0;
    let mut cal_dec_gain: uint8_t = 0;
    let mut rx_sig_1: uint8_t = 0;
    let mut rx_sig_0: uint8_t = 0;
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut threshold);
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut cal_dec_gain);
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut rx_sig_1);
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut rx_sig_0);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x6i32 as uint8_t);
    sx1302_agc_wait_status(0x7i32 as uint8_t);
    let mut rx_img_init_0: uint8_t = 0;
    let mut rx_img_init_1: uint8_t = 0;
    let mut amp: uint8_t = 0;
    let mut phi: uint8_t = 0;
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut rx_img_init_1);
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut rx_img_init_0);
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut amp);
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut phi);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x7i32 as uint8_t);
    sx1302_agc_wait_status(0x8i32 as uint8_t);
    let mut rx_img_0: uint8_t = 0;
    let mut rx_img_1: uint8_t = 0;
    let mut rx_noise_raw_0: uint8_t = 0;
    let mut rx_noise_raw_1: uint8_t = 0;
    let mut rx_img: libc::c_float = 0.;
    let mut rx_noise_raw: libc::c_float = 0.;
    let mut rx_img_init: libc::c_float = 0.;
    let mut rx_sig: libc::c_float = 0.;
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut rx_img_1);
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut rx_img_0);
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut rx_noise_raw_1);
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut rx_noise_raw_0);
    rx_sig = rx_sig_1 as libc::c_float * 256i32 as libc::c_float + rx_sig_0 as libc::c_float;
    rx_noise_raw =
        rx_noise_raw_1 as libc::c_float * 256i32 as libc::c_float + rx_noise_raw_0 as libc::c_float;
    rx_img_init =
        rx_img_init_1 as libc::c_float * 256i32 as libc::c_float + rx_img_init_0 as libc::c_float;
    rx_img = rx_img_1 as libc::c_float * 256i32 as libc::c_float + rx_img_0 as libc::c_float;
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x8i32 as uint8_t);
    (*res).amp = amp as int8_t;
    (*res).phi = phi as int8_t;
    (*res).snr =
        (20i32 as libc::c_double * log10((rx_sig / rx_noise_raw) as libc::c_double)) as uint16_t;
    (*res).rej_init =
        (20i32 as libc::c_double * log10((rx_sig / rx_img_init) as libc::c_double)) as uint16_t;
    (*res).rej = (20i32 as libc::c_double * log10((rx_sig / rx_img) as libc::c_double)) as uint16_t;
    /* Wait for calibration to be completed */
    sx1302_agc_wait_status(if rf_chain as libc::c_int == 0i32 {
        0x11i32
    } else {
        0x22i32
    } as uint8_t);
    printf(
        b"%s, RESULT: rf_chain:%u amp:%d phi:%d\n\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sx125x_cal_rx_image\x00"))
            .as_ptr(),
        rf_chain as libc::c_int,
        (*res).amp as libc::c_int,
        (*res).phi as libc::c_int,
    );
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx125x_cal_tx_dc_offset(
    mut rf_chain: uint8_t,
    mut freq_hz: uint32_t,
    mut dac_gain: uint8_t,
    mut mix_gain: uint8_t,
    mut radio_type: uint8_t,
    mut res: *mut lgw_sx125x_cal_tx_result_s,
) -> libc::c_int {
    let mut rx_freq_hz: uint32_t = 0;
    let mut tx_freq_hz: uint32_t = 0;
    let mut rx_freq_int: uint32_t = 0;
    let mut rx_freq_frac: uint32_t = 0;
    let mut tx_freq_int: uint32_t = 0;
    let mut tx_freq_frac: uint32_t = 0;
    let mut rx_pll_locked: uint8_t = 0;
    let mut tx_pll_locked: uint8_t = 0;
    let mut reg: uint16_t = 0;
    let mut tx_threshold: uint8_t = 64i32 as uint8_t;
    let mut i: libc::c_int = 0;
    printf(
        b"\n%s: rf_chain:%u, freq_hz:%u, dac_gain:%u, mix_gain:%u, radio_type:%d\n\x00" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sx125x_cal_tx_dc_offset\x00"))
            .as_ptr(),
        rf_chain as libc::c_int,
        freq_hz,
        dac_gain as libc::c_int,
        mix_gain as libc::c_int,
        radio_type as libc::c_int,
    );
    /* Set PLL frequencies */
    rx_freq_hz = freq_hz.wrapping_sub(250000i32 as libc::c_uint); /* integer part, gives the MSB */
    tx_freq_hz = freq_hz; /* fractional part, gives middle part and LSB */
    match radio_type as libc::c_int {
        1 => {
            rx_freq_int = rx_freq_hz.wrapping_div((15625i32 << 7i32) as libc::c_uint); /* integer part, gives the MSB */
            rx_freq_frac = (rx_freq_hz.wrapping_rem((15625i32 << 7i32) as libc::c_uint) << 9i32)
                .wrapping_div(15625i32 as libc::c_uint); /* fractional part, gives middle part and LSB */
            tx_freq_int = tx_freq_hz.wrapping_div((15625i32 << 7i32) as libc::c_uint); /* integer part, gives the MSB */
            tx_freq_frac = (tx_freq_hz.wrapping_rem((15625i32 << 7i32) as libc::c_uint) << 9i32)
                .wrapping_div(15625i32 as libc::c_uint)
        }
        2 => {
            rx_freq_int = rx_freq_hz.wrapping_div((15625i32 << 8i32) as libc::c_uint); /* fractional part, gives middle part and LSB */
            rx_freq_frac = (rx_freq_hz.wrapping_rem((15625i32 << 8i32) as libc::c_uint) << 8i32)
                .wrapping_div(15625i32 as libc::c_uint); /* integer part, gives the MSB */
            tx_freq_int = tx_freq_hz.wrapping_div((15625i32 << 8i32) as libc::c_uint); /* fractional part, gives middle part and LSB */
            tx_freq_frac = (tx_freq_hz.wrapping_rem((15625i32 << 8i32) as libc::c_uint) << 8i32)
                .wrapping_div(15625i32 as libc::c_uint)
        }
        _ => return -1i32,
    }
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_MSB,
        (0xffi32 as libc::c_uint & rx_freq_int) as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_MID,
        (0xffi32 as libc::c_uint & rx_freq_frac >> 8i32) as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_RX_LSB,
        (0xffi32 as libc::c_uint & rx_freq_frac) as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_MSB,
        (0xffi32 as libc::c_uint & tx_freq_int) as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_MID,
        (0xffi32 as libc::c_uint & tx_freq_frac >> 8i32) as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(
        SX125x_REG_FRF_TX_LSB,
        (0xffi32 as libc::c_uint & tx_freq_frac) as uint8_t,
        rf_chain,
    );
    /* Radio settings for calibration */
    //lgw_sx125x_reg_w(SX125x_RX_ANA_GAIN__LNA_ZIN, 1, rf_chain); /* Default: 1 */
    //lgw_sx125x_reg_w(SX125x_RX_ANA_GAIN__BB_GAIN, 15, rf_chain); /* Default: 15 */
    //lgw_sx125x_reg_w(SX125x_RX_ANA_GAIN__LNA_GAIN, 1, rf_chain); /* Default: 1 */
    lgw_sx125x_reg_w(SX125x_REG_RX_BW__BB_BW, 0i32 as uint8_t, rf_chain);
    lgw_sx125x_reg_w(SX125x_REG_RX_BW__ADC_TRIM, 6i32 as uint8_t, rf_chain);
    //lgw_sx125x_reg_w(SX125x_RX_BW__ADC_BW, 7, rf_chain);  /* Default: 7 */
    lgw_sx125x_reg_w(SX125x_REG_RX_PLL_BW__PLL_BW, 0i32 as uint8_t, rf_chain);
    lgw_sx125x_reg_w(SX125x_REG_TX_BW__PLL_BW, 0i32 as uint8_t, rf_chain);
    //lgw_sx125x_reg_w(SX125x_TX_BW__ANA_BW, 0, rf_chain); /* Default: 0 */
    lgw_sx125x_reg_w(SX125x_REG_TX_DAC_BW, 5i32 as uint8_t, rf_chain); /* Use external clock from SX1302 */
    lgw_sx125x_reg_w(
        SX125x_REG_CLK_SELECT__DAC_CLK_SELECT,
        1i32 as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__DAC_GAIN, dac_gain, rf_chain);
    lgw_sx125x_reg_w(SX125x_REG_TX_GAIN__MIX_GAIN, mix_gain, rf_chain);
    lgw_sx125x_reg_w(
        SX125x_REG_CLK_SELECT__RF_LOOPBACK_EN,
        1i32 as uint8_t,
        rf_chain,
    );
    lgw_sx125x_reg_w(SX125x_REG_MODE, 15i32 as uint8_t, rf_chain);
    wait_ms(1i32 as libc::c_ulong);
    lgw_sx125x_reg_r(
        SX125x_REG_MODE_STATUS__RX_PLL_LOCKED,
        &mut rx_pll_locked,
        rf_chain,
    );
    lgw_sx125x_reg_r(
        SX125x_REG_MODE_STATUS__TX_PLL_LOCKED,
        &mut tx_pll_locked,
        rf_chain,
    );
    if rx_pll_locked as libc::c_int == 0i32 || tx_pll_locked as libc::c_int == 0i32 {
        return -1i32;
    }
    /* Trig calibration */
    /* Select radio to be connected to the Signal Analyzer (warning: RadioA:1, RadioB:0) */
    lgw_reg_w(
        820i32 as uint16_t,
        if rf_chain as libc::c_int == 0i32 {
            1i32
        } else {
            0i32
        },
    );
    reg = if rf_chain as libc::c_int == 0i32 {
        84i32
    } else {
        192i32
    } as uint16_t;
    lgw_reg_w(reg, 0i32);
    reg = if rf_chain as libc::c_int == 0i32 {
        61i32
    } else {
        169i32
    } as uint16_t;
    lgw_reg_w(reg, 1i32);
    lgw_reg_w(reg, 0i32);
    reg = if rf_chain as libc::c_int == 0i32 {
        785i32
    } else {
        800i32
    } as uint16_t;
    lgw_reg_w(reg, 1i32);
    /* For debug */
    /* Set calibration parameters */
    sx1302_agc_mailbox_write(2i32 as uint8_t, (rf_chain as libc::c_int + 2i32) as uint8_t); /* Set TX test config: radioA:2 radioB:3 */
    sx1302_agc_mailbox_write(
        1i32 as uint8_t,
        (250000i32 as libc::c_double * 64e-6f64) as uint8_t,
    ); /* Set frequency */
    sx1302_agc_mailbox_write(0i32 as uint8_t, 0i32 as uint8_t); /* correlation duration: 0:1ms, 1:2ms, 2:4ms, 3:8ms) */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0i32 as uint8_t); /* sync */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x1i32 as uint8_t); /* sync */
    sx1302_agc_wait_status(0x1i32 as uint8_t); /* amp */
    sx1302_agc_mailbox_write(
        2i32 as uint8_t,
        rf_rx_image_amp[rf_chain as usize] as uint8_t,
    ); /* phi */
    sx1302_agc_mailbox_write(
        1i32 as uint8_t,
        rf_rx_image_phi[rf_chain as usize] as uint8_t,
    ); /* sync */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x2i32 as uint8_t); /* i offset init */
    sx1302_agc_wait_status(0x2i32 as uint8_t); /* q offset init */
    sx1302_agc_mailbox_write(2i32 as uint8_t, 0i32 as uint8_t); /* sync */
    sx1302_agc_mailbox_write(1i32 as uint8_t, 0i32 as uint8_t); /* sync */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x3i32 as uint8_t);
    sx1302_agc_wait_status(0x3i32 as uint8_t);
    sx1302_agc_mailbox_write(2i32 as uint8_t, 0i32 as uint8_t);
    sx1302_agc_mailbox_write(1i32 as uint8_t, tx_threshold);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x4i32 as uint8_t);
    /* Get calibration results */
    sx1302_agc_wait_status(0x6i32 as uint8_t); /* sync */
    let mut threshold: uint8_t = 0; /* sync */
    let mut cal_dec_gain: uint8_t = 0;
    let mut tx_sig_0: uint8_t = 0;
    let mut tx_sig_1: uint8_t = 0;
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut threshold);
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut cal_dec_gain);
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut tx_sig_1);
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut tx_sig_0);
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x6i32 as uint8_t);
    sx1302_agc_wait_status(0x7i32 as uint8_t);
    let mut tx_dc_0: uint8_t = 0;
    let mut tx_dc_1: uint8_t = 0;
    let mut offset_i: uint8_t = 0;
    let mut offset_q: uint8_t = 0;
    let mut tx_sig: libc::c_float = 0.;
    let mut tx_dc: libc::c_float = 0.;
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut tx_dc_1);
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut tx_dc_0);
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut offset_i);
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut offset_q);
    tx_sig = tx_sig_1 as libc::c_float * 256i32 as libc::c_float + tx_sig_0 as libc::c_float;
    tx_dc = tx_dc_1 as libc::c_float * 256i32 as libc::c_float + tx_dc_0 as libc::c_float;
    (*res).rej = (20i32 as libc::c_double * log10((tx_sig / tx_dc) as libc::c_double)) as uint16_t;
    (*res).offset_i = offset_i as int8_t;
    (*res).offset_q = offset_q as int8_t;
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x7i32 as uint8_t);
    /* -----------------------------------------------*/
    /* DEBUG: Get IQ offsets selected for iterations */
    let mut index: [uint8_t; 12] = [0; 12]; /* sync */
    sx1302_agc_wait_status(0x8i32 as uint8_t); /* sync */
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut *index.as_mut_ptr().offset(0)); /* sync */
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut *index.as_mut_ptr().offset(1));
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut *index.as_mut_ptr().offset(2));
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut *index.as_mut_ptr().offset(3));
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x8i32 as uint8_t);
    sx1302_agc_wait_status(0x9i32 as uint8_t);
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut *index.as_mut_ptr().offset(4));
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut *index.as_mut_ptr().offset(5));
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut *index.as_mut_ptr().offset(6));
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut *index.as_mut_ptr().offset(7));
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x9i32 as uint8_t);
    sx1302_agc_wait_status(0xai32 as uint8_t);
    sx1302_agc_mailbox_read(3i32 as uint8_t, &mut *index.as_mut_ptr().offset(8));
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut *index.as_mut_ptr().offset(9));
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut *index.as_mut_ptr().offset(10));
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut *index.as_mut_ptr().offset(11));
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0xai32 as uint8_t);
    /* DEBUG_CAL */
    /* -----------------------------------------------*/
    /* -----------------------------------------------*/
    /* DEBUG: Get TX_SIG returned by siognal analyzer */
    let mut msb: [uint8_t; 40] = [0; 40];
    let mut lsb: [uint8_t; 40] = [0; 40];
    i = 0i32;
    while i < 20i32 {
        sx1302_agc_wait_status((0xci32 + i) as uint8_t);
        sx1302_agc_mailbox_read(
            3i32 as uint8_t,
            &mut *msb.as_mut_ptr().offset((2i32 * i) as isize),
        );
        sx1302_agc_mailbox_read(
            2i32 as uint8_t,
            &mut *lsb.as_mut_ptr().offset((2i32 * i) as isize),
        );
        sx1302_agc_mailbox_read(
            1i32 as uint8_t,
            &mut *msb.as_mut_ptr().offset((2i32 * i + 1i32) as isize),
        );
        sx1302_agc_mailbox_read(
            0i32 as uint8_t,
            &mut *lsb.as_mut_ptr().offset((2i32 * i + 1i32) as isize),
        );
        sx1302_agc_mailbox_write(3i32 as uint8_t, (0xci32 + i) as uint8_t);
        i += 1
        /* sync */
    }
    sx1302_agc_wait_status((0xci32 + 20i32) as uint8_t);
    /* DEBUG_CAL */
    sx1302_agc_mailbox_write(3i32 as uint8_t, (0xci32 + 20i32) as uint8_t); /* sync */
    /* -----------------------------------------------*/
    printf(
        b"%s: RESULT: offset_i:%d offset_q:%d rej:%u\n\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sx125x_cal_tx_dc_offset\x00"))
            .as_ptr(),
        (*res).offset_i as libc::c_int,
        (*res).offset_q as libc::c_int,
        (*res).rej as libc::c_int,
    );
    /* Wait for calibration to be completed */
    sx1302_agc_wait_status(if rf_chain as libc::c_int == 0i32 {
        0x33i32
    } else {
        0x44i32
    } as uint8_t);
    /* TX_CALIB_DONE_BY_HAL */
    return 0i32;
}
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn cal_rx_result_init(
    mut res_rx_min: *mut lgw_sx125x_cal_rx_result_s,
    mut res_rx_max: *mut lgw_sx125x_cal_rx_result_s,
) {
    (*res_rx_min).amp = 31i32 as int8_t;
    (*res_rx_min).phi = 31i32 as int8_t;
    (*res_rx_min).rej = 255i32 as uint16_t;
    (*res_rx_min).rej_init = 255i32 as uint16_t;
    (*res_rx_min).snr = 255i32 as uint16_t;
    (*res_rx_max).amp = -32i32 as int8_t;
    (*res_rx_max).phi = -32i32 as int8_t;
    (*res_rx_max).rej = 0i32 as uint16_t;
    (*res_rx_max).rej_init = 0i32 as uint16_t;
    (*res_rx_max).snr = 0i32 as uint16_t;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn cal_rx_result_sort(
    mut res_rx: *mut lgw_sx125x_cal_rx_result_s,
    mut res_rx_min: *mut lgw_sx125x_cal_rx_result_s,
    mut res_rx_max: *mut lgw_sx125x_cal_rx_result_s,
) {
    if ((*res_rx).amp as libc::c_int) < (*res_rx_min).amp as libc::c_int {
        (*res_rx_min).amp = (*res_rx).amp
    }
    if ((*res_rx).phi as libc::c_int) < (*res_rx_min).phi as libc::c_int {
        (*res_rx_min).phi = (*res_rx).phi
    }
    if ((*res_rx).rej as libc::c_int) < (*res_rx_min).rej as libc::c_int {
        (*res_rx_min).rej = (*res_rx).rej
    }
    if ((*res_rx).rej_init as libc::c_int) < (*res_rx_min).rej_init as libc::c_int {
        (*res_rx_min).rej_init = (*res_rx).rej_init
    }
    if ((*res_rx).snr as libc::c_int) < (*res_rx_min).snr as libc::c_int {
        (*res_rx_min).snr = (*res_rx).snr
    }
    if (*res_rx).amp as libc::c_int > (*res_rx_max).amp as libc::c_int {
        (*res_rx_max).amp = (*res_rx).amp
    }
    if (*res_rx).phi as libc::c_int > (*res_rx_max).phi as libc::c_int {
        (*res_rx_max).phi = (*res_rx).phi
    }
    if (*res_rx).rej as libc::c_int > (*res_rx_max).rej as libc::c_int {
        (*res_rx_max).rej = (*res_rx).rej
    }
    if (*res_rx).rej_init as libc::c_int > (*res_rx_max).rej_init as libc::c_int {
        (*res_rx_max).rej_init = (*res_rx).rej_init
    }
    if (*res_rx).snr as libc::c_int > (*res_rx_max).snr as libc::c_int {
        (*res_rx_max).snr = (*res_rx).snr
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn cal_rx_result_assert(
    mut res_rx_min: *mut lgw_sx125x_cal_rx_result_s,
    mut res_rx_max: *mut lgw_sx125x_cal_rx_result_s,
) -> bool {
    if (*res_rx_max).amp as libc::c_int - (*res_rx_min).amp as libc::c_int > 4i32
        || (*res_rx_max).phi as libc::c_int - (*res_rx_min).phi as libc::c_int > 4i32
        || ((*res_rx_min).rej as libc::c_int) < 50i32
        || ((*res_rx_min).snr as libc::c_int) < 50i32
    {
        return 0i32 != 0;
    } else {
        return 1i32 != 0;
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn cal_tx_result_init(
    mut res_tx_min: *mut lgw_sx125x_cal_tx_result_s,
    mut res_tx_max: *mut lgw_sx125x_cal_tx_result_s,
) {
    (*res_tx_min).offset_i = 127i32 as int8_t;
    (*res_tx_min).offset_q = 127i32 as int8_t;
    (*res_tx_min).rej = 255i32 as uint16_t;
    (*res_tx_min).sig = 255i32 as uint16_t;
    (*res_tx_max).offset_i = -128i32 as int8_t;
    (*res_tx_max).offset_q = -128i32 as int8_t;
    (*res_tx_max).rej = 0i32 as uint16_t;
    (*res_tx_max).sig = 0i32 as uint16_t;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn cal_tx_result_sort(
    mut res_tx: *mut lgw_sx125x_cal_tx_result_s,
    mut res_tx_min: *mut lgw_sx125x_cal_tx_result_s,
    mut res_tx_max: *mut lgw_sx125x_cal_tx_result_s,
) {
    if ((*res_tx).offset_i as libc::c_int) < (*res_tx_min).offset_i as libc::c_int {
        (*res_tx_min).offset_i = (*res_tx).offset_i
    }
    if ((*res_tx).offset_q as libc::c_int) < (*res_tx_min).offset_q as libc::c_int {
        (*res_tx_min).offset_q = (*res_tx).offset_q
    }
    if ((*res_tx).rej as libc::c_int) < (*res_tx_min).rej as libc::c_int {
        (*res_tx_min).rej = (*res_tx).rej
    }
    if ((*res_tx).sig as libc::c_int) < (*res_tx_min).sig as libc::c_int {
        (*res_tx_min).sig = (*res_tx).sig
    }
    if (*res_tx).offset_i as libc::c_int > (*res_tx_max).offset_i as libc::c_int {
        (*res_tx_max).offset_i = (*res_tx).offset_i
    }
    if (*res_tx).offset_q as libc::c_int > (*res_tx_max).offset_q as libc::c_int {
        (*res_tx_max).offset_q = (*res_tx).offset_q
    }
    if (*res_tx).rej as libc::c_int > (*res_tx_max).rej as libc::c_int {
        (*res_tx_max).rej = (*res_tx).rej
    }
    if (*res_tx).sig as libc::c_int > (*res_tx_max).sig as libc::c_int {
        (*res_tx_max).sig = (*res_tx).sig
    };
}
#[no_mangle]
pub unsafe extern "C" fn cal_tx_result_assert(
    mut res_tx_min: *mut lgw_sx125x_cal_tx_result_s,
    mut res_tx_max: *mut lgw_sx125x_cal_tx_result_s,
) -> bool {
    if (*res_tx_max).offset_i as libc::c_int - (*res_tx_min).offset_i as libc::c_int > 4i32
        || (*res_tx_max).offset_q as libc::c_int - (*res_tx_min).offset_q as libc::c_int > 4i32
        || ((*res_tx_min).rej as libc::c_int) < 10i32
    {
        return 0i32 != 0;
    } else {
        return 1i32 != 0;
    };
}
/* --- EOF ------------------------------------------------------------------ */
/* TX_CALIB_DONE_BY_HAL */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
