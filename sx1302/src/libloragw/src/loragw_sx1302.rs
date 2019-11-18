use libc;
extern "C" {
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC MACROS -------------------------------------------------------- */
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief Connect LoRa concentrator by opening SPI link
    @param spidev_path path to the SPI device to be used to connect to the SX1302
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    @brief Disconnect LoRa concentrator by closing SPI link
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    @brief LoRa concentrator register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    @brief LoRa concentrator register read
    @param register_id register number in the data structure describing registers
    @param reg_value pointer to a variable where to write register read value
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    @brief LoRa concentrator register burst write
    @param register_id register number in the data structure describing registers
    @param data pointer to byte array that will be sent to the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    @brief LoRa concentrator register burst read
    @param register_id register number in the data structure describing registers
    @param data pointer to byte array that will be written from the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    /* *
    TODO
    */
    /* *
    TODO
    */
    #[no_mangle]
    fn lgw_mem_rb(
        mem_addr: uint16_t,
        data: *mut uint8_t,
        size: uint16_t,
        fifo_mode: bool,
    ) -> libc::c_int;
    #[no_mangle]
    fn lgw_mem_wb(mem_addr: uint16_t, data: *const uint8_t, size: uint16_t) -> libc::c_int;
    #[no_mangle]
    fn lgw_reg_r(register_id: uint16_t, reg_value: *mut int32_t) -> libc::c_int;
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
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS ----------------------------------------------------- */
    /* *
    @brief Initialize the timestamp_counter instance
    @param self     Pointer to the counter handler
    @return N/A
    */
    #[no_mangle]
    fn timestamp_counter_new(self_0: *mut timestamp_counter_t);
    /* *
    @brief Convert the 27-bits counter given by the SX1302 to a 32-bits counter which wraps on a uint32_t.
    @param self     Pointer to the counter handler
    @param pps      Set to true to expand the counter based on the PPS trig wrapping status
    @param cnt_us   The 27-bits counter to be expanded
    @return the 32-bits counter
    */
    #[no_mangle]
    fn timestamp_counter_expand(
        self_0: *mut timestamp_counter_t,
        pps: bool,
        cnt_us: uint32_t,
    ) -> uint32_t;
    /* *
    @brief Reads the SX1302 internal counter register, and return the 32-bits 1 MHz counter
    @param self     Pointer to the counter handler
    @param pps      Set to true to expand the counter based on the PPS trig wrapping status
    @return the current 32-bits counter
    */
    #[no_mangle]
    fn timestamp_counter_get(self_0: *mut timestamp_counter_t, pps: bool) -> uint32_t;
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
    #[no_mangle]
    fn timestamp_counter_correction(
        ifmod: libc::c_int,
        bandwidth: uint8_t,
        datarate: uint8_t,
        coderate: uint8_t,
        crc_en: uint32_t,
        payload_length: uint16_t,
    ) -> uint32_t;
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
    #[no_mangle]
    fn timestamp_counter_mode(
        enable_precision_ts: bool,
        max_ts_metrics: uint8_t,
        nb_symbols: uint8_t,
    ) -> libc::c_int;
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief Initialize the rx_buffer instance
    @param self     A pointer to a rx_buffer handler
    @return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
    */
    #[no_mangle]
    fn rx_buffer_new(self_0: *mut rx_buffer_t) -> libc::c_int;
    /* *
    @brief Fetch packets from the SX1302 internal RX buffer, and count packets available.
    @param self     A pointer to a rx_buffer handler
    @return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
    */
    #[no_mangle]
    fn rx_buffer_fetch(self_0: *mut rx_buffer_t) -> libc::c_int;
    /* *
    @brief Parse the rx_buffer and return the first packet available in the given structure.
    @param self     A pointer to a rx_buffer handler
    @param pkt      A pointer to the structure to receive the packet parsed
    @return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
    */
    #[no_mangle]
    fn rx_buffer_pop(self_0: *mut rx_buffer_t, pkt: *mut rx_packet_t) -> libc::c_int;
    #[no_mangle]
    fn sx1250_calibrate(rf_chain: uint8_t, freq_hz: uint32_t) -> libc::c_int;
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    #[no_mangle]
    fn sx1302_cal_start(
        version: uint8_t,
        rf_chain_cfg: *mut lgw_conf_rxrf_s,
        txgain_lut: *mut lgw_tx_gain_lut_s,
    ) -> libc::c_int;
    /*
     / _____)             _              | |
    ( (____  _____ ____ _| |_ _____  ____| |__
     \____ \| ___ |    (_   _) ___ |/ ___)  _ \
     _____) ) ____| | | || |_| ____( (___| | | |
    (______/|_____)_|_|_| \__)_____)\____)_| |_|
      (C)2019 Semtech

    Description:
        LoRa concentrator HAL debug functions

    License: Revised BSD License, see LICENSE.TXT file include in the project
    */
    /* -------------------------------------------------------------------------- */
    /* --- DEPENDANCIES --------------------------------------------------------- */
    /* library configuration options (dynamically generated) */
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC MACROS -------------------------------------------------------- */
    /* -------------------------------------------------------------------------- */
    /* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
    /* *
    @brief
    @param
    */
    #[no_mangle]
    fn dbg_log_buffer_to_file(file: *mut FILE, buffer: *mut uint8_t, size: uint16_t);
    /* *
    @brief
    @param
    */
    #[no_mangle]
    fn dbg_log_payload_diff_to_file(
        file: *mut FILE,
        buffer1: *mut uint8_t,
        buffer2: *mut uint8_t,
        size: uint16_t,
    );
    /* *
    @brief
    @param
    */
    #[no_mangle]
    fn dbg_check_payload(
        context: *mut lgw_conf_debug_s,
        file: *mut FILE,
        payload_received: *mut uint8_t,
        size: uint8_t,
        ref_payload_idx: uint8_t,
        sf: uint8_t,
    ) -> libc::c_int;
    /* *
    @brief TODO
    @param TODO
    @return TODO
    */
    #[no_mangle]
    fn lgw_bw_getval(x: libc::c_int) -> int32_t;
    /* -------------------------------------------------------------------------- */
    /* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
    /* Log file */
    #[no_mangle]
    static mut log_file: *mut FILE;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type lgw_radio_type_t = libc::c_uint;
pub const LGW_RADIO_TYPE_SX1250: lgw_radio_type_t = 5;
pub const LGW_RADIO_TYPE_SX1276: lgw_radio_type_t = 4;
pub const LGW_RADIO_TYPE_SX1272: lgw_radio_type_t = 3;
pub const LGW_RADIO_TYPE_SX1257: lgw_radio_type_t = 2;
pub const LGW_RADIO_TYPE_SX1255: lgw_radio_type_t = 1;
pub const LGW_RADIO_TYPE_NONE: lgw_radio_type_t = 0;
/* *
@struct lgw_conf_board_s
@brief Configuration structure for board specificities
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_board_s {
    pub lorawan_public: bool,
    pub clksrc: uint8_t,
    pub full_duplex: bool,
    pub spidev_path: [libc::c_char; 64],
    /* !> Path to access the SPI device to connect to the SX1302 */
}
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
@struct lgw_conf_rxif_s
@brief Configuration structure for an IF chain
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_rxif_s {
    pub enable: bool,
    pub rf_chain: uint8_t,
    pub freq_hz: int32_t,
    pub bandwidth: uint8_t,
    pub datarate: uint32_t,
    pub sync_word_size: uint8_t,
    pub sync_word: uint64_t,
    pub implicit_hdr: bool,
    pub implicit_payload_length: uint8_t,
    pub implicit_crc_en: bool,
    pub implicit_coderate: uint8_t,
    /* !> LoRa Service implicit header coding rate  */
}
/* *
@struct lgw_pkt_rx_s
@brief Structure containing the metadata of a packet that was received and a pointer to the payload
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_pkt_rx_s {
    pub freq_hz: uint32_t,
    pub freq_offset: int32_t,
    pub if_chain: uint8_t,
    pub status: uint8_t,
    pub count_us: uint32_t,
    pub rf_chain: uint8_t,
    pub modem_id: uint8_t,
    pub modulation: uint8_t,
    pub bandwidth: uint8_t,
    pub datarate: uint32_t,
    pub coderate: uint8_t,
    pub rssic: libc::c_float,
    pub rssis: libc::c_float,
    pub snr: libc::c_float,
    pub snr_min: libc::c_float,
    pub snr_max: libc::c_float,
    pub crc: uint16_t,
    pub size: uint16_t,
    pub payload: [uint8_t; 256],
    /* !> buffer containing the payload */
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
    pub freq_offset: int8_t,
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
/* *
@struct lgw_conf_debug_s
@brief Configuration structure for debug
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_ref_payload_s {
    pub id: uint32_t,
    pub payload: [uint8_t; 255],
    pub prev_cnt: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_debug_s {
    pub nb_ref_payload: uint8_t,
    pub ref_payload: [conf_ref_payload_s; 16],
    pub log_file_name: [libc::c_char; 128],
}
/* *
@struct lgw_conf_debug_s
@brief Configuration structure for debug
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_conf_timestamp_s {
    pub enable_precision_ts: bool,
    pub max_ts_metrics: uint8_t,
    pub nb_symbols: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lgw_context_s {
    pub is_started: bool,
    pub board_cfg: lgw_conf_board_s,
    pub rf_chain_cfg: [lgw_conf_rxrf_s; 2],
    pub if_chain_cfg: [lgw_conf_rxif_s; 10],
    pub lora_service_cfg: lgw_conf_rxif_s,
    pub fsk_cfg: lgw_conf_rxif_s,
    pub tx_gain_lut: [lgw_tx_gain_lut_s; 2],
    pub timestamp_cfg: lgw_conf_timestamp_s,
    pub debug_cfg: lgw_conf_debug_s,
}
/* *
@struct lgw_context_s
@brief Configuration context shared across modules
*/
pub type lgw_context_t = lgw_context_s;
/* Global context */
/* RX context */
/* LoRa service channel config parameters */
/* FSK channel config parameters */
/* TX context */
/* Misc */
/* Debug */
/* *
@struct rx_buffer_s
@brief buffer to hold the data fetched from the sx1302 RX buffer
*/
pub type rx_buffer_t = rx_buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rx_buffer_s {
    pub buffer: [uint8_t; 4096],
    pub buffer_size: uint16_t,
    pub buffer_index: libc::c_int,
    pub buffer_pkt_nb: uint8_t,
}
/* !> byte array to hald the data fetched from the RX buffer */
/* !> The number of bytes currently stored in the buffer */
/* !> Current parsing index in the buffer */
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
    SX1302 AGC parameters definition.

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- PRIVATE TYPES -------------------------------------------------------- */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agc_gain_params_s {
    pub ana_min: uint8_t,
    pub ana_max: uint8_t,
    pub ana_thresh_l: uint8_t,
    pub ana_thresh_h: uint8_t,
    pub dec_attn_min: uint8_t,
    pub dec_attn_max: uint8_t,
    pub dec_thresh_l: uint8_t,
    pub dec_thresh_h1: uint8_t,
    pub dec_thresh_h2: uint8_t,
    pub chan_attn_min: uint8_t,
    pub chan_attn_max: uint8_t,
    pub chan_thresh_l: uint8_t,
    pub chan_thresh_h: uint8_t,
    pub deviceSel: uint8_t,
    pub hpMax: uint8_t,
    pub paDutyCycle: uint8_t,
    /* sx1250 only */
}
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    SX1302 RX buffer Hardware Abstraction Layer

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
/* *
@struct rx_packet_s
@brief packet structure as contained in the sx1302 RX packet engine
*/
pub type rx_packet_t = rx_packet_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rx_packet_s {
    pub rxbytenb_modem: uint8_t,
    pub rx_channel_in: uint8_t,
    pub crc_en: bool,
    pub coding_rate: uint8_t,
    pub rx_rate_sf: uint8_t,
    pub modem_id: uint8_t,
    pub frequency_offset_error: int32_t,
    pub payload: [uint8_t; 255],
    pub payload_crc_error: bool,
    pub sync_error: bool,
    pub header_error: bool,
    pub timing_set: bool,
    pub snr_average: int8_t,
    pub rssi_chan_avg: uint8_t,
    pub rssi_signal_avg: uint8_t,
    pub rssi_chan_max_neg_delta: uint8_t,
    pub rssi_chan_max_pos_delta: uint8_t,
    pub rssi_sig_max_neg_delta: uint8_t,
    pub rssi_sig_max_pos_delta: uint8_t,
    pub timestamp_cnt: uint32_t,
    pub rx_crc16_value: uint16_t,
    pub num_ts_metrics_stored: uint8_t,
    pub timestamp_avg: [uint8_t; 255],
    pub timestamp_stddev: [uint8_t; 255],
    pub packet_checksum: uint8_t,
}
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* LoRa only */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS ---------------------------------------------------- */
#[no_mangle]
pub static mut agc_params_sx1250: agc_gain_params_s = {
    let mut init = agc_gain_params_s {
        ana_min: 1i32 as uint8_t,
        ana_max: 13i32 as uint8_t,
        ana_thresh_l: 3i32 as uint8_t,
        ana_thresh_h: 12i32 as uint8_t,
        dec_attn_min: 4i32 as uint8_t,
        dec_attn_max: 15i32 as uint8_t,
        dec_thresh_l: 40i32 as uint8_t,
        dec_thresh_h1: 80i32 as uint8_t,
        dec_thresh_h2: 90i32 as uint8_t,
        chan_attn_min: 4i32 as uint8_t,
        chan_attn_max: 14i32 as uint8_t,
        chan_thresh_l: 52i32 as uint8_t,
        chan_thresh_h: 132i32 as uint8_t,
        deviceSel: 0i32 as uint8_t,
        hpMax: 7i32 as uint8_t,
        paDutyCycle: 4i32 as uint8_t,
    };
    init
};
#[no_mangle]
pub static mut agc_params_sx125x: agc_gain_params_s = {
    let mut init = agc_gain_params_s {
        ana_min: 0i32 as uint8_t,
        ana_max: 9i32 as uint8_t,
        ana_thresh_l: 16i32 as uint8_t,
        ana_thresh_h: 35i32 as uint8_t,
        dec_attn_min: 7i32 as uint8_t,
        dec_attn_max: 11i32 as uint8_t,
        dec_thresh_l: 45i32 as uint8_t,
        dec_thresh_h1: 100i32 as uint8_t,
        dec_thresh_h2: 115i32 as uint8_t,
        chan_attn_min: 4i32 as uint8_t,
        chan_attn_max: 14i32 as uint8_t,
        chan_thresh_l: 52i32 as uint8_t,
        chan_thresh_h: 132i32 as uint8_t,
        deviceSel: 0i32 as uint8_t,
        hpMax: 0i32 as uint8_t,
        paDutyCycle: 0i32 as uint8_t,
    };
    init
};
/* configuration of available IF chains and modems on the hardware */
/* constant arrays defining hardware capability */
#[no_mangle]
pub static mut ifmod_config: [uint8_t; 10] = [
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x11i32 as uint8_t,
    0x10i32 as uint8_t,
    0x20i32 as uint8_t,
];
static mut cal_firmware_sx125x: [uint8_t; 8192] = [
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x6fi32 as uint8_t,
    0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4i32 as uint8_t,
    0x88i32 as uint8_t,
    0x84i32 as uint8_t,
    0xai32 as uint8_t,
    0x82i32 as uint8_t,
    0x47i32 as uint8_t,
    0i32 as uint8_t,
    0xf4i32 as uint8_t,
    0x40i32 as uint8_t,
    0x34i32 as uint8_t,
    0x2bi32 as uint8_t,
    0xf4i32 as uint8_t,
    0x1ci32 as uint8_t,
    0xb4i32 as uint8_t,
    0x13i32 as uint8_t,
    0xb4i32 as uint8_t,
    0xdi32 as uint8_t,
    0xb4i32 as uint8_t,
    0x8i32 as uint8_t,
    0x34i32 as uint8_t,
    0x6i32 as uint8_t,
    0x74i32 as uint8_t,
    0x4i32 as uint8_t,
    0x34i32 as uint8_t,
    0x2i32 as uint8_t,
    0x34i32 as uint8_t,
    0x10i32 as uint8_t,
    0x34i32 as uint8_t,
    0xbi32 as uint8_t,
    0xb4i32 as uint8_t,
    0x7i32 as uint8_t,
    0xb4i32 as uint8_t,
    0x5i32 as uint8_t,
    0x74i32 as uint8_t,
    0x3i32 as uint8_t,
    0x74i32 as uint8_t,
    0x2i32 as uint8_t,
    0x34i32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0x64i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x81i32 as uint8_t,
    0x84i32 as uint8_t,
    0xai32 as uint8_t,
    0x4i32 as uint8_t,
    0xc6i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0i32 as uint8_t,
    0xf4i32 as uint8_t,
    0x4i32 as uint8_t,
    0xc6i32 as uint8_t,
    0x30i32 as uint8_t,
    0x69i32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0x4i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2di32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x31i32 as uint8_t,
    0x58i32 as uint8_t,
    0xb5i32 as uint8_t,
    0x29i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x48i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xb4i32 as uint8_t,
    0x41i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xeai32 as uint8_t,
    0x2ei32 as uint8_t,
    0x48i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xcfi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xb3i32 as uint8_t,
    0x29i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x88i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x32i32 as uint8_t,
    0x70i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x32i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x33i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xb4i32 as uint8_t,
    0i32 as uint8_t,
    0x6i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0xei32 as uint8_t,
    0xaai32 as uint8_t,
    0x34i32 as uint8_t,
    0x8i32 as uint8_t,
    0x7i32 as uint8_t,
    0xfai32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x2ai32 as uint8_t,
    0x6i32 as uint8_t,
    0x30i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xb4i32 as uint8_t,
    0i32 as uint8_t,
    0x6i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0xf3i32 as uint8_t,
    0x69i32 as uint8_t,
    0x21i32 as uint8_t,
    0x6ai32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xb3i32 as uint8_t,
    0x40i32 as uint8_t,
    0x4i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2di32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x20i32 as uint8_t,
    0x38i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x18i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xb9i32 as uint8_t,
    0x81i32 as uint8_t,
    0xc2i32 as uint8_t,
    0x1i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xbai32 as uint8_t,
    0x40i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc3i32 as uint8_t,
    0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xbbi32 as uint8_t,
    0x80i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xbci32 as uint8_t,
    0x40i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc5i32 as uint8_t,
    0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0xbi32 as uint8_t,
    0x70i32 as uint8_t,
    0xd7i32 as uint8_t,
    0x80i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xb4i32 as uint8_t,
    0i32 as uint8_t,
    0x33i32 as uint8_t,
    0x98i32 as uint8_t,
    0x6i32 as uint8_t,
    0xabi32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x31i32 as uint8_t,
    0x4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x39i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x42i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xd6i32 as uint8_t,
    0x81i32 as uint8_t,
    0x69i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x31i32 as uint8_t,
    0x4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xcfi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xebi32 as uint8_t,
    0x6ai32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcai32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xb1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x4bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x40i32 as uint8_t,
    0x4di32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0x69i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x51i32 as uint8_t,
    0x8i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x40i32 as uint8_t,
    0x52i32 as uint8_t,
    0x8i32 as uint8_t,
    0xcci32 as uint8_t,
    0i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xd6i32 as uint8_t,
    0xcai32 as uint8_t,
    0x56i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x44i32 as uint8_t,
    0xabi32 as uint8_t,
    0x2fi32 as uint8_t,
    0x88i32 as uint8_t,
    0xd1i32 as uint8_t,
    0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0xd2i32 as uint8_t,
    0x41i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x40i32 as uint8_t,
    0x4di32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x7fi32 as uint8_t,
    0xebi32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0xd7i32 as uint8_t,
    0x3i32 as uint8_t,
    0x57i32 as uint8_t,
    0x82i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0xaai32 as uint8_t,
    0xeai32 as uint8_t,
    0x4bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xb7i32 as uint8_t,
    0x80i32 as uint8_t,
    0x4ci32 as uint8_t,
    0x8i32 as uint8_t,
    0xb8i32 as uint8_t,
    0i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x81i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x41i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xc1i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xb9i32 as uint8_t,
    0x40i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbai32 as uint8_t,
    0x40i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc3i32 as uint8_t,
    0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbbi32 as uint8_t,
    0x80i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xebi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xdci32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbci32 as uint8_t,
    0x40i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc5i32 as uint8_t,
    0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xdci32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xebi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0x39i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x42i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xcei32 as uint8_t,
    0x81i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x96i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x4fi32 as uint8_t,
    0x88i32 as uint8_t,
    0xe6i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x96i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x83i32 as uint8_t,
    0xd7i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0xd6i32 as uint8_t,
    0x81i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xd6i32 as uint8_t,
    0xcai32 as uint8_t,
    0x56i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0x5ai32 as uint8_t,
    0xeci32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0x39i32 as uint8_t,
    0x7ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xb1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x40i32 as uint8_t,
    0x4di32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0x44i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x51i32 as uint8_t,
    0x8i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x96i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x56i32 as uint8_t,
    0xc7i32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x51i32 as uint8_t,
    0x8i32 as uint8_t,
    0xe6i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x96i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x56i32 as uint8_t,
    0xc7i32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x52i32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0xd7i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x17i32 as uint8_t,
    0xeci32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x4ei32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x4ei32 as uint8_t,
    0x48i32 as uint8_t,
    0x39i32 as uint8_t,
    0x7ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x40i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x9i32 as uint8_t,
    0x30i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xai32 as uint8_t,
    0x57i32 as uint8_t,
    0x82i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x86i32 as uint8_t,
    0xebi32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xb9i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbai32 as uint8_t,
    0x40i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbbi32 as uint8_t,
    0x80i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xbci32 as uint8_t,
    0x40i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc5i32 as uint8_t,
    0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xbei32 as uint8_t,
    0x80i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc7i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xbfi32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x43i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc9i32 as uint8_t,
    0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xc1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x66i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xcai32 as uint8_t,
    0i32 as uint8_t,
    0xcei32 as uint8_t,
    0x81i32 as uint8_t,
    0x39i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x42i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xd6i32 as uint8_t,
    0x81i32 as uint8_t,
    0x9i32 as uint8_t,
    0x30i32 as uint8_t,
    0xd6i32 as uint8_t,
    0xcai32 as uint8_t,
    0x56i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0x1di32 as uint8_t,
    0x2di32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcai32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xb1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x30i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x4fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x40i32 as uint8_t,
    0x4di32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0xf3i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x51i32 as uint8_t,
    0x8i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xf3i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x4ei32 as uint8_t,
    0x48i32 as uint8_t,
    0x39i32 as uint8_t,
    0x7ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x40i32 as uint8_t,
    0xc4i32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x4fi32 as uint8_t,
    0x88i32 as uint8_t,
    0xb5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xb6i32 as uint8_t,
    0x40i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x88i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x34i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x37i32 as uint8_t,
    0x88i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x38i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x6i32 as uint8_t,
    0x30i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x6i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x66i32 as uint8_t,
    0x2di32 as uint8_t,
    0x35i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x36i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x7i32 as uint8_t,
    0xfai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x9fi32 as uint8_t,
    0xadi32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x60i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x61i32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x62i32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x8i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x8i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0xe0i32 as uint8_t,
    0xedi32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x64i32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x65i32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x66i32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x67i32 as uint8_t,
    0x88i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x9i32 as uint8_t,
    0x30i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x9i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x21i32 as uint8_t,
    0xaei32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x68i32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x69i32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x6ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x83i32 as uint8_t,
    0x96i32 as uint8_t,
    0x6bi32 as uint8_t,
    0x88i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xai32 as uint8_t,
    0x30i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0xai32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x62i32 as uint8_t,
    0xeei32 as uint8_t,
    0xd7i32 as uint8_t,
    0xc1i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0x57i32 as uint8_t,
    0xdi32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0x57i32 as uint8_t,
    0xdi32 as uint8_t,
    0xa0i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0xd7i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0x57i32 as uint8_t,
    0xdi32 as uint8_t,
    0xa1i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0x57i32 as uint8_t,
    0xdi32 as uint8_t,
    0xa1i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0xd7i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xci32 as uint8_t,
    0xfei32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0xb1i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xai32 as uint8_t,
    0x57i32 as uint8_t,
    0x82i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x73i32 as uint8_t,
    0x6ei32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xci32 as uint8_t,
    0xfei32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0xc6i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xabi32 as uint8_t,
    0x40i32 as uint8_t,
    0xabi32 as uint8_t,
    0x9fi32 as uint8_t,
    0x9i32 as uint8_t,
    0xefi32 as uint8_t,
    0xa1i32 as uint8_t,
    0x1fi32 as uint8_t,
    0x4i32 as uint8_t,
    0xafi32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0x41i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xa3i32 as uint8_t,
    0x83i32 as uint8_t,
    0x80i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xffi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa5i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc2i32 as uint8_t,
    0xa6i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x23i32 as uint8_t,
    0x4ai32 as uint8_t,
    0x25i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x40i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa9i32 as uint8_t,
    0x41i32 as uint8_t,
    0xa8i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xa9i32 as uint8_t,
    0x83i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xaai32 as uint8_t,
    0i32 as uint8_t,
    0x27i32 as uint8_t,
    0x48i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x2ai32 as uint8_t,
    0x2i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x2i32 as uint8_t,
    0xafi32 as uint8_t,
    0x26i32 as uint8_t,
    0x8i32 as uint8_t,
    0x28i32 as uint8_t,
    0xc2i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x80i32 as uint8_t,
    0x34i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x2bi32 as uint8_t,
    0xc7i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x7fi32 as uint8_t,
    0x3ei32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x4i32 as uint8_t,
    0xafi32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0x41i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xa3i32 as uint8_t,
    0x83i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa5i32 as uint8_t,
    0x41i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xa5i32 as uint8_t,
    0x83i32 as uint8_t,
    0x7fi32 as uint8_t,
    0x70i32 as uint8_t,
    0xa6i32 as uint8_t,
    0i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x26i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x40i32 as uint8_t,
    0x25i32 as uint8_t,
    0x49i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xa9i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x29i32 as uint8_t,
    0x2i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x2ai32 as uint8_t,
    0x2fi32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x27i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x7fi32 as uint8_t,
    0xb4i32 as uint8_t,
    0x4i32 as uint8_t,
    0xafi32 as uint8_t,
    0xa8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x20i32 as uint8_t,
    0x38i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x18i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa5i32 as uint8_t,
    0i32 as uint8_t,
    0xa5i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x3ci32 as uint8_t,
    0x6fi32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x30i32 as uint8_t,
    0x78i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x18i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa5i32 as uint8_t,
    0i32 as uint8_t,
    0xa5i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x4ei32 as uint8_t,
    0x6fi32 as uint8_t,
    0x39i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa6i32 as uint8_t,
    0i32 as uint8_t,
    0x55i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x26i32 as uint8_t,
    0x9ci32 as uint8_t,
    0x50i32 as uint8_t,
    0xefi32 as uint8_t,
    0x28i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x3ai32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x28i32 as uint8_t,
    0xai32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x3bi32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xb8i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x1i32 as uint8_t,
    0x34i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xb8i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0i32 as uint8_t,
    0xf4i32 as uint8_t,
    0x24i32 as uint8_t,
    0xai32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xai32 as uint8_t,
    0xb8i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x1i32 as uint8_t,
    0x34i32 as uint8_t,
    0i32 as uint8_t,
    0xf4i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x23i32 as uint8_t,
    0x58i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x87i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0x8di32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0x8ci32 as uint8_t,
    0xa3i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x98i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xci32 as uint8_t,
    0xfei32 as uint8_t,
    0xb1i32 as uint8_t,
    0i32 as uint8_t,
    0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xb2i32 as uint8_t,
    0i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x31i32 as uint8_t,
    0x46i32 as uint8_t,
    0x32i32 as uint8_t,
    0x4i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0x42i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x12i32 as uint8_t,
    0x74i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x23i32 as uint8_t,
    0x2i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xcfi32 as uint8_t,
    0x80i32 as uint8_t,
    0x52i32 as uint8_t,
    0x8i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcei32 as uint8_t,
    0x40i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x4ei32 as uint8_t,
    0x48i32 as uint8_t,
    0x42i32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd4i32 as uint8_t,
    0i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x39i32 as uint8_t,
    0x7ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x11i32 as uint8_t,
    0x74i32 as uint8_t,
    0xfdi32 as uint8_t,
    0xf9i32 as uint8_t,
    0x2i32 as uint8_t,
    0x38i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0x34i32 as uint8_t,
    0xfdi32 as uint8_t,
    0xf9i32 as uint8_t,
    0x2i32 as uint8_t,
    0x38i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0x34i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8di32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8di32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xb1i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xb1i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x30i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2fi32 as uint8_t,
    0xe1i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x30i32 as uint8_t,
    0x83i32 as uint8_t,
    0xd7i32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x70i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2fi32 as uint8_t,
    0xe1i32 as uint8_t,
    0x83i32 as uint8_t,
    0x1i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x2ai32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0xffi32 as uint8_t,
    0xbfi32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x95i32 as uint8_t,
    0x41i32 as uint8_t,
    0x96i32 as uint8_t,
    0x41i32 as uint8_t,
    0x97i32 as uint8_t,
    0x81i32 as uint8_t,
    0x98i32 as uint8_t,
    0x1i32 as uint8_t,
    0x99i32 as uint8_t,
    0x41i32 as uint8_t,
    0x9ai32 as uint8_t,
    0x41i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x81i32 as uint8_t,
    0x9ci32 as uint8_t,
    0x41i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x81i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x81i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x4ai32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x65i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0xedi32 as uint8_t,
    0x6ai32 as uint8_t,
    0x9bi32 as uint8_t,
    0x81i32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x65i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x4i32 as uint8_t,
    0x6bi32 as uint8_t,
    0x3di32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe2i32 as uint8_t,
    0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe3i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3fi32 as uint8_t,
    0x30i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe4i32 as uint8_t,
    0i32 as uint8_t,
    0x3i32 as uint8_t,
    0x30i32 as uint8_t,
    0xe4i32 as uint8_t,
    0x85i32 as uint8_t,
    0x64i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x65i32 as uint8_t,
    0x48i32 as uint8_t,
    0x2i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x50i32 as uint8_t,
    0xabi32 as uint8_t,
    0x3di32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xdfi32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x65i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x88i32 as uint8_t,
    0xabi32 as uint8_t,
    0x3di32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xdci32 as uint8_t,
    0x40i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xddi32 as uint8_t,
    0x80i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3i32 as uint8_t,
    0x30i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x65i32 as uint8_t,
    0x48i32 as uint8_t,
    0x4i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0xc0i32 as uint8_t,
    0xabi32 as uint8_t,
    0x9ei32 as uint8_t,
    0x81i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x81i32 as uint8_t,
    0x3di32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xdei32 as uint8_t,
    0x80i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xe1i32 as uint8_t,
    0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xe2i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x2i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x56i32 as uint8_t,
    0xa4i32 as uint8_t,
    0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x5ci32 as uint8_t,
    0xa4i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x11i32 as uint8_t,
    0x30i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x2ci32 as uint8_t,
    0x62i32 as uint8_t,
    0x8bi32 as uint8_t,
    0xci32 as uint8_t,
    0x6ci32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x56i32 as uint8_t,
    0xa4i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x5ci32 as uint8_t,
    0xa4i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x22i32 as uint8_t,
    0x30i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x2ci32 as uint8_t,
    0x62i32 as uint8_t,
    0x8i32 as uint8_t,
    0x2i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x18i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x4ci32 as uint8_t,
    0x64i32 as uint8_t,
    0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x39i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x33i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x2ci32 as uint8_t,
    0x62i32 as uint8_t,
    0x8i32 as uint8_t,
    0x3i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x24i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x4ci32 as uint8_t,
    0x64i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x39i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x44i32 as uint8_t,
    0x30i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x2ci32 as uint8_t,
    0x62i32 as uint8_t,
    0x8i32 as uint8_t,
    0x4i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x30i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xe4i32 as uint8_t,
    0i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x37i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x55i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x3bi32 as uint8_t,
    0x2ci32 as uint8_t,
    0x62i32 as uint8_t,
    0x8i32 as uint8_t,
    0x5i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x4i32 as uint8_t,
    0x6bi32 as uint8_t,
    0x63i32 as uint8_t,
    0x48i32 as uint8_t,
    0x3ei32 as uint8_t,
    0xe4i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x37i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x66i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x4i32 as uint8_t,
    0x6bi32 as uint8_t,
    0xa9i32 as uint8_t,
    0i32 as uint8_t,
    0x5fi32 as uint8_t,
    0xc8i32 as uint8_t,
    0xaai32 as uint8_t,
    0i32 as uint8_t,
    0x60i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xabi32 as uint8_t,
    0x40i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x48i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x5di32 as uint8_t,
    0x88i32 as uint8_t,
    0xadi32 as uint8_t,
    0x40i32 as uint8_t,
    0x5ei32 as uint8_t,
    0x88i32 as uint8_t,
    0xaei32 as uint8_t,
    0x40i32 as uint8_t,
    0x64i32 as uint8_t,
    0x8i32 as uint8_t,
    0xafi32 as uint8_t,
    0x80i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x5fi32 as uint8_t,
    0xc8i32 as uint8_t,
    0xadi32 as uint8_t,
    0x40i32 as uint8_t,
    0x60i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xaei32 as uint8_t,
    0x40i32 as uint8_t,
    0x61i32 as uint8_t,
    0x8i32 as uint8_t,
    0xafi32 as uint8_t,
    0x80i32 as uint8_t,
    0x64i32 as uint8_t,
    0x8i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa9i32 as uint8_t,
    0i32 as uint8_t,
    0x61i32 as uint8_t,
    0x8i32 as uint8_t,
    0xaai32 as uint8_t,
    0i32 as uint8_t,
    0x64i32 as uint8_t,
    0x8i32 as uint8_t,
    0xabi32 as uint8_t,
    0x40i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xb3i32 as uint8_t,
    0x40i32 as uint8_t,
    0x4i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x8i32 as uint8_t,
    0xd6i32 as uint8_t,
    0x40i32 as uint8_t,
    0xbi32 as uint8_t,
    0x70i32 as uint8_t,
    0xd7i32 as uint8_t,
    0xc1i32 as uint8_t,
    0xdbi32 as uint8_t,
    0x80i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xb4i32 as uint8_t,
    0i32 as uint8_t,
    0x33i32 as uint8_t,
    0x98i32 as uint8_t,
    0xe0i32 as uint8_t,
    0xaci32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xfbi32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x1fi32 as uint8_t,
    0x6di32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0xdbi32 as uint8_t,
    0x3i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x82i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x1fi32 as uint8_t,
    0x6di32 as uint8_t,
    0x88i32 as uint8_t,
    0x6ci32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xfbi32 as uint8_t,
    0xe7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x39i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x4i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0xcfi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x10i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xcai32 as uint8_t,
    0xeci32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xb5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xb6i32 as uint8_t,
    0x40i32 as uint8_t,
    0xf3i32 as uint8_t,
    0xa7i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xb9i32 as uint8_t,
    0x40i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xbai32 as uint8_t,
    0x40i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0xfei32 as uint8_t,
    0xfci32 as uint8_t,
    0xadi32 as uint8_t,
    0x40i32 as uint8_t,
    0x29i32 as uint8_t,
    0x49i32 as uint8_t,
    0xaei32 as uint8_t,
    0x40i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0x1i32 as uint8_t,
    0x7ci32 as uint8_t,
    0xafi32 as uint8_t,
    0x80i32 as uint8_t,
    0x29i32 as uint8_t,
    0x8i32 as uint8_t,
    0x2i32 as uint8_t,
    0x7ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2di32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x53i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xc1i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xai32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0x2di32 as uint8_t,
    0x7ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x53i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0x85i32 as uint8_t,
    0xedi32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x40i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xd4i32 as uint8_t,
    0i32 as uint8_t,
    0x4i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xai32 as uint8_t,
    0x5bi32 as uint8_t,
    0x82i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x64i32 as uint8_t,
    0xedi32 as uint8_t,
    0x53i32 as uint8_t,
    0x48i32 as uint8_t,
    0xb7i32 as uint8_t,
    0x80i32 as uint8_t,
    0x54i32 as uint8_t,
    0x8i32 as uint8_t,
    0xb8i32 as uint8_t,
    0i32 as uint8_t,
    0xf3i32 as uint8_t,
    0xa7i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xd8i32 as uint8_t,
    0x41i32 as uint8_t,
    0xd9i32 as uint8_t,
    0x81i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xc1i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x58i32 as uint8_t,
    0x87i32 as uint8_t,
    0xbei32 as uint8_t,
    0x80i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x59i32 as uint8_t,
    0xc7i32 as uint8_t,
    0xc7i32 as uint8_t,
    0x40i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x58i32 as uint8_t,
    0x87i32 as uint8_t,
    0xbfi32 as uint8_t,
    0xc0i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x59i32 as uint8_t,
    0x42i32 as uint8_t,
    0xc8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x58i32 as uint8_t,
    0x2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x59i32 as uint8_t,
    0xc7i32 as uint8_t,
    0xc9i32 as uint8_t,
    0i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x58i32 as uint8_t,
    0x2i32 as uint8_t,
    0xc1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x5bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xai32 as uint8_t,
    0xfei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2i32 as uint8_t,
    0xa0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x59i32 as uint8_t,
    0x42i32 as uint8_t,
    0xcai32 as uint8_t,
    0i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xdai32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xf7i32 as uint8_t,
    0xe7i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x46i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xcfi32 as uint8_t,
    0xc1i32 as uint8_t,
    0xdai32 as uint8_t,
    0x81i32 as uint8_t,
    0x5i32 as uint8_t,
    0x30i32 as uint8_t,
    0xdai32 as uint8_t,
    0xcai32 as uint8_t,
    0x5ai32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0x32i32 as uint8_t,
    0xeei32 as uint8_t,
    0x1fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xaei32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x67i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0xffi32 as uint8_t,
    0x2di32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xe7i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xffi32 as uint8_t,
    0x2di32 as uint8_t,
    0x4fi32 as uint8_t,
    0x88i32 as uint8_t,
    0x3di32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd8i32 as uint8_t,
    0i32 as uint8_t,
    0xedi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x6i32 as uint8_t,
    0x30i32 as uint8_t,
    0xdbi32 as uint8_t,
    0xai32 as uint8_t,
    0x5bi32 as uint8_t,
    0x82i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x5ci32 as uint8_t,
    0x9ai32 as uint8_t,
    0x2di32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xbei32 as uint8_t,
    0x80i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xc7i32 as uint8_t,
    0x40i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xbfi32 as uint8_t,
    0xc0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x80i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xc9i32 as uint8_t,
    0i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcai32 as uint8_t,
    0i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xc2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xcbi32 as uint8_t,
    0x40i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc3i32 as uint8_t,
    0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xffi32 as uint8_t,
    0x7ei32 as uint8_t,
    0xcci32 as uint8_t,
    0i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcdi32 as uint8_t,
    0x40i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xc5i32 as uint8_t,
    0i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0x1i32 as uint8_t,
    0xbei32 as uint8_t,
    0xcei32 as uint8_t,
    0x40i32 as uint8_t,
    0xe1i32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xdai32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xf7i32 as uint8_t,
    0xe7i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x46i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xcfi32 as uint8_t,
    0xc1i32 as uint8_t,
    0xdai32 as uint8_t,
    0x81i32 as uint8_t,
    0x9i32 as uint8_t,
    0x30i32 as uint8_t,
    0xdai32 as uint8_t,
    0xcai32 as uint8_t,
    0x5ai32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0xc3i32 as uint8_t,
    0x2ei32 as uint8_t,
    0x1fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0xcbi32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xaei32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x97i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xbei32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0xd3i32 as uint8_t,
    0x67i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2bi32 as uint8_t,
    0x48i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x2di32 as uint8_t,
    0x27i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x56i32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x51i32 as uint8_t,
    0x70i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x51i32 as uint8_t,
    0x79i32 as uint8_t,
    0x67i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x50i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x3i32 as uint8_t,
    0x59i32 as uint8_t,
    0x90i32 as uint8_t,
    0xaei32 as uint8_t,
    0x56i32 as uint8_t,
    0x48i32 as uint8_t,
    0xe7i32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x90i32 as uint8_t,
    0xaei32 as uint8_t,
    0x4fi32 as uint8_t,
    0x88i32 as uint8_t,
    0x3di32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd8i32 as uint8_t,
    0i32 as uint8_t,
    0xedi32 as uint8_t,
    0xa7i32 as uint8_t,
    0x8ai32 as uint8_t,
    0x95i32 as uint8_t,
    0x51i32 as uint8_t,
    0x8i32 as uint8_t,
    0xbbi32 as uint8_t,
    0x80i32 as uint8_t,
    0x52i32 as uint8_t,
    0x8i32 as uint8_t,
    0xbci32 as uint8_t,
    0x40i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x30i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x34i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x35i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x36i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x6i32 as uint8_t,
    0x30i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x6i32 as uint8_t,
    0xbai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0xci32 as uint8_t,
    0xefi32 as uint8_t,
    0x39i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ai32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x58i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x59i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x7i32 as uint8_t,
    0x70i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x7i32 as uint8_t,
    0xfai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x45i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x3bi32 as uint8_t,
    0x88i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x19i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x3ci32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ai32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x37i32 as uint8_t,
    0x88i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1bi32 as uint8_t,
    0xb0i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x38i32 as uint8_t,
    0x8i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x1ci32 as uint8_t,
    0x70i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x8i32 as uint8_t,
    0xf0i32 as uint8_t,
    0x9bi32 as uint8_t,
    0x40i32 as uint8_t,
    0x3ci32 as uint8_t,
    0xb0i32 as uint8_t,
    0x83i32 as uint8_t,
    0x52i32 as uint8_t,
    0x3i32 as uint8_t,
    0x53i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x97i32 as uint8_t,
    0xd4i32 as uint8_t,
    0x97i32 as uint8_t,
    0x90i32 as uint8_t,
    0xdi32 as uint8_t,
    0x8i32 as uint8_t,
    0xd5i32 as uint8_t,
    0x40i32 as uint8_t,
    0x9ei32 as uint8_t,
    0x40i32 as uint8_t,
    0x55i32 as uint8_t,
    0x48i32 as uint8_t,
    0x8i32 as uint8_t,
    0x7ai32 as uint8_t,
    0x3i32 as uint8_t,
    0x9di32 as uint8_t,
    0x7ei32 as uint8_t,
    0xefi32 as uint8_t,
    0x10i32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x1i32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x96i32 as uint8_t,
    0i32 as uint8_t,
    0x23i32 as uint8_t,
    0x8i32 as uint8_t,
    0x95i32 as uint8_t,
    0i32 as uint8_t,
    0x17i32 as uint8_t,
    0x94i32 as uint8_t,
    0x17i32 as uint8_t,
    0x50i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xa4i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xa3i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0xa2i32 as uint8_t,
    0x2fi32 as uint8_t,
    0x21i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xa3i32 as uint8_t,
    0i32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x80i32 as uint8_t,
    0x7ai32 as uint8_t,
    0xa3i32 as uint8_t,
    0x42i32 as uint8_t,
    0x3i32 as uint8_t,
    0x18i32 as uint8_t,
    0xaci32 as uint8_t,
    0x6fi32 as uint8_t,
    0x22i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x24i32 as uint8_t,
    0xc8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x3di32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x1fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x46i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xaci32 as uint8_t,
    0i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x46i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x2ci32 as uint8_t,
    0x8i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0x80i32 as uint8_t,
    0x40i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x3di32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x74i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x3di32 as uint8_t,
    0xbei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0x46i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0x83i32 as uint8_t,
    0x93i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x15i32 as uint8_t,
    0xb4i32 as uint8_t,
    0xbdi32 as uint8_t,
    0x80i32 as uint8_t,
    0x1fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x46i32 as uint8_t,
    0x8i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x1fi32 as uint8_t,
    0xf0i32 as uint8_t,
    0xa1i32 as uint8_t,
    0xc0i32 as uint8_t,
    0xe0i32 as uint8_t,
    0x70i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x3di32 as uint8_t,
    0x88i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0xd1i32 as uint8_t,
    0i32 as uint8_t,
    0x57i32 as uint8_t,
    0x88i32 as uint8_t,
    0xd2i32 as uint8_t,
    0i32 as uint8_t,
    0x5ai32 as uint8_t,
    0x48i32 as uint8_t,
    0xcfi32 as uint8_t,
    0x80i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x4fi32 as uint8_t,
    0x88i32 as uint8_t,
    0x46i32 as uint8_t,
    0x3ei32 as uint8_t,
    0x84i32 as uint8_t,
    0x80i32 as uint8_t,
    0i32 as uint8_t,
    0x48i32 as uint8_t,
    0xd9i32 as uint8_t,
    0x40i32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
    0x29i32 as uint8_t,
    0x43i32 as uint8_t,
    0xffi32 as uint8_t,
    0x3ai32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x13i32 as uint8_t,
    0xb4i32 as uint8_t,
    0xc6i32 as uint8_t,
    0i32 as uint8_t,
    0x3di32 as uint8_t,
    0x88i32 as uint8_t,
    0xa2i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x14i32 as uint8_t,
    0x74i32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xaci32 as uint8_t,
    0xcdi32 as uint8_t,
    0x3i32 as uint8_t,
    0xd0i32 as uint8_t,
    0xaci32 as uint8_t,
    0xcdi32 as uint8_t,
    0x8i32 as uint8_t,
    0x40i32 as uint8_t,
];
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
/* Radio calibration firmware */
/* text_cal_sx1257_16_Nov_1 */
/* Buffer to hold RX data */
#[no_mangle]
pub static mut rx_buffer: rx_buffer_t = rx_buffer_t {
    buffer: [0; 4096],
    buffer_size: 0,
    buffer_index: 0,
    buffer_pkt_nb: 0,
};
/* Internal timestamp counter */
#[no_mangle]
pub static mut counter_us: timestamp_counter_t = timestamp_counter_t {
    counter_us_raw_27bits_inst_prev: 0,
    counter_us_raw_27bits_pps_prev: 0,
    counter_us_raw_27bits_inst_wrap: 0,
    counter_us_raw_27bits_pps_wrap: 0,
};
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS DEFINITION ----------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn calculate_freq_to_time_drift(
    mut freq_hz: uint32_t,
    mut bw: uint8_t,
    mut mant: *mut uint16_t,
    mut exp: *mut uint8_t,
) -> libc::c_int {
    let mut mantissa_u64: uint64_t = 0;
    let mut exponent: uint8_t = 0i32 as uint8_t;
    let mut bw_hz: int32_t = 0;
    /* check input variables */
    if mant.is_null() {
        return -1i32;
    }
    if exp.is_null() {
        return -1i32;
    }
    bw_hz = lgw_bw_getval(bw as libc::c_int);
    if bw_hz < 0i32 {
        printf(
            b"ERROR: Unsupported bandwidth for frequency to time drift calculation\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    mantissa_u64 = (bw_hz as uint64_t)
        .wrapping_mul((2i32 << 20i32 - 1i32) as libc::c_ulong)
        .wrapping_div(freq_hz as libc::c_ulong);
    while mantissa_u64 < 2048i32 as libc::c_ulong {
        exponent = (exponent as libc::c_int + 1i32) as uint8_t;
        mantissa_u64 <<= 1i32
    }
    *mant = mantissa_u64 as uint16_t;
    *exp = exponent;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn lora_crc16(data: libc::c_char, mut crc: *mut libc::c_int) {
    let mut next: libc::c_int = 0i32;
    next = data as libc::c_int >> 0i32 & 1i32 ^ *crc >> 12i32 & 1i32 ^ *crc >> 8i32 & 1i32;
    next +=
        (data as libc::c_int >> 1i32 & 1i32 ^ *crc >> 13i32 & 1i32 ^ *crc >> 9i32 & 1i32) << 1i32;
    next +=
        (data as libc::c_int >> 2i32 & 1i32 ^ *crc >> 14i32 & 1i32 ^ *crc >> 10i32 & 1i32) << 2i32;
    next +=
        (data as libc::c_int >> 3i32 & 1i32 ^ *crc >> 15i32 & 1i32 ^ *crc >> 11i32 & 1i32) << 3i32;
    next += (data as libc::c_int >> 4i32 & 1i32 ^ *crc >> 12i32 & 1i32) << 4i32;
    next += (data as libc::c_int >> 5i32 & 1i32
        ^ *crc >> 13i32 & 1i32
        ^ *crc >> 12i32 & 1i32
        ^ *crc >> 8i32 & 1i32)
        << 5i32;
    next += (data as libc::c_int >> 6i32 & 1i32
        ^ *crc >> 14i32 & 1i32
        ^ *crc >> 13i32 & 1i32
        ^ *crc >> 9i32 & 1i32)
        << 6i32;
    next += (data as libc::c_int >> 7i32 & 1i32
        ^ *crc >> 15i32 & 1i32
        ^ *crc >> 14i32 & 1i32
        ^ *crc >> 10i32 & 1i32)
        << 7i32;
    next += (*crc >> 0i32 & 1i32 ^ *crc >> 15i32 & 1i32 ^ *crc >> 11i32 & 1i32) << 8i32;
    next += (*crc >> 1i32 & 1i32 ^ *crc >> 12i32 & 1i32) << 9i32;
    next += (*crc >> 2i32 & 1i32 ^ *crc >> 13i32 & 1i32) << 10i32;
    next += (*crc >> 3i32 & 1i32 ^ *crc >> 14i32 & 1i32) << 11i32;
    next +=
        (*crc >> 4i32 & 1i32 ^ *crc >> 15i32 & 1i32 ^ *crc >> 12i32 & 1i32 ^ *crc >> 8i32 & 1i32)
            << 12i32;
    next += (*crc >> 5i32 & 1i32 ^ *crc >> 13i32 & 1i32 ^ *crc >> 9i32 & 1i32) << 13i32;
    next += (*crc >> 6i32 & 1i32 ^ *crc >> 14i32 & 1i32 ^ *crc >> 10i32 & 1i32) << 14i32;
    next += (*crc >> 7i32 & 1i32 ^ *crc >> 15i32 & 1i32 ^ *crc >> 11i32 & 1i32) << 15i32;
    *crc = next;
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief TODO
@param TODO
@return TODO
*/
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_init(mut conf_ts: *mut lgw_conf_timestamp_s) {
    timestamp_counter_new(&mut counter_us);
    if !conf_ts.is_null() {
        timestamp_counter_mode(
            (*conf_ts).enable_precision_ts,
            (*conf_ts).max_ts_metrics,
            (*conf_ts).nb_symbols,
        );
    }
    /* Initialize RX buffer */
    rx_buffer_new(&mut rx_buffer);
}
/* *
@brief Get the SX1302 unique identifier
@param eui  pointerto the memory holding the concentrator EUI
@return LGW_REG_SUCCESS if no error, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_get_eui(mut eui: *mut uint64_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut val: int32_t = 0;
    *eui = 0i32 as uint64_t;
    i = 0i32;
    while i < 8i32 {
        err = lgw_reg_w(826i32 as uint16_t, i);
        if err != 0i32 {
            return -1i32;
        }
        err = lgw_reg_r(827i32 as uint16_t, &mut val);
        if err != 0i32 {
            return -1i32;
        }
        *eui |= (val as uint8_t as uint64_t) << 56i32 - i * 8i32;
        i += 1
    }
    return 0i32;
}
/* *
@brief Check AGC & ARB MCUs parity error, and update timestamp counter wraping status
@brief This function needs to be called regularly (every few seconds) by the upper layer
@param N/A
@return LGW_REG_SUCCESS if no error, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_update() -> libc::c_int {
    let mut val: int32_t = 0;
    /* Check MCUs parity errors */
    lgw_reg_r(22i32 as uint16_t, &mut val);
    if val != 0i32 {
        printf(
            b"ERROR: Parity error check failed on AGC firmware\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    lgw_reg_r(745i32 as uint16_t, &mut val);
    if val != 0i32 {
        printf(
            b"ERROR: Parity error check failed on ARB firmware\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Update internal timestamp counter wrapping status */
    timestamp_counter_get(&mut counter_us, 0i32 != 0); /* maintain inst counter */
    timestamp_counter_get(&mut counter_us, 1i32 != 0); /* maintain pps counter */
    return 0i32;
}
/* *
@brief Select the clock source radio
@param rf_chain The RF chain index from which to get the clock source
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_clock_select(mut rf_chain: uint8_t) -> libc::c_int {
    /* Check input parameters */
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    /* Switch SX1302 clock from SPI clock to radio clock of the selected RF chain */
    match rf_chain as libc::c_int {
        0 => {
            lgw_reg_w(56i32 as uint16_t, 0x1i32);
            lgw_reg_w(55i32 as uint16_t, 0i32);
        }
        1 => {
            lgw_reg_w(56i32 as uint16_t, 0i32);
            lgw_reg_w(55i32 as uint16_t, 0x1i32);
        }
        _ => return -1i32,
    }
    /* Enable clock dividers */
    lgw_reg_w(54i32 as uint16_t, 0x1i32);
    /* Set the RIF clock to the 32MHz clock of the radio */
    lgw_reg_w(1i32 as uint16_t, 0x1i32);
    return 0i32;
}
/* *
@brief Apply the radio reset sequence to the required RF chain index
@param rf_chain The RF chain index of the radio to be reset
@param type     The type of radio to be reset
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_reset(
    mut rf_chain: uint8_t,
    mut type_0: lgw_radio_type_t,
) -> libc::c_int {
    let mut reg_radio_en: uint16_t = 0;
    let mut reg_radio_rst: uint16_t = 0;
    /* Check input parameters */
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    if type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1255 as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1257 as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint
    {
        return -1i32;
    }
    /* Switch to SPI clock before reseting the radio */
    lgw_reg_w(1i32 as uint16_t, 0i32);
    /* Enable the radio */
    reg_radio_en = if rf_chain as libc::c_int == 0i32 {
        27i32
    } else {
        31i32
    } as uint16_t;
    lgw_reg_w(reg_radio_en, 0x1i32);
    /* Select the proper reset sequence depending on the radio type */
    reg_radio_rst = if rf_chain as libc::c_int == 0i32 {
        26i32
    } else {
        30i32
    } as uint16_t; /* wait for auto calibration to complete */
    lgw_reg_w(reg_radio_rst, 0x1i32);
    wait_ms(500i32 as libc::c_ulong);
    lgw_reg_w(reg_radio_rst, 0i32);
    wait_ms(10i32 as libc::c_ulong);
    match type_0 as libc::c_uint {
        1 | 2 => {}
        5 => {
            lgw_reg_w(reg_radio_rst, 0x1i32);
            wait_ms(10i32 as libc::c_ulong);
        }
        _ => return -1i32,
    }
    return 0i32;
}
/* *
@brief Configure the radio type for the given RF chain
@param rf_chain The RF chain index to be configured
@param type     The type of radio to be set for the given RF chain
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_set_mode(
    mut rf_chain: uint8_t,
    mut type_0: lgw_radio_type_t,
) -> libc::c_int {
    let mut reg: uint16_t = 0;
    /* Check input parameters */
    if rf_chain as libc::c_int >= 2i32 {
        return -1i32;
    }
    if type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1255 as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1257 as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint
    {
        return -1i32;
    }
    /* Set the radio mode */
    reg = if rf_chain as libc::c_int == 0i32 {
        5i32
    } else {
        4i32
    } as uint16_t;
    match type_0 as libc::c_uint {
        5 => {
            lgw_reg_w(reg, 0x1i32);
        }
        _ => {
            lgw_reg_w(reg, 0i32);
        }
    }
    return 0i32;
}
/* *
@brief Give/Release control over the radios to/from the Host
@param host_ctrl    Set to true to give control to the host, false otherwise
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_host_ctrl(mut host_ctrl: bool) -> libc::c_int {
    lgw_reg_w(
        2i32 as uint16_t,
        if host_ctrl as libc::c_int == 0i32 {
            0i32
        } else {
            0x1i32
        },
    );
    return 0i32;
}
/* *
@brief Perform the radio calibration sequence and fill the TX gain LUT with calibration offsets
@param context_rf_chain The RF chains array from which to get RF chains current configuration
@param clksrc           The RF chain index which provides the clock source
@param txgain_lut       A pointer to the TX gain LUT to be filled
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_calibrate(
    mut context_rf_chain: *mut lgw_conf_rxrf_s,
    mut clksrc: uint8_t,
    mut txgain_lut: *mut lgw_tx_gain_lut_s,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* -- Reset radios */
    i = 0i32;
    while i < 2i32 {
        if (*context_rf_chain.offset(i as isize)).enable as libc::c_int == 1i32 {
            sx1302_radio_reset(i as uint8_t, (*context_rf_chain.offset(i as isize)).type_0);
            sx1302_radio_set_mode(i as uint8_t, (*context_rf_chain.offset(i as isize)).type_0);
        }
        i += 1
    }
    /* -- Select the radio which provides the clock to the sx1302 */
    sx1302_radio_clock_select(clksrc);
    /* -- Ensure PA/LNA are disabled */
    lgw_reg_w(19i32 as uint16_t, 1i32);
    lgw_reg_w(28i32 as uint16_t, 0i32);
    lgw_reg_w(29i32 as uint16_t, 0i32);
    /* -- Start calibration */
    if (*context_rf_chain.offset(clksrc as isize)).type_0 as libc::c_uint
        == LGW_RADIO_TYPE_SX1257 as libc::c_int as libc::c_uint
        || (*context_rf_chain.offset(clksrc as isize)).type_0 as libc::c_uint
            == LGW_RADIO_TYPE_SX1255 as libc::c_int as libc::c_uint
    {
        if sx1302_agc_load_firmware(cal_firmware_sx125x.as_mut_ptr()) != 0i32 {
            printf(
                b"ERROR: Failed to load calibration fw\n\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
        }
        if sx1302_cal_start(1i32 as uint8_t, context_rf_chain, txgain_lut) != 0i32 {
            printf(b"ERROR: radio calibration failed\n\x00" as *const u8 as *const libc::c_char);
            sx1302_radio_reset(0i32 as uint8_t, (*context_rf_chain.offset(0)).type_0);
            sx1302_radio_reset(1i32 as uint8_t, (*context_rf_chain.offset(1)).type_0);
            return -1i32;
        }
    } else {
        i = 0i32;
        while i < 2i32 {
            if (*context_rf_chain.offset(i as isize)).enable as libc::c_int == 1i32 {
                if sx1250_calibrate(i as uint8_t, (*context_rf_chain.offset(i as isize)).freq_hz)
                    != 0
                {
                    printf(
                        b"ERROR: radio calibration failed\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -1i32;
                }
            }
            i += 1
        }
    }
    /* -- Release control over FE */
    lgw_reg_w(19i32 as uint16_t, 0i32);
    return 0i32;
}
/* *
@brief Configure the PA and LNA LUTs
@param N/A
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_pa_lna_lut_configure() -> libc::c_int {
    lgw_reg_w(34i32 as uint16_t, 0x4i32); /* Enable PA: RADIO_CTRL[2] is high when PA_EN=1 & LNA_EN=0 */
    lgw_reg_w(36i32 as uint16_t, 0x4i32); /* Enable PA: RADIO_CTRL[8] is high when PA_EN=1 & LNA_EN=0 */
    lgw_reg_w(35i32 as uint16_t, 0x2i32); /* Enable LNA: RADIO_CTRL[1] is high when PA_EN=0 & LNA_EN=1 */
    lgw_reg_w(37i32 as uint16_t, 0x2i32); /* Enable LNA: RADIO_CTRL[7] is high when PA_EN=0 & LNA_EN=1 */
    return 0i32;
}
/* *
@brief Configure the Radio Front-End stage of the SX1302
@param N/A
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_radio_fe_configure() -> libc::c_int {
    lgw_reg_w(793i32 as uint16_t, 0x3i32);
    lgw_reg_w(794i32 as uint16_t, 0x7i32);
    lgw_reg_w(808i32 as uint16_t, 0x3i32);
    lgw_reg_w(809i32 as uint16_t, 0x7i32);
    lgw_reg_w(788i32 as uint16_t, 23i32);
    lgw_reg_w(789i32 as uint16_t, 66i32);
    lgw_reg_w(803i32 as uint16_t, 23i32);
    lgw_reg_w(804i32 as uint16_t, 66i32);
    lgw_reg_w(785i32 as uint16_t, 1i32);
    lgw_reg_w(787i32 as uint16_t, 0xbi32);
    lgw_reg_w(800i32 as uint16_t, 1i32);
    lgw_reg_w(802i32 as uint16_t, 0xbi32);
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_get_ifmod_config(mut if_chain: uint8_t) -> uint8_t {
    return ifmod_config[if_chain as usize];
}
/* *
@brief Configure the channelizer stage of the SX1302
@param if_cfg   A pointer to the channels configuration
@param fix_gain Set to true to force the channelizer to a fixed gain, false to let the AGC controlling it
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_channelizer_configure(
    mut if_cfg: *mut lgw_conf_rxif_s,
    mut fix_gain: bool,
) -> libc::c_int {
    let mut if_freq: int32_t = 0;
    let mut channels_mask: uint8_t = 0i32 as uint8_t;
    let mut i: libc::c_int = 0;
    /* Check input parameters */
    if if_cfg.is_null() {
        printf(
            b"ERROR: Failed to configure LoRa channelizer\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Select which radio is connected to each multi-SF channel */
    i = 0i32;
    while i < 8i32 {
        channels_mask = (channels_mask as libc::c_int
            | ((*if_cfg.offset(i as isize)).rf_chain as libc::c_int) << i)
            as uint8_t;
        i += 1
    }
    lgw_reg_w(340i32 as uint16_t, channels_mask as int32_t);
    /* Select which radio is connected to the LoRa service channel */
    lgw_reg_w(848i32 as uint16_t, (*if_cfg.offset(8)).rf_chain as int32_t);
    /* Select which radio is connected to the FSK channel */
    lgw_reg_w(1009i32 as uint16_t, (*if_cfg.offset(9)).rf_chain as int32_t);
    /* Configure multi-SF channels IF frequencies */
    if_freq = ((*if_cfg.offset(0)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(324i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(325i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(1)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(326i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(327i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(2)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(328i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(329i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(3)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(330i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(331i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(4)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(332i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(333i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(5)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(334i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(335i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(6)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(336i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(337i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    if_freq = ((*if_cfg.offset(7)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(338i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(339i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    /* Configure LoRa service channel IF frequency */
    if_freq = ((*if_cfg.offset(8)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(846i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(847i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    /* Configure FSK channel IF frequency */
    if_freq = ((*if_cfg.offset(9)).freq_hz << 5i32) / 15625i32;
    lgw_reg_w(998i32 as uint16_t, if_freq >> 8i32 & 0x1fi32);
    lgw_reg_w(999i32 as uint16_t, if_freq >> 0i32 & 0xffi32);
    /* Set the low pass filtering corner frequency for RSSI indicator */
    lgw_reg_w(341i32 as uint16_t, 0x5i32);
    /* Set the channelizer RSSI reset value */
    lgw_reg_w(343i32 as uint16_t, 85i32);
    /* Force channelizer in fix gain, or let it be controlled by AGC */
    if fix_gain as libc::c_int == 1i32 {
        lgw_reg_w(349i32 as uint16_t, 0i32);
        lgw_reg_w(352i32 as uint16_t, 5i32);
    } else {
        /* Allow the AGC to control gains */
        lgw_reg_w(349i32 as uint16_t, 0x1i32);
        /* Disable the internal DAGC */
        lgw_reg_w(344i32 as uint16_t, 255i32);
        lgw_reg_w(345i32 as uint16_t, 0i32);
        lgw_reg_w(346i32 as uint16_t, 15i32);
        lgw_reg_w(347i32 as uint16_t, 0i32);
    }
    return 0i32;
}
/* *
@brief Configure the FSK modem
@param cfg  A pointer to the channel configuration
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_fsk_configure(mut cfg: *mut lgw_conf_rxif_s) -> libc::c_int {
    let mut fsk_sync_word_reg: uint64_t = 0; /* 125KHz */
    let mut fsk_br_reg: uint32_t = 0; /* variable length */
    lgw_reg_w(
        1005i32 as uint16_t,
        (*cfg).sync_word_size as libc::c_int - 1i32,
    ); /* ?? */
    fsk_sync_word_reg = (*cfg).sync_word << 8i32 * (8i32 - (*cfg).sync_word_size as libc::c_int);
    lgw_reg_w(
        1027i32 as uint16_t,
        (fsk_sync_word_reg >> 0i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1026i32 as uint16_t,
        (fsk_sync_word_reg >> 8i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1025i32 as uint16_t,
        (fsk_sync_word_reg >> 16i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1024i32 as uint16_t,
        (fsk_sync_word_reg >> 24i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1023i32 as uint16_t,
        (fsk_sync_word_reg >> 32i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1022i32 as uint16_t,
        (fsk_sync_word_reg >> 40i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1021i32 as uint16_t,
        (fsk_sync_word_reg >> 48i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1020i32 as uint16_t,
        (fsk_sync_word_reg >> 56i32) as uint8_t as int32_t,
    );
    fsk_br_reg = (32000000i32 as libc::c_uint).wrapping_div((*cfg).datarate);
    lgw_reg_w(
        1018i32 as uint16_t,
        (fsk_br_reg >> 8i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        1019i32 as uint16_t,
        (fsk_br_reg >> 0i32) as uint8_t as int32_t,
    );
    lgw_reg_w(1006i32 as uint16_t, 0x3i32);
    lgw_reg_w(1010i32 as uint16_t, 0i32);
    lgw_reg_w(1007i32 as uint16_t, 0i32);
    lgw_reg_w(1011i32 as uint16_t, 4i32);
    lgw_reg_w(1003i32 as uint16_t, 1i32);
    lgw_reg_w(1002i32 as uint16_t, 1i32);
    lgw_reg_w(1001i32 as uint16_t, 2i32);
    lgw_reg_w(1000i32 as uint16_t, 0i32);
    lgw_reg_w(1012i32 as uint16_t, 10i32);
    lgw_reg_w(1015i32 as uint16_t, 255i32);
    lgw_reg_w(1013i32 as uint16_t, 0i32);
    lgw_reg_w(1014i32 as uint16_t, 0i32);
    lgw_reg_w(1008i32 as uint16_t, 1i32);
    lgw_reg_w(1016i32 as uint16_t, 0i32);
    lgw_reg_w(1017i32 as uint16_t, 128i32);
    return 0i32;
}
/* *
@brief Configure the correlator stage of the SX1302 LoRa multi-SF modems
@param N/A
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_correlator_configure() -> libc::c_int {
    lgw_reg_w(367i32 as uint16_t, 52i32); /* 12 11 10 9 8 7 6 5 */
    lgw_reg_w(369i32 as uint16_t, 24i32); /* 1 correlator per channel */
    lgw_reg_w(371i32 as uint16_t, 7i32);
    lgw_reg_w(374i32 as uint16_t, 5i32);
    lgw_reg_w(383i32 as uint16_t, 52i32);
    lgw_reg_w(385i32 as uint16_t, 24i32);
    lgw_reg_w(387i32 as uint16_t, 7i32);
    lgw_reg_w(390i32 as uint16_t, 5i32);
    lgw_reg_w(399i32 as uint16_t, 52i32);
    lgw_reg_w(401i32 as uint16_t, 24i32);
    lgw_reg_w(403i32 as uint16_t, 7i32);
    lgw_reg_w(406i32 as uint16_t, 5i32);
    lgw_reg_w(415i32 as uint16_t, 52i32);
    lgw_reg_w(417i32 as uint16_t, 24i32);
    lgw_reg_w(419i32 as uint16_t, 7i32);
    lgw_reg_w(422i32 as uint16_t, 5i32);
    lgw_reg_w(431i32 as uint16_t, 52i32);
    lgw_reg_w(433i32 as uint16_t, 24i32);
    lgw_reg_w(435i32 as uint16_t, 7i32);
    lgw_reg_w(438i32 as uint16_t, 5i32);
    lgw_reg_w(447i32 as uint16_t, 52i32);
    lgw_reg_w(449i32 as uint16_t, 24i32);
    lgw_reg_w(451i32 as uint16_t, 7i32);
    lgw_reg_w(454i32 as uint16_t, 5i32);
    lgw_reg_w(463i32 as uint16_t, 52i32);
    lgw_reg_w(465i32 as uint16_t, 24i32);
    lgw_reg_w(467i32 as uint16_t, 7i32);
    lgw_reg_w(470i32 as uint16_t, 5i32);
    lgw_reg_w(479i32 as uint16_t, 52i32);
    lgw_reg_w(481i32 as uint16_t, 24i32);
    lgw_reg_w(483i32 as uint16_t, 7i32);
    lgw_reg_w(486i32 as uint16_t, 5i32);
    lgw_reg_w(355i32 as uint16_t, 0xffi32);
    lgw_reg_w(358i32 as uint16_t, 0xffi32);
    lgw_reg_w(359i32 as uint16_t, 0xffi32);
    lgw_reg_w(357i32 as uint16_t, 0xffi32);
    lgw_reg_w(356i32 as uint16_t, 0xffi32);
    /* For debug: get packets with sync_error and header_error in FIFO */
    return 0i32;
}
/* *
@brief Configure the correlator stage of the SX1302 LoRa single-SF modem
@param cfg  A pointer to the channel configuration
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_service_correlator_configure(
    mut cfg: *mut lgw_conf_rxif_s,
) -> libc::c_int {
    /* Common config for all SF */
    lgw_reg_w(976i32 as uint16_t, 7i32);
    lgw_reg_w(975i32 as uint16_t, 5i32);
    lgw_reg_w(981i32 as uint16_t, 1i32);
    match (*cfg).datarate {
        5 => {
            lgw_reg_w(906i32 as uint16_t, 1i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        6 => {
            lgw_reg_w(906i32 as uint16_t, 1i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        7 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        8 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        9 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        10 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        11 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        12 => {
            lgw_reg_w(906i32 as uint16_t, 0i32);
            lgw_reg_w(982i32 as uint16_t, 52i32);
        }
        _ => {
            printf(
                b"ERROR: Failed to configure LoRa service modem correlators\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return -1i32;
        }
    }
    return 0i32;
}
/* *
@brief Configure the LoRa multi-SF modems
@param radio_freq_hz    The center frequency of the RF chain (0 or 1)
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_modem_configure(mut radio_freq_hz: uint32_t) -> libc::c_int {
    let mut mantissa: uint16_t = 0i32 as uint16_t;
    let mut exponent: uint8_t = 0i32 as uint8_t;
    /* TODO: test if channel is enabled */
    lgw_reg_w(493i32 as uint16_t, 0i32);
    lgw_reg_w(517i32 as uint16_t, 0x1i32);
    lgw_reg_w(530i32 as uint16_t, 0x1i32);
    lgw_reg_w(528i32 as uint16_t, 0x1i32);
    /* Enable full modems */
    lgw_reg_w(841i32 as uint16_t, 0xffi32);
    /* Enable limited modems */
    lgw_reg_w(842i32 as uint16_t, 0xffi32);
    /* Configure coarse sync between correlators and modems */
    lgw_reg_w(693i32 as uint16_t, 0i32);
    lgw_reg_w(694i32 as uint16_t, 126i32);
    /* Configure fine sync offset for each channel */
    lgw_reg_w(775i32 as uint16_t, 1i32);
    lgw_reg_w(774i32 as uint16_t, 5i32);
    lgw_reg_w(777i32 as uint16_t, 9i32);
    lgw_reg_w(776i32 as uint16_t, 13i32);
    lgw_reg_w(779i32 as uint16_t, 1i32);
    lgw_reg_w(778i32 as uint16_t, 5i32);
    lgw_reg_w(781i32 as uint16_t, 9i32);
    lgw_reg_w(780i32 as uint16_t, 13i32);
    /* Configure PPM offset */
    lgw_reg_w(698i32 as uint16_t, 0i32);
    lgw_reg_w(697i32 as uint16_t, 0i32);
    lgw_reg_w(696i32 as uint16_t, 0i32);
    lgw_reg_w(695i32 as uint16_t, 0i32);
    lgw_reg_w(702i32 as uint16_t, 0i32);
    lgw_reg_w(701i32 as uint16_t, 0i32);
    lgw_reg_w(700i32 as uint16_t, 0x1i32);
    lgw_reg_w(699i32 as uint16_t, 0x1i32);
    /* Improve SF5 and SF6 performances */
    lgw_reg_w(590i32 as uint16_t, 3i32); // Default is 1
    lgw_reg_w(591i32 as uint16_t, 3i32); // Default is 2
                                         /* Improve SF11/SF12 performances */
    lgw_reg_w(603i32 as uint16_t, 1i32);
    lgw_reg_w(602i32 as uint16_t, 1i32);
    lgw_reg_w(628i32 as uint16_t, 1i32);
    lgw_reg_w(627i32 as uint16_t, 1i32);
    /* Freq2TimeDrift computation */
    if calculate_freq_to_time_drift(
        radio_freq_hz,
        0x4i32 as uint8_t,
        &mut mantissa,
        &mut exponent,
    ) != 0i32
    {
        printf(
            b"ERROR: failed to calculate frequency to time drift for LoRa modem\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    lgw_reg_w(
        633i32 as uint16_t,
        mantissa as libc::c_int >> 8i32 & 0xffi32,
    );
    lgw_reg_w(634i32 as uint16_t, mantissa as libc::c_int & 0xffi32);
    lgw_reg_w(635i32 as uint16_t, exponent as int32_t);
    /* Time drift compensation */
    lgw_reg_w(639i32 as uint16_t, 1i32);
    return 0i32;
}
/* *
@brief Configure the LoRa single-SF modem
@param cfg              A pointer to the channel configuration
@param radio_freq_hz    The center frequency of the RF chain (0 or 1)
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_service_modem_configure(
    mut cfg: *mut lgw_conf_rxif_s,
    mut radio_freq_hz: uint32_t,
) -> libc::c_int {
    let mut mantissa: uint16_t = 0i32 as uint16_t; // Default value
    let mut exponent: uint8_t = 0i32 as uint8_t; // Default value
    lgw_reg_w(853i32 as uint16_t, 0i32);
    lgw_reg_w(877i32 as uint16_t, 0x1i32);
    lgw_reg_w(890i32 as uint16_t, 0x1i32);
    lgw_reg_w(888i32 as uint16_t, 0x1i32);
    lgw_reg_w(944i32 as uint16_t, 0x3i32);
    lgw_reg_w(948i32 as uint16_t, 0x3i32);
    match (*cfg).datarate {
        5 | 6 => {
            lgw_reg_w(946i32 as uint16_t, 0x4i32);
            lgw_reg_w(947i32 as uint16_t, 0i32);
        }
        7 | 8 | 9 | 10 => {
            lgw_reg_w(946i32 as uint16_t, 0x6i32);
            lgw_reg_w(947i32 as uint16_t, 0i32);
        }
        11 | 12 => {
            lgw_reg_w(946i32 as uint16_t, 0x7i32);
            match (*cfg).bandwidth as libc::c_int {
                4 => {
                    lgw_reg_w(947i32 as uint16_t, 0x1i32);
                }
                5 => {
                    lgw_reg_w(947i32 as uint16_t, 0x2i32);
                }
                6 => {
                    lgw_reg_w(947i32 as uint16_t, 0x3i32);
                }
                _ => {
                    printf(
                        b"ERROR: unsupported bandwidth %u for LoRa Service modem\n\x00" as *const u8
                            as *const libc::c_char,
                        (*cfg).bandwidth as libc::c_int,
                    );
                }
            }
        }
        _ => {
            printf(
                b"ERROR: unsupported datarate %u for LoRa Service modem\n\x00" as *const u8
                    as *const libc::c_char,
                (*cfg).datarate,
            );
        }
    }
    lgw_reg_w(
        909i32 as uint16_t,
        if (*cfg).implicit_hdr as libc::c_int == 1i32 {
            1i32
        } else {
            0i32
        },
    );
    lgw_reg_w(
        910i32 as uint16_t,
        if (*cfg).implicit_crc_en as libc::c_int == 1i32 {
            1i32
        } else {
            0i32
        },
    );
    lgw_reg_w(905i32 as uint16_t, (*cfg).implicit_coderate as int32_t);
    lgw_reg_w(
        911i32 as uint16_t,
        (*cfg).implicit_payload_length as int32_t,
    );
    lgw_reg_w(901i32 as uint16_t, (*cfg).datarate as int32_t);
    lgw_reg_w(900i32 as uint16_t, (*cfg).bandwidth as int32_t);
    lgw_reg_w(
        903i32 as uint16_t,
        ((*cfg).bandwidth as libc::c_int == 0x4i32
            && ((*cfg).datarate == 11i32 as libc::c_uint
                || (*cfg).datarate == 12i32 as libc::c_uint)
            || (*cfg).bandwidth as libc::c_int == 0x5i32
                && (*cfg).datarate == 12i32 as libc::c_uint) as libc::c_int,
    );
    //SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG6_PREAMBLE_SYMB_NB
    //SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG7_PREAMBLE_SYMB_NB
    /* Freq2TimeDrift computation */
    if calculate_freq_to_time_drift(
        radio_freq_hz,
        (*cfg).bandwidth,
        &mut mantissa,
        &mut exponent,
    ) != 0i32
    {
        printf(
            b"ERROR: failed to calculate frequency to time drift for LoRa service modem\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    lgw_reg_w(
        955i32 as uint16_t,
        mantissa as libc::c_int >> 8i32 & 0xffi32,
    );
    lgw_reg_w(956i32 as uint16_t, mantissa as libc::c_int & 0xffi32);
    lgw_reg_w(957i32 as uint16_t, exponent as int32_t);
    /* Time drift compensation */
    lgw_reg_w(961i32 as uint16_t, 1i32);
    lgw_reg_w(880i32 as uint16_t, 1i32);
    lgw_reg_w(904i32 as uint16_t, 1i32);
    lgw_reg_w(908i32 as uint16_t, 1i32);
    lgw_reg_w(907i32 as uint16_t, 1i32);
    return 0i32;
}
/* *
@brief Enable the modems
@param N/A
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_modem_enable() -> libc::c_int {
    /* Enable LoRa multi-SF modems */
    lgw_reg_w(14i32 as uint16_t, 0x1i32);
    /* Enable LoRa service modem */
    lgw_reg_w(15i32 as uint16_t, 0x1i32);
    /* Enable FSK modem */
    lgw_reg_w(13i32 as uint16_t, 0x1i32);
    /* Enable RX */
    lgw_reg_w(12i32 as uint16_t, 0x1i32);
    return 0i32;
}
/* *
@brief Configure the syncword to be used by LoRa modems (public:0x34, private:0x12)
@param public           Set to true to use the "public" syncword, false to use the private one
@param lora_service_sf  The spreading factor configured for the single-SF LoRa modem
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_syncword(
    mut public: bool,
    mut lora_service_sf: uint8_t,
) -> libc::c_int {
    /* Multi-SF modem configuration */
    lgw_reg_w(574i32 as uint16_t, 2i32);
    lgw_reg_w(575i32 as uint16_t, 4i32);
    lgw_reg_w(576i32 as uint16_t, 2i32);
    lgw_reg_w(577i32 as uint16_t, 4i32);
    if public as libc::c_int == 1i32 {
        lgw_reg_w(578i32 as uint16_t, 6i32);
        lgw_reg_w(579i32 as uint16_t, 8i32);
    } else {
        lgw_reg_w(578i32 as uint16_t, 2i32);
        lgw_reg_w(579i32 as uint16_t, 4i32);
    }
    /* LoRa Service modem configuration */
    if public as libc::c_int == 0i32
        || lora_service_sf as libc::c_int == 5i32
        || lora_service_sf as libc::c_int == 6i32
    {
        lgw_reg_w(932i32 as uint16_t, 2i32);
        lgw_reg_w(933i32 as uint16_t, 4i32);
    } else {
        lgw_reg_w(932i32 as uint16_t, 6i32);
        lgw_reg_w(933i32 as uint16_t, 8i32);
    }
    return 0i32;
}
/* *
@brief Get the current SX1302 internal counter value
@param pps      True for getting the counter value at last PPS
@return the counter value in mciroseconds (32-bits)
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_timestamp_counter(mut pps: bool) -> uint32_t {
    return timestamp_counter_get(&mut counter_us, pps);
}
/* *
@brief Enable/Disable the GPS to allow PPS trigger and counter sampling
@param enbale   Set to true to enable, false otherwise
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_gps_enable(mut enable: bool) -> libc::c_int {
    if enable as libc::c_int == 1i32 {
        lgw_reg_w(306i32 as uint16_t, 1i32);
        lgw_reg_w(305i32 as uint16_t, 1i32);
    /* invert polarity for PPS */
    } else {
        lgw_reg_w(306i32 as uint16_t, 0i32);
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_load_firmware(mut firmware: *const uint8_t) -> libc::c_int {
    let mut val: int32_t = 0;
    let mut fw_check: [uint8_t; 8192] = [0; 8192];
    let mut gpio_sel: int32_t = 0x1i32;
    /* Configure GPIO to let AGC MCU access board LEDs */
    lgw_reg_w(282i32 as uint16_t, gpio_sel); /* GPIO output direction */
    lgw_reg_w(283i32 as uint16_t, gpio_sel);
    lgw_reg_w(284i32 as uint16_t, gpio_sel);
    lgw_reg_w(285i32 as uint16_t, gpio_sel);
    lgw_reg_w(286i32 as uint16_t, gpio_sel);
    lgw_reg_w(287i32 as uint16_t, gpio_sel);
    lgw_reg_w(288i32 as uint16_t, gpio_sel);
    lgw_reg_w(289i32 as uint16_t, gpio_sel);
    lgw_reg_w(275i32 as uint16_t, 0xffi32);
    /* Take control over AGC MCU */
    lgw_reg_w(20i32 as uint16_t, 0x1i32);
    lgw_reg_w(21i32 as uint16_t, 0x1i32);
    lgw_reg_w(0i32 as uint16_t, 0i32);
    /* Write AGC fw in AGC MEM */
    lgw_mem_wb(0i32 as uint16_t, firmware, 8192i32 as uint16_t);
    /* Read back and check */
    lgw_mem_rb(
        0i32 as uint16_t,
        fw_check.as_mut_ptr(),
        8192i32 as uint16_t,
        0i32 != 0,
    );
    if memcmp(
        firmware as *const libc::c_void,
        fw_check.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8192]>() as libc::c_ulong,
    ) != 0i32
    {
        printf(b"ERROR: Failed to load fw\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    /* Release control over AGC MCU */
    lgw_reg_w(21i32 as uint16_t, 0i32);
    lgw_reg_w(20i32 as uint16_t, 0i32);
    lgw_reg_r(22i32 as uint16_t, &mut val);
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_status(mut status: *mut uint8_t) -> libc::c_int {
    let mut val: int32_t = 0;
    if lgw_reg_r(23i32 as uint16_t, &mut val) != 0i32 {
        printf(b"ERROR: Failed to get AGC status\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    *status = val as uint8_t;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_wait_status(mut status: uint8_t) -> libc::c_int {
    let mut val: uint8_t = 0;
    loop {
        if sx1302_agc_status(&mut val) != 0i32 {
            return -1i32;
        }
        if !(val as libc::c_int != status as libc::c_int) {
            break;
        }
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_mailbox_read(
    mut mailbox: uint8_t,
    mut value: *mut uint8_t,
) -> libc::c_int {
    let mut reg: uint16_t = 0;
    let mut val: int32_t = 0;
    /* Check parameters */
    if mailbox as libc::c_int > 3i32 {
        printf(b"ERROR: invalid AGC mailbox ID\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    reg = (52i32 - mailbox as libc::c_int) as uint16_t;
    if lgw_reg_r(reg, &mut val) != 0i32 {
        printf(b"ERROR: failed to read AGC mailbox\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    *value = val as uint8_t;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_mailbox_write(
    mut mailbox: uint8_t,
    mut value: uint8_t,
) -> libc::c_int {
    let mut reg: uint16_t = 0;
    /* Check parameters */
    if mailbox as libc::c_int > 3i32 {
        printf(b"ERROR: invalid AGC mailbox ID\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    reg = (48i32 - mailbox as libc::c_int) as uint16_t;
    if lgw_reg_w(reg, value as int32_t) != 0i32 {
        printf(b"ERROR: failed to write AGC mailbox\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_agc_start(
    mut version: uint8_t,
    mut radio_type: lgw_radio_type_t,
    mut ana_gain: uint8_t,
    mut dec_gain: uint8_t,
    mut fdd_mode: uint8_t,
) -> libc::c_int {
    let mut val: uint8_t = 0;
    let mut agc_params: agc_gain_params_s = agc_gain_params_s {
        ana_min: 0,
        ana_max: 0,
        ana_thresh_l: 0,
        ana_thresh_h: 0,
        dec_attn_min: 0,
        dec_attn_max: 0,
        dec_thresh_l: 0,
        dec_thresh_h1: 0,
        dec_thresh_h2: 0,
        chan_attn_min: 0,
        chan_attn_max: 0,
        chan_thresh_l: 0,
        chan_thresh_h: 0,
        deviceSel: 0,
        hpMax: 0,
        paDutyCycle: 0,
    };
    /* Check parameters */
    if radio_type as libc::c_uint != LGW_RADIO_TYPE_SX1255 as libc::c_int as libc::c_uint
        && radio_type as libc::c_uint != LGW_RADIO_TYPE_SX1257 as libc::c_int as libc::c_uint
        && radio_type as libc::c_uint != LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint
    {
        return -1i32;
    }
    /* Wait for AGC fw to be started, and VERSION available in mailbox */
    sx1302_agc_wait_status(0x1i32 as uint8_t); /* fw has started, VERSION is ready in mailbox */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != version as libc::c_int {
        printf(
            b"ERROR: wrong AGC fw version (%d)\n\x00" as *const u8 as *const libc::c_char,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure Radio A gains */
    sx1302_agc_mailbox_write(0i32 as uint8_t, ana_gain); /* 0:auto agc*/
    sx1302_agc_mailbox_write(1i32 as uint8_t, dec_gain);
    if radio_type as libc::c_uint != LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint {
        printf(
            b"AGC: setting fdd_mode to %u\n\x00" as *const u8 as *const libc::c_char,
            fdd_mode as libc::c_int,
        );
        sx1302_agc_mailbox_write(2i32 as uint8_t, fdd_mode);
    }
    /* notify AGC that gains has been set to mailbox for Radio A */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x80i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received gain settings for Radio A */
    sx1302_agc_wait_status(0x2i32 as uint8_t);
    /* Check ana_gain setting */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != ana_gain as libc::c_int {
        printf(
            b"ERROR: Analog gain of Radio A has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Check dec_gain setting */
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != dec_gain as libc::c_int {
        printf(
            b"ERROR: Decimator gain of Radio A has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Check FDD mode setting */
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut val);
    if val as libc::c_int != fdd_mode as libc::c_int {
        printf(
            b"ERROR: FDD mode of Radio A has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Configure Radio B gains */
    sx1302_agc_mailbox_write(0i32 as uint8_t, ana_gain); /* 0:auto agc*/
    sx1302_agc_mailbox_write(1i32 as uint8_t, dec_gain);
    if radio_type as libc::c_uint != LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint {
        sx1302_agc_mailbox_write(2i32 as uint8_t, fdd_mode);
    }
    /* notify AGC that gains has been set to mailbox for Radio B */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x20i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received gain settings for Radio B */
    sx1302_agc_wait_status(0x3i32 as uint8_t);
    /* Check ana_gain setting */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != ana_gain as libc::c_int {
        printf(
            b"ERROR: Analog gain of Radio B has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Check dec_gain setting */
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != dec_gain as libc::c_int {
        printf(
            b"ERROR: Decimator gain of Radio B has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Check FDD mode setting */
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut val);
    if val as libc::c_int != fdd_mode as libc::c_int {
        printf(
            b"ERROR: FDD mode of Radio B has not been set properly\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* Configure AGC gains */
    agc_params =
        if radio_type as libc::c_uint == LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint {
            agc_params_sx1250
        } else {
            agc_params_sx125x
        };
    /* Configure analog gain min/max */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.ana_min);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.ana_max);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x3i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x4i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.ana_min as libc::c_int {
        printf(
            b"ERROR: wrong ana_min (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.ana_min as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.ana_max as libc::c_int {
        printf(
            b"ERROR: ana_max (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.ana_max as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure analog thresholds */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.ana_thresh_l);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.ana_thresh_h);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x4i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x5i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.ana_thresh_l as libc::c_int {
        printf(
            b"ERROR: wrong ana_thresh_l (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.ana_thresh_l as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.ana_thresh_h as libc::c_int {
        printf(
            b"ERROR: wrong ana_thresh_h (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.ana_thresh_h as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure decimator attenuation min/max */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.dec_attn_min);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.dec_attn_max);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x5i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x6i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.dec_attn_min as libc::c_int {
        printf(
            b"ERROR: wrong dec_attn_min (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.dec_attn_min as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.dec_attn_max as libc::c_int {
        printf(
            b"ERROR: wrong dec_attn_max (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.dec_attn_max as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure decimator attenuation thresholds */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.dec_thresh_l);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.dec_thresh_h1);
    sx1302_agc_mailbox_write(2i32 as uint8_t, agc_params.dec_thresh_h2);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x6i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x7i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.dec_thresh_l as libc::c_int {
        printf(
            b"ERROR: wrong dec_thresh_l (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.dec_thresh_l as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.dec_thresh_h1 as libc::c_int {
        printf(
            b"ERROR: wrong dec_thresh_h1 (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.dec_thresh_h1 as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(2i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.dec_thresh_h2 as libc::c_int {
        printf(
            b"ERROR: wrong dec_thresh_h2 (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.dec_thresh_h2 as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure channel attenuation min/max */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.chan_attn_min);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.chan_attn_max);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x7i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x8i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.chan_attn_min as libc::c_int {
        printf(
            b"ERROR: wrong chan_attn_min (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.chan_attn_min as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.chan_attn_max as libc::c_int {
        printf(
            b"ERROR: wrong chan_attn_max (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.chan_attn_max as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Configure channel attenuation threshold */
    sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.chan_thresh_l);
    sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.chan_thresh_h);
    /* notify AGC that params have been set to mailbox */
    sx1302_agc_mailbox_write(3i32 as uint8_t, 0x8i32 as uint8_t);
    /* Wait for AGC to acknoledge it has received params */
    sx1302_agc_wait_status(0x9i32 as uint8_t);
    /* Check params */
    sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.chan_thresh_l as libc::c_int {
        printf(
            b"ERROR: wrong chan_thresh_l (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.chan_thresh_l as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
    if val as libc::c_int != agc_params.chan_thresh_h as libc::c_int {
        printf(
            b"ERROR: wrong chan_thresh_h (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
            agc_params.chan_thresh_h as libc::c_int,
            val as libc::c_int,
        );
        return -1i32;
    }
    if radio_type as libc::c_uint == LGW_RADIO_TYPE_SX1250 as libc::c_int as libc::c_uint {
        /* Configure sx1250 SetPAConfig */
        sx1302_agc_mailbox_write(0i32 as uint8_t, agc_params.deviceSel);
        sx1302_agc_mailbox_write(1i32 as uint8_t, agc_params.hpMax);
        sx1302_agc_mailbox_write(2i32 as uint8_t, agc_params.paDutyCycle);
        /* notify AGC that params have been set to mailbox */
        sx1302_agc_mailbox_write(3i32 as uint8_t, 0x9i32 as uint8_t);
        /* Wait for AGC to acknoledge it has received params */
        sx1302_agc_wait_status(0xai32 as uint8_t);
        /* Check params */
        sx1302_agc_mailbox_read(0i32 as uint8_t, &mut val);
        if val as libc::c_int != agc_params.deviceSel as libc::c_int {
            printf(
                b"ERROR: wrong deviceSel (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
                agc_params.deviceSel as libc::c_int,
                val as libc::c_int,
            );
            return -1i32;
        }
        sx1302_agc_mailbox_read(1i32 as uint8_t, &mut val);
        if val as libc::c_int != agc_params.hpMax as libc::c_int {
            printf(
                b"ERROR: wrong hpMax (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
                agc_params.hpMax as libc::c_int,
                val as libc::c_int,
            );
            return -1i32;
        }
        sx1302_agc_mailbox_read(2i32 as uint8_t, &mut val);
        if val as libc::c_int != agc_params.paDutyCycle as libc::c_int {
            printf(
                b"ERROR: wrong paDutyCycle (w:%u r:%u)\n\x00" as *const u8 as *const libc::c_char,
                agc_params.paDutyCycle as libc::c_int,
                val as libc::c_int,
            );
            return -1i32;
        }
        /* notify AGC that it can resume */
        sx1302_agc_mailbox_write(3i32 as uint8_t, 0xai32 as uint8_t);
    } else {
        /* notify AGC that it can resume */
        sx1302_agc_mailbox_write(3i32 as uint8_t, 0x9i32 as uint8_t);
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_load_firmware(mut firmware: *const uint8_t) -> libc::c_int {
    let mut fw_check: [uint8_t; 8192] = [0; 8192]; /* GPIO output direction */
    let mut gpio_sel: int32_t = 0x2i32;
    let mut val: int32_t = 0;
    lgw_reg_w(282i32 as uint16_t, gpio_sel);
    lgw_reg_w(283i32 as uint16_t, gpio_sel);
    lgw_reg_w(284i32 as uint16_t, gpio_sel);
    lgw_reg_w(285i32 as uint16_t, gpio_sel);
    lgw_reg_w(286i32 as uint16_t, gpio_sel);
    lgw_reg_w(287i32 as uint16_t, gpio_sel);
    lgw_reg_w(288i32 as uint16_t, gpio_sel);
    lgw_reg_w(289i32 as uint16_t, gpio_sel);
    lgw_reg_w(275i32 as uint16_t, 0xffi32);
    /* Take control over ARB MCU */
    lgw_reg_w(743i32 as uint16_t, 0x1i32);
    lgw_reg_w(744i32 as uint16_t, 0x1i32);
    lgw_reg_w(0i32 as uint16_t, 0i32);
    /* Write ARB fw in ARB MEM */
    lgw_mem_wb(
        0x2000i32 as uint16_t,
        &*firmware.offset(0),
        8192i32 as uint16_t,
    );
    /* Read back and check */
    lgw_mem_rb(
        0x2000i32 as uint16_t,
        fw_check.as_mut_ptr(),
        8192i32 as uint16_t,
        0i32 != 0,
    );
    if memcmp(
        firmware as *const libc::c_void,
        fw_check.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8192]>() as libc::c_ulong,
    ) != 0i32
    {
        printf(b"ERROR: Failed to load fw\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    /* Release control over ARB MCU */
    lgw_reg_w(744i32 as uint16_t, 0i32);
    lgw_reg_w(743i32 as uint16_t, 0i32);
    lgw_reg_r(745i32 as uint16_t, &mut val);
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_status(mut status: *mut uint8_t) -> libc::c_int {
    let mut val: int32_t = 0;
    if lgw_reg_r(746i32 as uint16_t, &mut val) != 0i32 {
        printf(b"ERROR: Failed to get AGC status\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    *status = val as uint8_t;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_wait_status(mut status: uint8_t) -> libc::c_int {
    let mut val: uint8_t = 0;
    loop {
        if sx1302_arb_status(&mut val) != 0i32 {
            return -1i32;
        }
        if !(val as libc::c_int != status as libc::c_int) {
            break;
        }
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_debug_read(
    mut reg_id: uint8_t,
    mut value: *mut uint8_t,
) -> libc::c_int {
    let mut reg: uint16_t = 0;
    let mut val: int32_t = 0;
    /* Check parameters */
    if reg_id as libc::c_int > 15i32 {
        printf(b"ERROR: invalid ARB debug register ID\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    reg = (758i32 + reg_id as libc::c_int) as uint16_t;
    if lgw_reg_r(reg, &mut val) != 0i32 {
        printf(
            b"ERROR: failed to read ARB debug register\n\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    *value = val as uint8_t;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_debug_write(
    mut reg_id: uint8_t,
    mut value: uint8_t,
) -> libc::c_int {
    let mut reg: uint16_t = 0;
    /* Check parameters */
    if reg_id as libc::c_int > 3i32 {
        printf(b"ERROR: invalid ARB debug register ID\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    reg = (754i32 + reg_id as libc::c_int) as uint16_t;
    if lgw_reg_w(reg, value as int32_t) != 0i32 {
        printf(
            b"ERROR: failed to write ARB debug register ID\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    return 0i32;
}
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_set_debug_stats(mut enable: bool, mut sf: uint8_t) {
    if enable as libc::c_int == 1i32 {
        lgw_reg_w(754i32 as uint16_t, sf as int32_t);
    };
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_get_debug_stats_detect(mut channel: uint8_t) -> uint8_t {
    let mut dbg_val: int32_t = 0;
    if channel as libc::c_int >= 8i32 {
        printf(
            b"ERROR: wrong configuration, channel num must be < 8\x00" as *const u8
                as *const libc::c_char,
        );
        return 0i32 as uint8_t;
    }
    lgw_reg_r((758i32 + channel as libc::c_int) as uint16_t, &mut dbg_val);
    return dbg_val as uint8_t;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_get_debug_stats_alloc(mut channel: uint8_t) -> uint8_t {
    let mut dbg_val: int32_t = 0;
    if channel as libc::c_int >= 8i32 {
        printf(
            b"ERROR: wrong configuration, channel num must be < 8\x00" as *const u8
                as *const libc::c_char,
        );
        return 0i32 as uint8_t;
    }
    lgw_reg_r((766i32 + channel as libc::c_int) as uint16_t, &mut dbg_val);
    return dbg_val as uint8_t;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_print_debug_stats() {
    let mut i: libc::c_int = 0;
    let mut nb_detect: uint8_t = 0;
    let mut nb_alloc: uint8_t = 0;
    let mut nb_detect_total: libc::c_int = 0i32;
    let mut nb_alloc_total: libc::c_int = 0i32;
    /* Get number of detects for all channels */
    nb_detect_total = 0i32;
    i = 0i32;
    while i < 8i32 {
        nb_detect = sx1302_arb_get_debug_stats_detect(i as uint8_t);
        nb_detect_total += nb_detect as libc::c_int;
        i += 1
    }
    /* Get number of modem allocation for all channels */
    nb_alloc_total = 0i32;
    i = 0i32;
    while i < 8i32 {
        nb_alloc = sx1302_arb_get_debug_stats_alloc(i as uint8_t);
        nb_alloc_total += nb_alloc as libc::c_int;
        i += 1
    }
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_arb_start(mut version: uint8_t) -> libc::c_int {
    let mut val: uint8_t = 0;
    /* Wait for ARB fw to be started, and VERSION available in debug registers */
    sx1302_arb_wait_status(0x1i32 as uint8_t);
    /* Get firmware VERSION */
    sx1302_arb_debug_read(0i32 as uint8_t, &mut val);
    if val as libc::c_int != version as libc::c_int {
        printf(
            b"ERROR: wrong ARB fw version (%d)\n\x00" as *const u8 as *const libc::c_char,
            val as libc::c_int,
        );
        return -1i32;
    }
    /* Enable/disable ARB detect/modem alloc stats for the specified SF */
    sx1302_arb_set_debug_stats(1i32 != 0, 7i32 as uint8_t);
    /* 0:Disable 1:Enable double demod for different timing set (best_timestamp / best_demodulation) - Only available for SF9 -> SF12 */
    sx1302_arb_debug_write(3i32 as uint8_t, 0i32 as uint8_t);
    /* Set double detect packet filtering threshold [0..3] */
    sx1302_arb_debug_write(2i32 as uint8_t, 3i32 as uint8_t);
    /* Notify ARB that it can resume */
    sx1302_arb_debug_write(1i32 as uint8_t, 1i32 as uint8_t);
    /* Wait for ARB to acknoledge */
    sx1302_arb_wait_status(0i32 as uint8_t);
    return 0i32;
}
/* *
@brief Get the number of packets available in rx_buffer and fetch data from ...
@brief ... the SX1302 if rx_buffer is empty.
@param  nb_pkt A pointer to allocated memory to hold the number of packet fetched
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_fetch(mut nb_pkt: *mut uint8_t) -> libc::c_int {
    let mut err: libc::c_int = 0;
    /* Fetch packets from sx1302 if no more left in RX buffer */
    if rx_buffer.buffer_pkt_nb as libc::c_int == 0i32 {
        /* Initialize RX buffer */
        err = rx_buffer_new(&mut rx_buffer);
        if err != 0i32 {
            printf(
                b"ERROR: Failed to initialize RX buffer\n\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
        }
        /* Fetch RX buffer if any data available */
        err = rx_buffer_fetch(&mut rx_buffer);
        if err != 0i32 {
            printf(b"ERROR: Failed to fetch RX buffer\n\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    } else {
        printf(
            b"Note: remaining %u packets in RX buffer, do not fetch sx1302 yet...\n\x00"
                as *const u8 as *const libc::c_char,
            rx_buffer.buffer_pkt_nb as libc::c_int,
        );
    }
    /* Return the number of packet fetched */
    *nb_pkt = rx_buffer.buffer_pkt_nb;
    return 0i32;
}
/* *
@brief Parse and return the next packet available in rx_buffer.
@param context      Gateway configuration context
@param p            The structure to get the packet parsed
@return LGW_REG_SUCCESS if a packet could be parsed, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_parse(
    mut context: *mut lgw_context_t,
    mut p: *mut lgw_pkt_rx_s,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* type of if_chain/modem a packet was received by */
    let mut err: libc::c_int = 0; /* correction to account for processing delay */
    let mut ifmod: libc::c_int = 0;
    let mut payload_crc16_calc: uint16_t = 0;
    let mut cr: uint8_t = 0;
    let mut timestamp_correction: uint32_t = 0;
    let mut pkt: rx_packet_t = rx_packet_t {
        rxbytenb_modem: 0,
        rx_channel_in: 0,
        crc_en: false,
        coding_rate: 0,
        rx_rate_sf: 0,
        modem_id: 0,
        frequency_offset_error: 0,
        payload: [0; 255],
        payload_crc_error: false,
        sync_error: false,
        header_error: false,
        timing_set: false,
        snr_average: 0,
        rssi_chan_avg: 0,
        rssi_signal_avg: 0,
        rssi_chan_max_neg_delta: 0,
        rssi_chan_max_pos_delta: 0,
        rssi_sig_max_neg_delta: 0,
        rssi_sig_max_pos_delta: 0,
        timestamp_cnt: 0,
        rx_crc16_value: 0,
        num_ts_metrics_stored: 0,
        timestamp_avg: [0; 255],
        timestamp_stddev: [0; 255],
        packet_checksum: 0,
    };
    /* Check input params */
    if context.is_null() {
        return -1i32;
    }
    if p.is_null() {
        return -1i32;
    }
    /* FOR DEBUG: Print statistics of number of detects and modem allocations from ARB for configured SF (see sx1302_arb_start()) */
    sx1302_arb_print_debug_stats();
    /* get packet from RX buffer */
    err = rx_buffer_pop(&mut rx_buffer, &mut pkt);
    if err != 0i32 {
        return -1i32;
    }
    /* copy payload to result struct */
    memcpy(
        (*p).payload.as_mut_ptr() as *mut libc::c_void,
        &mut pkt.payload as *mut [uint8_t; 255] as *mut libc::c_void,
        pkt.rxbytenb_modem as libc::c_ulong,
    );
    (*p).size = pkt.rxbytenb_modem as uint16_t;
    /* process metadata */
    (*p).modem_id = pkt.modem_id;
    (*p).if_chain = pkt.rx_channel_in;
    if (*p).if_chain as libc::c_int >= 10i32 {
        return -1i32;
    }
    ifmod = ifmod_config[(*p).if_chain as usize] as libc::c_int;
    (*p).rf_chain = (*context).if_chain_cfg[(*p).if_chain as usize].rf_chain;
    /* Get the frequency for the channel configuration */
    (*p).freq_hz = ((*context).rf_chain_cfg[(*p).rf_chain as usize].freq_hz as int32_t
        + (*context).if_chain_cfg[(*p).if_chain as usize].freq_hz) as uint32_t;
    /* Get signal strength : offset and temperature compensation will be applied later */
    (*p).rssic = pkt.rssi_chan_avg as libc::c_float;
    (*p).rssis = pkt.rssi_signal_avg as libc::c_float;
    /* Get modulation metadata */
    if ifmod == 0x11i32 || ifmod == 0x10i32 {
        (*p).modulation = 0x10i32 as uint8_t;
        /* Get CRC status */
        if pkt.crc_en as libc::c_int != 0
            || (*context).lora_service_cfg.implicit_crc_en as libc::c_int == 1i32
        {
            /* CRC enabled */
            if pkt.payload_crc_error {
                (*p).status = 0x11i32 as uint8_t
            } else {
                (*p).status = 0x10i32 as uint8_t;
                /* Sanity check of the payload CRC */
                if (*p).size as libc::c_int > 0i32 {
                    payload_crc16_calc =
                        sx1302_lora_payload_crc((*p).payload.as_mut_ptr(), (*p).size as uint8_t);
                    if payload_crc16_calc as libc::c_int != pkt.rx_crc16_value as libc::c_int {
                        printf(
                            b"ERROR: Payload CRC16 check failed (got:0x%04X calc:0x%04X)\n\x00"
                                as *const u8 as *const libc::c_char,
                            pkt.rx_crc16_value as libc::c_int,
                            payload_crc16_calc as libc::c_int,
                        );
                        if !log_file.is_null() {
                            fprintf(
                                log_file,
                                b"ERROR: Payload CRC16 check failed (got:0x%04X calc:0x%04X)\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                pkt.rx_crc16_value as libc::c_int,
                                payload_crc16_calc as libc::c_int,
                            );
                            dbg_log_buffer_to_file(
                                log_file,
                                rx_buffer.buffer.as_mut_ptr(),
                                rx_buffer.buffer_size,
                            );
                        }
                        return -1i32;
                    }
                }
            }
        } else {
            /* CRC disabled */
            (*p).status = 0x1i32 as uint8_t
        }
        /* FOR DEBUG: Check data integrity for known devices (debug context) */
        if (*p).status as libc::c_int == 0x10i32 || (*p).status as libc::c_int == 0x1i32 {
            /*  We compare the received payload with predefined ones to ensure that the payload content is what we expect.
                4 bytes: ID to identify the payload
                4 bytes: packet counter used to initialize the seed for pseudo-random generation
                x bytes: pseudo-random payload
            */
            let mut res: libc::c_int = 0;
            i = 0i32;
            while i < (*context).debug_cfg.nb_ref_payload as libc::c_int {
                res = dbg_check_payload(
                    &mut (*context).debug_cfg,
                    log_file,
                    (*p).payload.as_mut_ptr(),
                    (*p).size as uint8_t,
                    i as uint8_t,
                    pkt.rx_rate_sf,
                );
                if res == -1i32 {
                    printf(
                        b"ERROR: 0x%08X payload error\n\x00" as *const u8 as *const libc::c_char,
                        (*context).debug_cfg.ref_payload[i as usize].id,
                    );
                    if !log_file.is_null() {
                        fprintf(
                            log_file,
                            b"ERROR: 0x%08X payload error\n\x00" as *const u8
                                as *const libc::c_char,
                            (*context).debug_cfg.ref_payload[i as usize].id,
                        );
                        dbg_log_buffer_to_file(
                            log_file,
                            rx_buffer.buffer.as_mut_ptr(),
                            rx_buffer.buffer_size,
                        );
                        dbg_log_payload_diff_to_file(
                            log_file,
                            (*p).payload.as_mut_ptr(),
                            (*context).debug_cfg.ref_payload[i as usize]
                                .payload
                                .as_mut_ptr(),
                            (*p).size,
                        );
                    }
                    return -1i32;
                } else {
                    (res) == 1i32;
                }
                i += 1
            }
        }
        /* Get SNR - converted from 0.25dB step to dB */
        (*p).snr = pkt.snr_average as libc::c_float / 4i32 as libc::c_float;
        /* Get bandwidth */
        if ifmod == 0x11i32 {
            (*p).bandwidth = 0x4i32 as uint8_t
        /* fixed in hardware */
        } else {
            (*p).bandwidth = (*context).lora_service_cfg.bandwidth
            /* get the parameter from the config variable */
        }
        /* Get datarate */
        match pkt.rx_rate_sf as libc::c_int {
            5 => (*p).datarate = 5i32 as uint32_t,
            6 => (*p).datarate = 6i32 as uint32_t,
            7 => (*p).datarate = 7i32 as uint32_t,
            8 => (*p).datarate = 8i32 as uint32_t,
            9 => (*p).datarate = 9i32 as uint32_t,
            10 => (*p).datarate = 10i32 as uint32_t,
            11 => (*p).datarate = 11i32 as uint32_t,
            12 => (*p).datarate = 12i32 as uint32_t,
            _ => (*p).datarate = 0i32 as uint32_t,
        }
        /* Get coding rate */
        if ifmod == 0x11i32 || (*context).lora_service_cfg.implicit_hdr as libc::c_int == 0i32 {
            cr = pkt.coding_rate
        } else {
            cr = (*context).lora_service_cfg.implicit_coderate
        }
        match cr as libc::c_int {
            1 => (*p).coderate = 0x1i32 as uint8_t,
            2 => (*p).coderate = 0x2i32 as uint8_t,
            3 => (*p).coderate = 0x3i32 as uint8_t,
            4 => (*p).coderate = 0x4i32 as uint8_t,
            _ => (*p).coderate = 0i32 as uint8_t,
        }
        /* Get frequency offset in Hz depending on bandwidth */
        match (*p).bandwidth as libc::c_int {
            4 => {
                (*p).freq_offset =
                    (pkt.frequency_offset_error as libc::c_float * 0.11920929f32) as int32_t
            }
            5 => {
                (*p).freq_offset =
                    (pkt.frequency_offset_error as libc::c_float * 0.238418579f32) as int32_t
            }
            6 => {
                (*p).freq_offset =
                    (pkt.frequency_offset_error as libc::c_float * 0.476837158f32) as int32_t
            }
            _ => {
                (*p).freq_offset = 0i32;
                printf(b"Invalid frequency offset\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        /* Get timestamp correction to be applied */
        timestamp_correction = timestamp_counter_correction(
            ifmod,
            (*p).bandwidth,
            (*p).datarate as uint8_t,
            (*p).coderate,
            pkt.crc_en as uint32_t,
            pkt.rxbytenb_modem as uint16_t,
        )
    } else if ifmod == 0x20i32 {
        (*p).modulation = 0x20i32 as uint8_t;
        /* Get CRC status */
        if pkt.crc_en {
            /* CRC enabled */
            if pkt.payload_crc_error {
                printf(b"FSK: CRC ERR\n\x00" as *const u8 as *const libc::c_char);
                (*p).status = 0x11i32 as uint8_t
            } else {
                printf(b"FSK: CRC OK\n\x00" as *const u8 as *const libc::c_char);
                (*p).status = 0x10i32 as uint8_t
            }
        } else {
            /* CRC disabled */
            (*p).status = 0x1i32 as uint8_t
        }
        /* Get modulation params */
        (*p).bandwidth = (*context).fsk_cfg.bandwidth;
        (*p).datarate = (*context).fsk_cfg.datarate;
        /* Compute timestamp correction to be applied */
        timestamp_correction = (680000i32 as uint32_t)
            .wrapping_div((*context).fsk_cfg.datarate)
            .wrapping_sub(20i32 as libc::c_uint);
        /* RSSI correction */
        (*p).rssic = ((86i32 as libc::c_float + 1i32 as libc::c_float * (*p).rssic)
            as libc::c_double
            + 0i32 as libc::c_double * pow((*p).rssic as libc::c_double, 2i32 as libc::c_double))
            as libc::c_float;
        /* Undefined for FSK */
        (*p).coderate = 0i32 as uint8_t;
        (*p).snr = -128.0f64 as libc::c_float;
        (*p).rssis = -128.0f64 as libc::c_float
    } else {
        (*p).status = 0i32 as uint8_t;
        (*p).modulation = 0i32 as uint8_t;
        (*p).rssic = -128.0f64 as libc::c_float;
        (*p).rssis = -128.0f64 as libc::c_float;
        (*p).snr = -128.0f64 as libc::c_float;
        (*p).snr_min = -128.0f64 as libc::c_float;
        (*p).snr_max = -128.0f64 as libc::c_float;
        (*p).bandwidth = 0i32 as uint8_t;
        (*p).datarate = 0i32 as uint32_t;
        (*p).coderate = 0i32 as uint8_t;
        timestamp_correction = 0i32 as uint32_t
    }
    /* Scale packet timestamp to 1 MHz (microseconds) */
    (*p).count_us = pkt.timestamp_cnt.wrapping_div(32i32 as libc::c_uint);
    /* Expand 27-bits counter to 32-bits counter, based on current wrapping status */
    (*p).count_us = timestamp_counter_expand(&mut counter_us, 0i32 != 0, (*p).count_us);
    /* Packet timestamp corrected */
    (*p).count_us = (*p).count_us.wrapping_sub(timestamp_correction);
    /* Packet CRC status */
    (*p).crc = pkt.rx_crc16_value;
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_lora_payload_crc(
    mut data: *const uint8_t,
    mut size: uint8_t,
) -> uint16_t {
    let mut i: libc::c_int = 0;
    let mut crc: libc::c_int = 0i32;
    i = 0i32;
    while i < size as libc::c_int {
        lora_crc16(*data.offset(i as isize) as libc::c_char, &mut crc);
        i += 1
    }
    //printf("CRC16: 0x%02X 0x%02X (%X)\n", (uint8_t)(crc >> 8), (uint8_t)crc, crc);
    return crc as uint16_t;
}
/* *
@brief Configure the delay to be applied by the SX1302 for TX to start
@param rf_chain     The RF chain index to be configured
@param radio_type   The type of radio for this RF chain
@param modulation   The modulation used for the TX
@param bandwidth    The bandwidth used for the TX
@param delay        The TX start delay calculated and applied
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_tx_set_start_delay(
    mut rf_chain: uint8_t,
    mut radio_type: lgw_radio_type_t,
    mut modulation: uint8_t,
    mut bandwidth: uint8_t,
    mut delay: *mut uint16_t,
) -> libc::c_int {
    let mut tx_start_delay: uint16_t = (1500i32 * 32i32) as uint16_t;
    let mut radio_bw_delay: uint16_t = 0i32 as uint16_t;
    let mut filter_delay: uint16_t = 0i32 as uint16_t;
    let mut modem_delay: uint16_t = 0i32 as uint16_t;
    let mut bw_hz: int32_t = lgw_bw_getval(bandwidth as libc::c_int);
    let mut val: int32_t = 0;
    let mut chirp_low_pass: uint8_t = 0i32 as uint8_t;
    if delay.is_null() {
        return -1i32;
    }
    /* Adjust with radio type and bandwidth */
    match radio_type as libc::c_uint {
        5 => {
            if bandwidth as libc::c_int == 0x4i32 {
                radio_bw_delay = 19i32 as uint16_t
            } else if bandwidth as libc::c_int == 0x5i32 {
                radio_bw_delay = 24i32 as uint16_t
            } else if bandwidth as libc::c_int == 0x6i32 {
                radio_bw_delay = 21i32 as uint16_t
            } else {
                return -1i32;
            }
        }
        1 | 2 => {
            radio_bw_delay = (3i32 * 32i32 + 4i32) as uint16_t;
            if bandwidth as libc::c_int == 0x4i32 {
                radio_bw_delay = (radio_bw_delay as libc::c_int + 0i32) as uint16_t
            } else if bandwidth as libc::c_int == 0x5i32 {
                radio_bw_delay = (radio_bw_delay as libc::c_int + 6i32) as uint16_t
            } else if bandwidth as libc::c_int == 0x6i32 {
                radio_bw_delay = (radio_bw_delay as libc::c_int + 0i32) as uint16_t
            } else {
                return -1i32;
            }
        }
        _ => return -1i32,
    }
    /* Adjust with modulation */
    if modulation as libc::c_int == 0x10i32 {
        lgw_reg_r(
            if 0i32 == 0i32 { 145i32 } else { 253i32 } as uint16_t,
            &mut val,
        );
        chirp_low_pass = val as uint8_t;
        filter_delay = (((1i32 << chirp_low_pass as libc::c_int) - 1i32) as libc::c_double * 1e6f64
            / bw_hz as libc::c_double) as uint16_t;
        modem_delay =
            (8i32 as libc::c_double * (32e6f64 / (32i32 * bw_hz) as libc::c_double)) as uint16_t
    /* if bw=125k then modem freq=4MHz */
    } else {
        /* TODO */
        filter_delay = 0i32 as uint16_t;
        modem_delay = 0i32 as uint16_t
    }
    /* Compute total delay */
    tx_start_delay = (tx_start_delay as libc::c_int
        - (radio_bw_delay as libc::c_int
            + filter_delay as libc::c_int
            + modem_delay as libc::c_int)) as uint16_t;
    /* Configure the SX1302 with the calculated delay */
    lgw_reg_w(
        if rf_chain as libc::c_int == 0i32 {
            66i32
        } else {
            174i32
        } as uint16_t,
        (tx_start_delay as libc::c_int >> 8i32) as uint8_t as int32_t,
    );
    lgw_reg_w(
        if rf_chain as libc::c_int == 0i32 {
            67i32
        } else {
            175i32
        } as uint16_t,
        (tx_start_delay as libc::c_int >> 0i32) as uint8_t as int32_t,
    );
    /* return tx_start_delay */
    *delay = tx_start_delay;
    return 0i32;
}
/* *
@brief Compute the offset to be applied on RSSI for temperature compensation
@param context  a pointer to the memory that holds the current temp comp context
@param temperature  the temperature for which to compute the offset to be applied
@return the offset to be applied to RSSI
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_rssi_get_temperature_offset(
    mut context: *mut lgw_rssi_tcomp_s,
    mut temperature: libc::c_float,
) -> libc::c_float {
    /* Chekc params */
    if context.is_null() {
        return -1i32 as libc::c_float;
    }
    /* Compute the offset to be applied to RSSI for given temperature */
    return (((*context).coeff_a as libc::c_double
        * pow(temperature as libc::c_double, 4i32 as libc::c_double)
        + (*context).coeff_b as libc::c_double
            * pow(temperature as libc::c_double, 3i32 as libc::c_double)
        + (*context).coeff_c as libc::c_double
            * pow(temperature as libc::c_double, 2i32 as libc::c_double)
        + ((*context).coeff_d * temperature) as libc::c_double
        + (*context).coeff_e as libc::c_double)
        / pow(2i32 as libc::c_double, 16i32 as libc::c_double)) as libc::c_float;
}
/* *
@brief Get current TX status of the SX1302
@param rf_chain the TX chain we want to get the status from
@return current status
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_tx_status(mut rf_chain: uint8_t) -> uint8_t {
    let mut err: libc::c_int = 0;
    let mut read_value: int32_t = 0;
    err = lgw_reg_r(
        if rf_chain as libc::c_int == 0i32 {
            80i32
        } else {
            188i32
        } as uint16_t,
        &mut read_value,
    );
    if err != 0i32 {
        printf(b"ERROR: Failed to read TX STATUS\x00" as *const u8 as *const libc::c_char);
        return 0i32 as uint8_t;
    }
    if read_value == 0x80i32 {
        return 2i32 as uint8_t;
    } else if read_value == 0x30i32
        || read_value == 0x50i32
        || read_value == 0x60i32
        || read_value == 0x70i32
    {
        return 4i32 as uint8_t;
    } else if read_value == 0x91i32 || read_value == 0x92i32 {
        return 3i32 as uint8_t;
    } else {
        printf(
            b"ERROR: UNKNOWN TX STATUS 0x%02X\n\x00" as *const u8 as *const libc::c_char,
            read_value,
        );
        return 0i32 as uint8_t;
    };
}
/* *
@brief Get current RX status of the SX1302
@param rf_chain the RX chain we want to get the status from
@return current status
@note NOT IMPLEMENTED
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_rx_status(mut rf_chain: uint8_t) -> uint8_t {
    (rf_chain) != 0;
    /* dummy for compilation */
    /* Not implemented */
    return 0i32 as uint8_t;
}
/* *
@brief Abort current transmit
@param rf_chain the TX chain on which we want to abort transmit
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_tx_abort(mut rf_chain: uint8_t) -> libc::c_int {
    lgw_reg_w(
        if rf_chain as libc::c_int == 0i32 {
            61i32
        } else {
            169i32
        } as uint16_t,
        0i32,
    );
    lgw_reg_w(
        if rf_chain as libc::c_int == 0i32 {
            60i32
        } else {
            168i32
        } as uint16_t,
        0i32,
    );
    lgw_reg_w(
        if rf_chain as libc::c_int == 0i32 {
            59i32
        } else {
            167i32
        } as uint16_t,
        0i32,
    );
    loop {
        wait_ms(1i32 as libc::c_ulong);
        if !(sx1302_tx_status(rf_chain) as libc::c_int != 2i32) {
            break;
        }
    }
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_tx_configure(mut radio_type: lgw_radio_type_t) -> libc::c_int {
    /* Select the TX destination interface */
    match radio_type as libc::c_uint {
        5 => {
            /* Let AGC control PLL DIV (sx1250 only) */
            lgw_reg_w(88i32 as uint16_t, 1i32);
            lgw_reg_w(196i32 as uint16_t, 1i32);
            /* SX126x Tx RFFE */
            lgw_reg_w(85i32 as uint16_t, 0x1i32);
            lgw_reg_w(193i32 as uint16_t, 0x1i32);
        }
        2 => {
            /* SX1255/57 Tx RFFE */
            lgw_reg_w(85i32 as uint16_t, 0i32);
            lgw_reg_w(193i32 as uint16_t, 0i32);
        }
        _ => return -1i32,
    }
    /* Configure the TX mode of operation */
    lgw_reg_w(84i32 as uint16_t, 0x1i32); /* Modulation */
    lgw_reg_w(192i32 as uint16_t, 0x1i32); /* Modulation */
    /* Configure the output data clock edge */
    lgw_reg_w(83i32 as uint16_t, 0i32); /* Data on rising edge */
    lgw_reg_w(191i32 as uint16_t, 0i32); /* Data on rising edge */
    return 0i32;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn sx1302_send(
    mut radio_type: lgw_radio_type_t,
    mut tx_lut: *mut lgw_tx_gain_lut_s,
    mut lwan_public: bool,
    mut context_fsk: *mut lgw_conf_rxif_s,
    mut pkt_data: *mut lgw_pkt_tx_s,
) -> libc::c_int {
    let mut freq_reg: uint32_t = 0;
    let mut fdev_reg: uint32_t = 0;
    let mut freq_dev: uint32_t = 0;
    let mut fsk_br_reg: uint32_t = 0;
    let mut fsk_sync_word_reg: uint64_t = 0;
    let mut mem_addr: uint16_t = 0;
    let mut count_us: uint32_t = 0;
    let mut power: uint8_t = 0;
    let mut pow_index: uint8_t = 0;
    let mut mod_bw: uint8_t = 0;
    let mut pa_en: uint8_t = 0;
    let mut tx_start_delay: uint16_t = 0;
    /* CHeck input parameters */
    if tx_lut.is_null() {
        return -1i32;
    }
    if pkt_data.is_null() {
        return -1i32;
    }
    /* Select the proper modem */
    match (*pkt_data).modulation as libc::c_int {
        8 => {
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    70i32
                } else {
                    178i32
                } as uint16_t,
                0i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    86i32
                } else {
                    194i32
                } as uint16_t,
                0i32,
            );
        }
        16 => {
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    70i32
                } else {
                    178i32
                } as uint16_t,
                0i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    86i32
                } else {
                    194i32
                } as uint16_t,
                0x1i32,
            );
        }
        32 => {
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    70i32
                } else {
                    178i32
                } as uint16_t,
                0x1i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    86i32
                } else {
                    194i32
                } as uint16_t,
                0x2i32,
            );
        }
        _ => return -1i32,
    }
    /* Find the proper index in the TX gain LUT according to requested rf_power */
    pow_index = ((*tx_lut).size as libc::c_int - 1i32) as uint8_t;
    while pow_index as libc::c_int > 0i32 {
        if (*tx_lut).lut[pow_index as usize].rf_power as libc::c_int
            <= (*pkt_data).rf_power as libc::c_int
        {
            break;
        }
        pow_index = pow_index.wrapping_sub(1)
    }
    /* loading calibrated Tx DC offsets */
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            90i32
        } else {
            198i32
        } as uint16_t,
        (*tx_lut).lut[pow_index as usize].offset_i as int32_t,
    );
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            91i32
        } else {
            199i32
        } as uint16_t,
        (*tx_lut).lut[pow_index as usize].offset_q as int32_t,
    );
    /* Set the power parameters to be used for TX */
    match radio_type as libc::c_uint {
        5 => {
            pa_en = if (*tx_lut).lut[pow_index as usize].pa_gain as libc::c_int > 0i32 {
                1i32
            } else {
                0i32
            } as uint8_t; /* only 1 bit used to control the external PA */
            power = ((pa_en as libc::c_int) << 6i32
                | (*tx_lut).lut[pow_index as usize].pwr_idx as libc::c_int)
                as uint8_t
        }
        2 => {
            power = (((*tx_lut).lut[pow_index as usize].pa_gain as libc::c_int) << 6i32
                | ((*tx_lut).lut[pow_index as usize].dac_gain as libc::c_int) << 4i32
                | (*tx_lut).lut[pow_index as usize].mix_gain as libc::c_int)
                as uint8_t
        }
        _ => return -1i32,
    }
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            76i32
        } else {
            184i32
        } as uint16_t,
        power as int32_t,
    );
    /* Set digital gain */
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            89i32
        } else {
            197i32
        } as uint16_t,
        (*tx_lut).lut[pow_index as usize].dig_gain as int32_t,
    );
    /* Set Tx frequency */
    freq_reg = ((*pkt_data).freq_hz as uint64_t)
        .wrapping_mul((1i32 << 18i32) as libc::c_ulong)
        .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t; /* TODO: AGC fw to be updated for sx1255 */
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            92i32
        } else {
            200i32
        } as uint16_t,
        (freq_reg >> 16i32 & 0xffi32 as libc::c_uint) as int32_t,
    );
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            93i32
        } else {
            201i32
        } as uint16_t,
        (freq_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
    );
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            94i32
        } else {
            202i32
        } as uint16_t,
        (freq_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
    );
    /* Set AGC bandwidth and modulation type*/
    match (*pkt_data).modulation as libc::c_int {
        16 => mod_bw = (*pkt_data).bandwidth,
        8 | 32 => mod_bw = (0x1i32 << 7i32 | (*pkt_data).bandwidth as libc::c_int) as uint8_t,
        _ => {
            printf(b"ERROR: Modulation not supported\n\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    }
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            75i32
        } else {
            183i32
        } as uint16_t,
        mod_bw as int32_t,
    );
    /* Configure modem */
    match (*pkt_data).modulation as libc::c_int {
        8 => {
            /* Set frequency deviation */
            freq_dev = (ceil(fabs(
                ((*pkt_data).freq_offset as libc::c_float / 10i32 as libc::c_float)
                    as libc::c_double,
            )) * 10e3f64) as uint32_t;
            printf(
                b"CW: f_dev %d Hz\n\x00" as *const u8 as *const libc::c_char,
                freq_dev as libc::c_int,
            );
            fdev_reg = (freq_dev as uint64_t)
                .wrapping_mul((1i32 << 18i32) as libc::c_ulong)
                .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t;
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    95i32
                } else {
                    203i32
                } as uint16_t,
                (fdev_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    96i32
                } else {
                    204i32
                } as uint16_t,
                (fdev_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            /* Send frequency deviation to AGC fw for radio config */
            fdev_reg = (freq_dev as uint64_t)
                .wrapping_mul((1i32 << 25i32) as libc::c_ulong)
                .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t; /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                46i32 as uint16_t,
                (fdev_reg >> 16i32 & 0xffi32 as libc::c_uint) as int32_t,
            ); /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                47i32 as uint16_t,
                (fdev_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
            ); /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                48i32 as uint16_t,
                (fdev_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            /* Set the frequency offset (ratio of the frequency deviation)*/
            printf(
                b"CW: IF test mod freq %d\n\x00" as *const u8 as *const libc::c_char,
                ((*pkt_data).freq_offset as libc::c_float as libc::c_double
                    * 1e3f64
                    * 64i32 as libc::c_double
                    / freq_dev as libc::c_float as libc::c_double) as libc::c_int,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    97i32
                } else {
                    205i32
                } as uint16_t,
                ((*pkt_data).freq_offset as libc::c_float as libc::c_double
                    * 1e3f64
                    * 64i32 as libc::c_double
                    / freq_dev as libc::c_float as libc::c_double) as libc::c_int,
            );
        }
        16 => {
            /* Set bandwidth */
            freq_dev = (lgw_bw_getval((*pkt_data).bandwidth as libc::c_int) / 2i32) as uint32_t;
            fdev_reg = (freq_dev as uint64_t)
                .wrapping_mul((1i32 << 18i32) as libc::c_ulong)
                .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t;
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    95i32
                } else {
                    203i32
                } as uint16_t,
                (fdev_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    96i32
                } else {
                    204i32
                } as uint16_t,
                (fdev_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    123i32
                } else {
                    231i32
                } as uint16_t,
                (*pkt_data).bandwidth as int32_t,
            );
            /* Preamble length */
            if (*pkt_data).preamble as libc::c_int == 0i32 {
                /* if not explicit, use recommended LoRa preamble size */
                (*pkt_data).preamble = 8i32 as uint16_t
            } else if ((*pkt_data).preamble as libc::c_int) < 6i32 {
                /* enforce minimum preamble size */
                (*pkt_data).preamble = 6i32 as uint16_t
            } /* MSB */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    141i32
                } else {
                    249i32
                } as uint16_t,
                (*pkt_data).preamble as libc::c_int >> 8i32 & 0xffi32,
            ); /* LSB */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    140i32
                } else {
                    248i32
                } as uint16_t,
                (*pkt_data).preamble as libc::c_int >> 0i32 & 0xffi32,
            );
            /* LoRa datarate */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    124i32
                } else {
                    232i32
                } as uint16_t,
                (*pkt_data).datarate as int32_t,
            );
            if (*pkt_data).datarate < 10i32 as libc::c_uint {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        145i32
                    } else {
                        253i32
                    } as uint16_t,
                    6i32,
                );
            /* less filtering for low SF : TBC */
            } else {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        145i32
                    } else {
                        253i32
                    } as uint16_t,
                    7i32,
                );
            }
            /* Coding Rate */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    128i32
                } else {
                    236i32
                } as uint16_t,
                (*pkt_data).coderate as int32_t,
            );
            /* Start LoRa modem */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    130i32
                } else {
                    238i32
                } as uint16_t,
                1i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    131i32
                } else {
                    239i32
                } as uint16_t,
                2i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    137i32
                } else {
                    245i32
                } as uint16_t,
                1i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    149i32
                } else {
                    257i32
                } as uint16_t,
                0i32,
            );
            /* Modulation options */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    148i32
                } else {
                    256i32
                } as uint16_t,
                if (*pkt_data).invert_pol as libc::c_int != 0 {
                    1i32
                } else {
                    0i32
                },
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    132i32
                } else {
                    240i32
                } as uint16_t,
                if (*pkt_data).no_header as libc::c_int != 0 {
                    1i32
                } else {
                    0i32
                },
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    133i32
                } else {
                    241i32
                } as uint16_t,
                if (*pkt_data).no_crc as libc::c_int != 0 {
                    0i32
                } else {
                    1i32
                },
            );
            /* Syncword */
            if lwan_public as libc::c_int == 0i32
                || (*pkt_data).datarate == 5i32 as libc::c_uint
                || (*pkt_data).datarate == 6i32 as libc::c_uint
            {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        157i32
                    } else {
                        265i32
                    } as uint16_t,
                    2i32,
                );
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        160i32
                    } else {
                        268i32
                    } as uint16_t,
                    4i32,
                );
            } else {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        157i32
                    } else {
                        265i32
                    } as uint16_t,
                    6i32,
                );
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        160i32
                    } else {
                        268i32
                    } as uint16_t,
                    8i32,
                );
            }
            /* Set Fine Sync for SF5/SF6 */
            if (*pkt_data).datarate == 5i32 as libc::c_uint
                || (*pkt_data).datarate == 6i32 as libc::c_uint
            {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        129i32
                    } else {
                        237i32
                    } as uint16_t,
                    1i32,
                );
            } else {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        129i32
                    } else {
                        237i32
                    } as uint16_t,
                    0i32,
                );
            }
            /* Set Payload length */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    134i32
                } else {
                    242i32
                } as uint16_t,
                (*pkt_data).size as int32_t,
            );
            /* Set PPM offset (low datarate optimization) */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    125i32
                } else {
                    233i32
                } as uint16_t,
                0i32,
            );
            if (*pkt_data).bandwidth as libc::c_int == 0x4i32
                && ((*pkt_data).datarate == 11i32 as libc::c_uint
                    || (*pkt_data).datarate == 12i32 as libc::c_uint)
                || (*pkt_data).bandwidth as libc::c_int == 0x5i32
                    && (*pkt_data).datarate == 12i32 as libc::c_uint
            {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        126i32
                    } else {
                        234i32
                    } as uint16_t,
                    1i32,
                );
            } else {
                lgw_reg_w(
                    if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                        126i32
                    } else {
                        234i32
                    } as uint16_t,
                    0i32,
                );
            }
        }
        32 => {
            if context_fsk.is_null() {
                return -1i32;
            }
            /* Set frequency deviation */
            freq_dev = ((*pkt_data).f_dev as libc::c_int as libc::c_double * 1e3f64) as uint32_t;
            fdev_reg = (freq_dev as uint64_t)
                .wrapping_mul((1i32 << 18i32) as libc::c_ulong)
                .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t;
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    95i32
                } else {
                    203i32
                } as uint16_t,
                (fdev_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    96i32
                } else {
                    204i32
                } as uint16_t,
                (fdev_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            /* Send frequency deviation to AGC fw for radio config */
            fdev_reg = (freq_dev as uint64_t)
                .wrapping_mul((1i32 << 25i32) as libc::c_ulong)
                .wrapping_div(32000000u32 as libc::c_ulong) as uint32_t; /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                46i32 as uint16_t,
                (fdev_reg >> 16i32 & 0xffi32 as libc::c_uint) as int32_t,
            ); /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                47i32 as uint16_t,
                (fdev_reg >> 8i32 & 0xffi32 as libc::c_uint) as int32_t,
            ); /* Needed by AGC to configure the sx1250 */
            lgw_reg_w(
                48i32 as uint16_t,
                (fdev_reg >> 0i32 & 0xffi32 as libc::c_uint) as int32_t,
            );
            /* Modulation parameters */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    104i32
                } else {
                    212i32
                } as uint16_t,
                1i32,
            ); /* Variable length */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    103i32
                } else {
                    211i32
                } as uint16_t,
                if (*pkt_data).no_crc as libc::c_int != 0 {
                    0i32
                } else {
                    1i32
                },
            ); /* CCITT CRC */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    101i32
                } else {
                    209i32
                } as uint16_t,
                0i32,
            ); /* Whitening Encoding */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    102i32
                } else {
                    210i32
                } as uint16_t,
                2i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    113i32
                } else {
                    221i32
                } as uint16_t,
                1i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    112i32
                } else {
                    220i32
                } as uint16_t,
                2i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    111i32
                } else {
                    219i32
                } as uint16_t,
                1i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    109i32
                } else {
                    217i32
                } as uint16_t,
                (*context_fsk).sync_word_size as libc::c_int - 1i32,
            );
            /* Syncword */
            fsk_sync_word_reg = (*context_fsk).sync_word
                << 8i32 * (8i32 - (*context_fsk).sync_word_size as libc::c_int);
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    121i32
                } else {
                    229i32
                } as uint16_t,
                (fsk_sync_word_reg >> 0i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    120i32
                } else {
                    228i32
                } as uint16_t,
                (fsk_sync_word_reg >> 8i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    119i32
                } else {
                    227i32
                } as uint16_t,
                (fsk_sync_word_reg >> 16i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    118i32
                } else {
                    226i32
                } as uint16_t,
                (fsk_sync_word_reg >> 24i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    117i32
                } else {
                    225i32
                } as uint16_t,
                (fsk_sync_word_reg >> 32i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    116i32
                } else {
                    224i32
                } as uint16_t,
                (fsk_sync_word_reg >> 40i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    115i32
                } else {
                    223i32
                } as uint16_t,
                (fsk_sync_word_reg >> 48i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    114i32
                } else {
                    222i32
                } as uint16_t,
                (fsk_sync_word_reg >> 56i32) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    110i32
                } else {
                    218i32
                } as uint16_t,
                0i32,
            );
            /* Set datarate */
            fsk_br_reg = (32000000i32 as libc::c_uint).wrapping_div((*pkt_data).datarate);
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    107i32
                } else {
                    215i32
                } as uint16_t,
                (fsk_br_reg >> 8i32) as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    108i32
                } else {
                    216i32
                } as uint16_t,
                (fsk_br_reg >> 0i32) as int32_t,
            );
            /* Preamble length */
            if (*pkt_data).preamble as libc::c_int == 0i32 {
                /* if not explicit, use LoRaWAN preamble size */
                (*pkt_data).preamble = 5i32 as uint16_t
            } else if ((*pkt_data).preamble as libc::c_int) < 3i32 {
                /* enforce minimum preamble size */
                (*pkt_data).preamble = 3i32 as uint16_t
            } /* MSB */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    105i32
                } else {
                    213i32
                } as uint16_t,
                (*pkt_data).preamble as libc::c_int >> 8i32 & 0xffi32,
            ); /* LSB */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    106i32
                } else {
                    214i32
                } as uint16_t,
                (*pkt_data).preamble as libc::c_int >> 0i32 & 0xffi32,
            );
            /* Set Payload length */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    99i32
                } else {
                    207i32
                } as uint16_t,
                (*pkt_data).size as int32_t,
            );
        }
        _ => {
            printf(b"ERROR: Modulation not supported\n\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    }
    /* Set TX start delay */
    sx1302_tx_set_start_delay(
        (*pkt_data).rf_chain,
        radio_type,
        (*pkt_data).modulation,
        (*pkt_data).bandwidth,
        &mut tx_start_delay,
    );
    /* Write payload in transmit buffer */
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            68i32
        } else {
            176i32
        } as uint16_t,
        0x1i32,
    ); /* insert payload size in the packet for FSK variable mode (1 byte) */
    mem_addr = if (*pkt_data).rf_chain as libc::c_int == 0i32 {
        0x5300i32
    } else {
        0x5500i32
    } as uint16_t;
    if (*pkt_data).modulation as libc::c_int == 0x20i32 {
        lgw_mem_wb(
            mem_addr,
            &mut (*pkt_data).size as *mut uint16_t as *mut uint8_t,
            1i32 as uint16_t,
        );
        lgw_mem_wb(
            (mem_addr as libc::c_int + 1i32) as uint16_t,
            &mut *(*pkt_data).payload.as_mut_ptr().offset(0),
            (*pkt_data).size,
        );
    } else {
        lgw_mem_wb(
            mem_addr,
            &mut *(*pkt_data).payload.as_mut_ptr().offset(0),
            (*pkt_data).size,
        );
    }
    lgw_reg_w(
        if (*pkt_data).rf_chain as libc::c_int == 0i32 {
            68i32
        } else {
            176i32
        } as uint16_t,
        0i32,
    );
    /* Trigger transmit */
    match (*pkt_data).tx_mode as libc::c_int {
        0 => {
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    61i32
                } else {
                    169i32
                } as uint16_t,
                0i32,
            ); /* reset state machine */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    61i32
                } else {
                    169i32
                } as uint16_t,
                0x1i32,
            ); /* reset state machine */
        }
        1 => {
            count_us = (*pkt_data)
                .count_us
                .wrapping_mul(32i32 as libc::c_uint)
                .wrapping_sub(tx_start_delay as libc::c_uint); /* reset state machine */
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    65i32
                } else {
                    173i32
                } as uint16_t,
                (count_us >> 0i32 & 0xffi32 as libc::c_uint) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    64i32
                } else {
                    172i32
                } as uint16_t,
                (count_us >> 8i32 & 0xffi32 as libc::c_uint) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    63i32
                } else {
                    171i32
                } as uint16_t,
                (count_us >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    62i32
                } else {
                    170i32
                } as uint16_t,
                (count_us >> 24i32 & 0xffi32 as libc::c_uint) as uint8_t as int32_t,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    60i32
                } else {
                    168i32
                } as uint16_t,
                0i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    60i32
                } else {
                    168i32
                } as uint16_t,
                0x1i32,
            );
        }
        2 => {
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    59i32
                } else {
                    167i32
                } as uint16_t,
                0i32,
            );
            lgw_reg_w(
                if (*pkt_data).rf_chain as libc::c_int == 0i32 {
                    59i32
                } else {
                    167i32
                } as uint16_t,
                0x1i32,
            );
        }
        _ => {
            printf(b"ERROR: TX mode not supported\n\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    }
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
