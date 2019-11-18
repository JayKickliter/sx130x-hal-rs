use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    @brief LoRa concentrator register burst read
    @param register_id register number in the data structure describing registers
    @param data pointer to byte array that will be written from the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_rb(register_id: uint16_t, data: *mut uint8_t, size: uint16_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timestamp_counter_s {
    pub counter_us_raw_27bits_inst_prev: uint32_t,
    pub counter_us_raw_27bits_pps_prev: uint32_t,
    pub counter_us_raw_27bits_inst_wrap: uint8_t,
    pub counter_us_raw_27bits_pps_wrap: uint8_t,
}
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    SX1302 timestamp counter Hardware Abstraction Layer
    Handles the conversion of a 32-bits 32MHz counter into a 32-bits 1 MHz counter.
    This modules MUST be called regularly by the application to maintain counter
    wrapping handling for conversion in 1MHz counter.
    Provides function to compute the correction to be applied to the received
    timestamp for demodulation processing time.

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types*/
/* boolean type */
/* library configuration options (dynamically generated) */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC MACROS -------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC TYPES --------------------------------------------------------- */
/* *
@struct timestamp_counter_s
@brief context to maintain the internal counters (inst and pps trig) wrapping
*/
pub type timestamp_counter_t = timestamp_counter_s;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    SX1302 timestamp counter Hardware Abstraction Layer
    Handles the conversion of a 32-bits 32MHz counter into a 32-bits 1 MHz counter.
    This modules MUST be called regularly by the application to maintain counter
    wrapping handling for conversion in 1MHz counter.
    Provides function to compute the correction to be applied to the received
    timestamp for demodulation processing time.

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* printf fprintf */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE TYPES -------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DECLARATION ---------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_new(mut self_0: *mut timestamp_counter_t) {
    (*self_0).counter_us_raw_27bits_inst_prev = 0i32 as uint32_t;
    (*self_0).counter_us_raw_27bits_pps_prev = 0i32 as uint32_t;
    (*self_0).counter_us_raw_27bits_inst_wrap = 0i32 as uint8_t;
    (*self_0).counter_us_raw_27bits_pps_wrap = 0i32 as uint8_t;
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS ----------------------------------------------------- */
/* *
@brief Initialize the timestamp_counter instance
@param self     Pointer to the counter handler
@return N/A
*/
/* *
@brief Reset the timestamp_counter instance
@param self     Pointer to the counter handler
@return N/A
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_delete(mut self_0: *mut timestamp_counter_t) {
    (*self_0).counter_us_raw_27bits_inst_prev = 0i32 as uint32_t;
    (*self_0).counter_us_raw_27bits_pps_prev = 0i32 as uint32_t;
    (*self_0).counter_us_raw_27bits_inst_wrap = 0i32 as uint8_t;
    (*self_0).counter_us_raw_27bits_pps_wrap = 0i32 as uint8_t;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_update(
    mut self_0: *mut timestamp_counter_t,
    mut pps: bool,
    mut cnt: uint32_t,
) {
    let mut counter_us_raw_27bits_prev: uint32_t = 0;
    let mut counter_us_raw_27bits_wrap: uint8_t = 0;
    /* Get the previous counter value and wrap status */
    if pps as libc::c_int == 1i32 {
        counter_us_raw_27bits_prev = (*self_0).counter_us_raw_27bits_pps_prev;
        counter_us_raw_27bits_wrap = (*self_0).counter_us_raw_27bits_pps_wrap
    } else {
        counter_us_raw_27bits_prev = (*self_0).counter_us_raw_27bits_inst_prev;
        counter_us_raw_27bits_wrap = (*self_0).counter_us_raw_27bits_inst_wrap
    }
    /* Check if counter has wrapped, and update wrap status if necessary */
    if cnt < counter_us_raw_27bits_prev {
        counter_us_raw_27bits_wrap = (counter_us_raw_27bits_wrap as libc::c_int + 1i32) as uint8_t;
        counter_us_raw_27bits_wrap = (counter_us_raw_27bits_wrap as libc::c_int % 32i32) as uint8_t
    }
    /* Store counter value and wrap status for next time */
    if pps as libc::c_int == 1i32 {
        (*self_0).counter_us_raw_27bits_pps_prev = cnt;
        (*self_0).counter_us_raw_27bits_pps_wrap = counter_us_raw_27bits_wrap
    } else {
        (*self_0).counter_us_raw_27bits_inst_prev = cnt;
        (*self_0).counter_us_raw_27bits_inst_wrap = counter_us_raw_27bits_wrap
    };
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_get(
    mut self_0: *mut timestamp_counter_t,
    mut pps: bool,
) -> uint32_t {
    let mut x: libc::c_int = 0;
    let mut buff: [uint8_t; 4] = [0; 4];
    let mut counter_us_raw_27bits_now: uint32_t = 0;
    /* Get the 32MHz timestamp counter - 4 bytes */
    /* step of 31.25 ns */
    x = lgw_reg_rb(
        if pps as libc::c_int == 1i32 {
            307i32
        } else {
            311i32
        } as uint16_t,
        &mut *buff.as_mut_ptr().offset(0),
        4i32 as uint16_t,
    ); /* scale to 1MHz */
    if x != 0i32 {
        printf(
            b"ERROR: Failed to get timestamp counter value\n\x00" as *const u8
                as *const libc::c_char,
        );
        return 0i32 as uint32_t;
    }
    counter_us_raw_27bits_now = ((buff[0] as libc::c_int) << 24i32) as libc::c_uint & 0xff000000u32;
    counter_us_raw_27bits_now |= ((buff[1] as libc::c_int) << 16i32 & 0xff0000i32) as uint32_t;
    counter_us_raw_27bits_now |= ((buff[2] as libc::c_int) << 8i32 & 0xff00i32) as uint32_t;
    counter_us_raw_27bits_now |= ((buff[3] as libc::c_int) << 0i32 & 0xffi32) as uint32_t;
    counter_us_raw_27bits_now = (counter_us_raw_27bits_now as libc::c_uint)
        .wrapping_div(32i32 as libc::c_uint) as uint32_t
        as uint32_t;
    /* Update counter wrapping status */
    timestamp_counter_update(self_0, pps, counter_us_raw_27bits_now);
    /* Convert 27-bits counter to 32-bits counter */
    return timestamp_counter_expand(self_0, pps, counter_us_raw_27bits_now);
}
/* *
@brief Update the counter wrapping status based on given current counter
@param self     Pointer to the counter handler
@param pps      Set to true to update the PPS trig counter status
@param cnt      Current value of the counter to be used for the update
@return N/A
*/
/* *
@brief Convert the 27-bits counter given by the SX1302 to a 32-bits counter which wraps on a uint32_t.
@param self     Pointer to the counter handler
@param pps      Set to true to expand the counter based on the PPS trig wrapping status
@param cnt_us   The 27-bits counter to be expanded
@return the 32-bits counter
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_expand(
    mut self_0: *mut timestamp_counter_t,
    mut pps: bool,
    mut cnt_us: uint32_t,
) -> uint32_t {
    let mut counter_us_32bits: uint32_t = 0;
    if pps as libc::c_int == 1i32 {
        counter_us_32bits = (((*self_0).counter_us_raw_27bits_pps_wrap as libc::c_int) << 27i32)
            as libc::c_uint
            | cnt_us
    } else {
        counter_us_32bits = (((*self_0).counter_us_raw_27bits_inst_wrap as libc::c_int) << 27i32)
            as libc::c_uint
            | cnt_us
    }
    return counter_us_32bits;
}
/* *
@brief Configure the SX1302 to output legacy timestamp or precision timestamp
@note  Legacy timestamp gives a timestamp latched at the end of the packet
@note  Precision timestamp gives a timestamp latched at the end of the header
@note  and additionally supplies metrics every N symbols troughout the payload.
@param enable_precision_ts  A boolean to enable precision timestamp output.
@param max_ts_metrics       The number of timestamp metrics to be returned when precision timestamp is enabled
@param nb_symbols           The sampling rate of timestamp metrics
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_mode(
    mut enable_precision_ts: bool,
    mut max_ts_metrics: uint8_t,
    mut nb_symbols: uint8_t,
) -> libc::c_int {
    if enable_precision_ts as libc::c_int == 0i32 {
        /* Latch end-of-packet timestamp (sx1301 compatibility) */
        lgw_reg_w(710i32 as uint16_t, 0x1i32);
    } else {
        /* Latch end-of-preamble timestamp */
        lgw_reg_w(710i32 as uint16_t, 0i32);
        lgw_reg_w(713i32 as uint16_t, max_ts_metrics as int32_t);
        /* LoRa multi-SF modems */
        lgw_reg_w(684i32 as uint16_t, 0x1i32);
        lgw_reg_w(685i32 as uint16_t, nb_symbols as int32_t);
        /* LoRa service modem */
        lgw_reg_w(991i32 as uint16_t, 0x1i32);
        lgw_reg_w(992i32 as uint16_t, nb_symbols as int32_t);
    }
    return 0i32;
}
/* *
@brief Reads the SX1302 internal counter register, and return the 32-bits 1 MHz counter
@param self     Pointer to the counter handler
@param pps      Set to true to expand the counter based on the PPS trig wrapping status
@return the current 32-bits counter
*/
/* *
@brief Get the timestamp correction to applied to the packet timestamp
@param ifmod            modem type
@param bandwidth        modulation bandwidth
@param datarate         modulation datarate
@param coderate         modulation coding rate
@param crc_en           indicates if CRC is enabled or disabled
@param payload_length   payload length
@return The correction to be applied to the packet timestamp, in microseconds
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn timestamp_counter_correction(
    mut ifmod: libc::c_int,
    mut bandwidth: uint8_t,
    mut datarate: uint8_t,
    mut coderate: uint8_t,
    mut crc_en: uint32_t,
    mut payload_length: uint16_t,
) -> uint32_t {
    let mut val: int32_t = 0;
    let mut sf: uint32_t = datarate as uint32_t;
    let mut cr: uint32_t = coderate as uint32_t;
    let mut bw_pow: uint32_t = 0;
    let mut clk_period: uint32_t = 0;
    let mut nb_nibble: uint32_t = 0;
    let mut nb_nibble_in_hdr: uint32_t = 0;
    let mut nb_nibble_in_last_block: uint32_t = 0;
    let mut dft_peak_en: uint32_t = 0;
    let mut nb_iter: uint32_t = 0;
    let mut demap_delay: uint32_t = 0;
    let mut decode_delay: uint32_t = 0;
    let mut fft_delay_state3: uint32_t = 0;
    let mut fft_delay: uint32_t = 0;
    let mut delay_x: uint32_t = 0;
    let mut timestamp_correction: uint32_t = 0;
    let mut ppm: uint32_t = if bandwidth as libc::c_int == 0x4i32
        && (datarate as libc::c_int == 11i32 || datarate as libc::c_int == 12i32)
        || bandwidth as libc::c_int == 0x5i32 && datarate as libc::c_int == 12i32
    {
        1i32
    } else {
        0i32
    } as uint32_t;
    match bandwidth as libc::c_int {
        4 => bw_pow = 1i32 as uint32_t,
        5 => bw_pow = 2i32 as uint32_t,
        6 => bw_pow = 4i32 as uint32_t,
        _ => return 0i32 as uint32_t,
    }
    clk_period = (250000i32 as libc::c_uint).wrapping_div(bw_pow);
    delay_x = (16000000i32 as libc::c_uint)
        .wrapping_div(bw_pow)
        .wrapping_add(2031250i32 as libc::c_uint);
    nb_nibble = (payload_length as libc::c_uint)
        .wrapping_add((2i32 as libc::c_uint).wrapping_mul(crc_en))
        .wrapping_mul(2i32 as libc::c_uint)
        .wrapping_add(5i32 as libc::c_uint);
    if sf == 5i32 as libc::c_uint || sf == 6i32 as libc::c_uint {
        nb_nibble_in_hdr = sf
    } else {
        nb_nibble_in_hdr = sf.wrapping_sub(2i32 as libc::c_uint)
    }
    nb_nibble_in_last_block = nb_nibble.wrapping_sub(nb_nibble_in_hdr).wrapping_sub(
        sf.wrapping_sub((2i32 as libc::c_uint).wrapping_mul(ppm))
            .wrapping_mul(
                nb_nibble
                    .wrapping_sub(nb_nibble_in_hdr)
                    .wrapping_div(sf.wrapping_sub((2i32 as libc::c_uint).wrapping_mul(ppm))),
            ),
    );
    if nb_nibble_in_last_block == 0i32 as libc::c_uint {
        nb_nibble_in_last_block = sf.wrapping_sub((2i32 as libc::c_uint).wrapping_mul(ppm))
    }
    nb_iter = sf.wrapping_add(1i32 as libc::c_uint) >> 1i32;
    /* timestamp correction code, variable delay */
    if ifmod == 0x10i32 {
        lgw_reg_r(922i32 as uint16_t, &mut val);
    } else {
        lgw_reg_r(567i32 as uint16_t, &mut val);
    }
    if val != 0i32 {
        /* TODO: should we differentiate the mode (FULL/TRACK) ? */
        dft_peak_en = 1i32 as uint32_t
    } else {
        dft_peak_en = 0i32 as uint32_t
    }
    if sf >= 5i32 as libc::c_uint && sf <= 12i32 as libc::c_uint && bw_pow > 0i32 as libc::c_uint {
        if (2i32 as libc::c_uint)
            .wrapping_mul(
                (payload_length as libc::c_uint)
                    .wrapping_add((2i32 as libc::c_uint).wrapping_mul(crc_en)),
            )
            .wrapping_sub(sf.wrapping_sub(7i32 as libc::c_uint))
            <= 0i32 as libc::c_uint
        {
            /* payload fits entirely in first 8 symbols (header) */
            if sf > 6i32 as libc::c_uint {
                nb_nibble_in_last_block = sf.wrapping_sub(2i32 as libc::c_uint)
            } else {
                nb_nibble_in_last_block = sf
                // can't be acheived
            } /* header coding rate is 4 */
            dft_peak_en = 0i32 as uint32_t;
            cr = 4i32 as uint32_t;
            demap_delay = clk_period
                .wrapping_add(
                    ((1i32 << sf) as libc::c_uint)
                        .wrapping_mul(clk_period)
                        .wrapping_mul(3i32 as libc::c_uint)
                        .wrapping_div(4i32 as libc::c_uint),
                )
                .wrapping_add((3i32 as libc::c_uint).wrapping_mul(clk_period))
                .wrapping_add(
                    sf.wrapping_sub(2i32 as libc::c_uint)
                        .wrapping_mul(clk_period),
                )
        } else {
            demap_delay = clk_period
                .wrapping_add(
                    ((1i32 << sf) as libc::c_uint)
                        .wrapping_mul(clk_period)
                        .wrapping_mul(
                            (1i32 as libc::c_uint)
                                .wrapping_sub(ppm.wrapping_div(4i32 as libc::c_uint)),
                        ),
                )
                .wrapping_add((3i32 as libc::c_uint).wrapping_mul(clk_period))
                .wrapping_add(
                    sf.wrapping_sub((2i32 as libc::c_uint).wrapping_mul(ppm))
                        .wrapping_mul(clk_period),
                )
        }
        fft_delay_state3 = clk_period
            .wrapping_mul(
                (((1i32 << sf) - 6i32) as libc::c_uint).wrapping_add(
                    (2i32 as libc::c_uint).wrapping_mul(
                        ((1i32 << sf) as libc::c_uint)
                            .wrapping_mul(nb_iter.wrapping_sub(1i32 as libc::c_uint))
                            .wrapping_add(6i32 as libc::c_uint),
                    ),
                ),
            )
            .wrapping_add((4i32 as libc::c_uint).wrapping_mul(clk_period));
        if dft_peak_en != 0 {
            fft_delay = (5i32 as libc::c_uint)
                .wrapping_sub((2i32 as libc::c_uint).wrapping_mul(ppm))
                .wrapping_mul(
                    ((1i32 << sf) as libc::c_uint)
                        .wrapping_mul(clk_period)
                        .wrapping_add((7i32 as libc::c_uint).wrapping_mul(clk_period)),
                )
                .wrapping_add((2i32 as libc::c_uint).wrapping_mul(clk_period))
        } else {
            fft_delay = (((1i32 << sf) * 2i32) as libc::c_uint)
                .wrapping_mul(clk_period)
                .wrapping_add((3i32 as libc::c_uint).wrapping_mul(clk_period))
        }
        decode_delay = (5i32 as libc::c_uint)
            .wrapping_mul(clk_period)
            .wrapping_add(
                (9i32 as libc::c_uint)
                    .wrapping_mul(clk_period)
                    .wrapping_add(clk_period.wrapping_mul(cr))
                    .wrapping_mul(nb_nibble_in_last_block),
            )
            .wrapping_add((3i32 as libc::c_uint).wrapping_mul(clk_period));
        timestamp_correction = ((delay_x
            .wrapping_add(fft_delay_state3)
            .wrapping_add(fft_delay)
            .wrapping_add(demap_delay)
            .wrapping_add(decode_delay) as libc::c_double
            + 0.5e6f64) as uint32_t as libc::c_double
            / 1e6f64) as uint32_t
    //printf("INFO: timestamp_correction = %u us (delay_x %u, fft_delay_state3=%u, fft_delay=%u, demap_delay=%u, decode_delay = %u)\n", timestamp_correction, delay_x, fft_delay_state3, fft_delay, demap_delay, decode_delay);
    } else {
        timestamp_correction = 0i32 as uint32_t
    }
    return timestamp_correction;
}
/* --- EOF ------------------------------------------------------------------ */
