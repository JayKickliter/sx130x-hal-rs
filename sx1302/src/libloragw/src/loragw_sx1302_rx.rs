use libc;
extern "C" {
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    /* *
    @brief LoRa concentrator register write
    @param register_id register number in the data structure describing registers
    @param reg_value signed value to write to the register (for u32, use cast)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_w(register_id: uint16_t, reg_value: int32_t) -> libc::c_int;
    /* *
    @brief LoRa concentrator register burst read
    @param register_id register number in the data structure describing registers
    @param data pointer to byte array that will be written from the LoRa concentrator
    @param size size of the transfer, in byte(s)
    @return status of register operation (LGW_REG_SUCCESS/LGW_REG_ERROR)
    */
    #[no_mangle]
    fn lgw_reg_rb(register_id: uint16_t, data: *mut uint8_t, size: uint16_t) -> libc::c_int;
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
    fn lgw_reg_r(register_id: uint16_t, reg_value: *mut int32_t) -> libc::c_int;
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
pub struct rx_buffer_s {
    pub buffer: [uint8_t; 4096],
    pub buffer_size: uint16_t,
    pub buffer_index: libc::c_int,
    pub buffer_pkt_nb: uint8_t,
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
/* *
@struct rx_buffer_s
@brief buffer to hold the data fetched from the sx1302 RX buffer
*/
pub type rx_buffer_t = rx_buffer_s;
/* !> byte array to hald the data fetched from the RX buffer */
/* !> The number of bytes currently stored in the buffer */
/* !> Current parsing index in the buffer */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Initialize the rx_buffer instance
@param self     A pointer to a rx_buffer handler
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
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
pub unsafe extern "C" fn rx_buffer_new(mut self_0: *mut rx_buffer_t) -> libc::c_int {
    /* Check input params */
    if self_0.is_null() {
        return -1i32;
    }
    /* Initialize members */
    memset(
        (*self_0).buffer.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[uint8_t; 4096]>() as libc::c_ulong,
    );
    (*self_0).buffer_size = 0i32 as uint16_t;
    (*self_0).buffer_index = 0i32;
    (*self_0).buffer_pkt_nb = 0i32 as uint8_t;
    return 0i32;
}
/* *
@brief Reset the rx_buffer instance
@param self     A pointer to a rx_buffer handler
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_del(mut self_0: *mut rx_buffer_t) -> libc::c_int {
    /* Check input params */
    if self_0.is_null() {
        return -1i32;
    }
    /* Reset index & size */
    (*self_0).buffer_size = 0i32 as uint16_t;
    (*self_0).buffer_index = 0i32;
    (*self_0).buffer_pkt_nb = 0i32 as uint8_t;
    return 0i32;
}
/* *
@brief Fetch packets from the SX1302 internal RX buffer, and count packets available.
@param self     A pointer to a rx_buffer handler
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_fetch(mut self_0: *mut rx_buffer_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut buff: [uint8_t; 2] = [0; 2];
    /* Check input params */
    if self_0.is_null() {
        return -1i32;
    }
    /* Check if there is data in the FIFO */
    lgw_reg_rb(
        720i32 as uint16_t,
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong as uint16_t,
    );
    (*self_0).buffer_size = ((buff[0] as libc::c_int) << 8i32 & 0xff00i32) as uint16_t;
    (*self_0).buffer_size = ((*self_0).buffer_size as libc::c_int
        | ((buff[1] as libc::c_int) << 0i32 & 0xffi32) as uint16_t as libc::c_int)
        as uint16_t;
    /* Fetch bytes from fifo if any */
    if (*self_0).buffer_size as libc::c_int > 0i32 {
        memset(
            (*self_0).buffer.as_mut_ptr() as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<[uint8_t; 4096]>() as libc::c_ulong,
        );
        res = lgw_mem_rb(
            0x4000i32 as uint16_t,
            (*self_0).buffer.as_mut_ptr(),
            (*self_0).buffer_size,
            1i32 != 0,
        );
        if res != 0i32 {
            printf(
                b"ERROR: Failed to read RX buffer, SPI error\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return -1i32;
        }
        /* print debug info : TODO to be removed */
        i = 0i32;
        while i < (*self_0).buffer_size as libc::c_int {
            i += 1
        }
    }
    /* Parse buffer to get number of packet fetched */
    let mut payload_len: uint8_t = 0;
    let mut next_pkt_idx: uint16_t = 0;
    let mut idx: libc::c_int = 0i32;
    while idx < (*self_0).buffer_size as libc::c_int {
        if (*self_0).buffer[idx as usize] as libc::c_int != 0xa5i32
            || (*self_0).buffer[(idx + 1i32) as usize] as libc::c_int != 0xc0i32
        {
            printf(
                b"ERROR: syncword not found in rx_buffer\n\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
        }
        /* One packet found in the buffer */
        (*self_0).buffer_pkt_nb = ((*self_0).buffer_pkt_nb as libc::c_int + 1i32) as uint8_t;
        /* Compute the number of bytes for thsi packet */
        payload_len = ((*self_0).buffer[(idx + 2i32) as usize] as libc::c_int >> 0i32
            & (1i32 << 8i32) - 1i32) as uint8_t;
        next_pkt_idx = (9i32
            + payload_len as libc::c_int
            + 14i32
            + 2i32
                * ((*self_0).buffer[(idx + payload_len as libc::c_int + 21i32) as usize]
                    as libc::c_int
                    >> 0i32
                    & (1i32 << 8i32) - 1i32)) as uint16_t;
        /* Move to next packet */
        idx += next_pkt_idx as libc::c_int
    }
    /* Initialize the current buffer index to iterate on */
    (*self_0).buffer_index = 0i32;
    return 0i32;
}
/* *
@brief Parse the rx_buffer and return the first packet available in the given structure.
@param self     A pointer to a rx_buffer handler
@param pkt      A pointer to the structure to receive the packet parsed
@return LGW_REG_SUCCESS if success, LGW_REG_ERROR otherwise
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_pop(
    mut self_0: *mut rx_buffer_t,
    mut pkt: *mut rx_packet_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut checksum_rcv: uint8_t = 0;
    let mut checksum_calc: uint8_t = 0i32 as uint8_t;
    let mut checksum_idx: uint16_t = 0;
    let mut pkt_num_bytes: uint16_t = 0;
    /* Check input params */
    if self_0.is_null() {
        return -1i32;
    }
    if pkt.is_null() {
        return -1i32;
    }
    /* Is there any data to be parsed ? */
    if (*self_0).buffer_index >= (*self_0).buffer_size as libc::c_int {
        return -1i32;
    }
    /* Get pkt sync words */
    if (*self_0).buffer[(*self_0).buffer_index as usize] as libc::c_int != 0xa5i32
        || (*self_0).buffer[((*self_0).buffer_index + 1i32) as usize] as libc::c_int != 0xc0i32
    {
        printf(b"INFO: searching syncword...\n\x00" as *const u8 as *const libc::c_char);
        (*self_0).buffer_index += 1i32;
        return -1i32;
        /* TODO: while loop until syncword found ?? */
    }
    /* Get payload length */
    (*pkt).rxbytenb_modem =
        ((*self_0).buffer[((*self_0).buffer_index + 2i32) as usize] as libc::c_int >> 0i32
            & (1i32 << 8i32) - 1i32) as uint8_t;
    /* Get fine timestamp metrics */
    (*pkt).num_ts_metrics_stored = ((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 21i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32) as uint8_t;
    /* Calculate the total number of bytes in the packet */
    pkt_num_bytes = (9i32
        + (*pkt).rxbytenb_modem as libc::c_int
        + 14i32
        + 2i32 * (*pkt).num_ts_metrics_stored as libc::c_int) as uint16_t;
    /* Check if we have a complete packet in the rx buffer fetched */
    if (*self_0).buffer_index + pkt_num_bytes as libc::c_int > (*self_0).buffer_size as libc::c_int
    {
        printf(
            b"WARNING: aborting truncated message (size=%u)\n\x00" as *const u8
                as *const libc::c_char,
            (*self_0).buffer_size as libc::c_int,
        );
        return -1i32;
    }
    /* Get the checksum as received in the RX buffer */
    checksum_idx = (pkt_num_bytes as libc::c_int - 1i32) as uint16_t;
    checksum_rcv =
        (*self_0).buffer[((*self_0).buffer_index + pkt_num_bytes as libc::c_int - 1i32) as usize];
    /* Calculate the checksum from the actual payload bytes received */
    i = 0i32;
    while i < checksum_idx as libc::c_int {
        checksum_calc = (checksum_calc as libc::c_int
            + (*self_0).buffer[((*self_0).buffer_index + i) as usize] as libc::c_int)
            as uint8_t;
        i += 1
    }
    /* Check if the checksum is correct */
    if checksum_rcv as libc::c_int != checksum_calc as libc::c_int {
        printf(
            b"WARNING: checksum failed (got:0x%02X calc:0x%02X)\n\x00" as *const u8
                as *const libc::c_char,
            checksum_rcv as libc::c_int,
            checksum_calc as libc::c_int,
        );
        return -1i32;
    }
    /* Parse packet metadata */
    (*pkt).modem_id = ((*self_0).buffer[((*self_0).buffer_index + 5i32) as usize] as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32) as uint8_t;
    (*pkt).rx_channel_in =
        ((*self_0).buffer[((*self_0).buffer_index + 3i32) as usize] as libc::c_int >> 0i32
            & (1i32 << 8i32) - 1i32) as uint8_t;
    (*pkt).crc_en = (*self_0).buffer[((*self_0).buffer_index + 4i32) as usize] as libc::c_int
        >> 0i32
        & (1i32 << 1i32) - 1i32
        != 0;
    (*pkt).payload_crc_error = (*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 9i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 1i32) - 1i32
        != 0;
    (*pkt).sync_error = (*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 9i32) as usize]
        as libc::c_int
        >> 2i32
        & (1i32 << 1i32) - 1i32
        != 0;
    (*pkt).header_error = (*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 9i32) as usize]
        as libc::c_int
        >> 3i32
        & (1i32 << 1i32) - 1i32
        != 0;
    (*pkt).timing_set = (*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 9i32) as usize]
        as libc::c_int
        >> 4i32
        & (1i32 << 1i32) - 1i32
        != 0;
    (*pkt).coding_rate = ((*self_0).buffer[((*self_0).buffer_index + 4i32) as usize] as libc::c_int
        >> 1i32
        & (1i32 << 3i32) - 1i32) as uint8_t;
    (*pkt).rx_rate_sf = ((*self_0).buffer[((*self_0).buffer_index + 4i32) as usize] as libc::c_int
        >> 4i32
        & (1i32 << 4i32) - 1i32) as uint8_t;
    (*pkt).rssi_chan_avg = ((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 11i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32) as uint8_t;
    (*pkt).rssi_signal_avg = ((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 12i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32) as uint8_t;
    (*pkt).rx_crc16_value = (((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 19i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32)
        << 0i32
        & 0xffi32) as uint16_t;
    (*pkt).rx_crc16_value = ((*pkt).rx_crc16_value as libc::c_int
        | (((*self_0).buffer
            [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 20i32) as usize]
            as libc::c_int
            >> 0i32
            & (1i32 << 8i32) - 1i32)
            << 8i32
            & 0xff00i32) as uint16_t as libc::c_int) as uint16_t;
    (*pkt).snr_average = ((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 10i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32) as int8_t;
    (*pkt).frequency_offset_error =
        ((*self_0).buffer[((*self_0).buffer_index + 8i32) as usize] as libc::c_int >> 0i32
            & (1i32 << 4i32) - 1i32)
            << 16i32
            | ((*self_0).buffer[((*self_0).buffer_index + 7i32) as usize] as libc::c_int >> 0i32
                & (1i32 << 8i32) - 1i32)
                << 8i32
            | ((*self_0).buffer[((*self_0).buffer_index + 6i32) as usize] as libc::c_int >> 0i32
                & (1i32 << 8i32) - 1i32)
                << 0i32;
    if (*pkt).frequency_offset_error >= 1i32 << 19i32 {
        /* Handle signed value on 20bits */
        (*pkt).frequency_offset_error = (*pkt).frequency_offset_error - (1i32 << 20i32)
    }
    /* Packet timestamp (32MHz ) */
    (*pkt).timestamp_cnt = (((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 15i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32)
        << 0i32
        & 0xffi32) as uint32_t;
    (*pkt).timestamp_cnt |= (((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 16i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32)
        << 8i32
        & 0xff00i32) as uint32_t;
    (*pkt).timestamp_cnt |= (((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 17i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32)
        << 16i32
        & 0xff0000i32) as uint32_t;
    (*pkt).timestamp_cnt |= (((*self_0).buffer
        [((*self_0).buffer_index + (*pkt).rxbytenb_modem as libc::c_int + 18i32) as usize]
        as libc::c_int
        >> 0i32
        & (1i32 << 8i32) - 1i32)
        << 24i32) as libc::c_uint
        & 0xff000000u32;
    /* Sanity checks: check the range of few metadata */
    if (*pkt).modem_id as libc::c_int > 17i32 {
        printf(
            b"ERROR: modem_id is out of range - %u\n\x00" as *const u8 as *const libc::c_char,
            (*pkt).modem_id as libc::c_int,
        );
        return -1i32;
    } else {
        if (*pkt).modem_id as libc::c_int <= 16i32 {
            /* LoRa modems */
            if (*pkt).rx_channel_in as libc::c_int > 9i32 {
                printf(
                    b"ERROR: channel is out of range - %u\n\x00" as *const u8
                        as *const libc::c_char,
                    (*pkt).rx_channel_in as libc::c_int,
                );
                return -1i32;
            }
            if ((*pkt).rx_rate_sf as libc::c_int) < 5i32 || (*pkt).rx_rate_sf as libc::c_int > 12i32
            {
                printf(
                    b"ERROR: SF is out of range - %u\n\x00" as *const u8 as *const libc::c_char,
                    (*pkt).rx_rate_sf as libc::c_int,
                );
                return -1i32;
            }
        }
    }
    /* Parse & copy payload in packet struct */
    memcpy(
        (*pkt).payload.as_mut_ptr() as *mut libc::c_void,
        &mut *(*self_0)
            .buffer
            .as_mut_ptr()
            .offset(((*self_0).buffer_index + 9i32) as isize) as *mut uint8_t
            as *mut libc::c_void,
        (*pkt).rxbytenb_modem as libc::c_ulong,
    );
    /* Move buffer index toward next message */
    (*self_0).buffer_index += 9i32
        + (*pkt).rxbytenb_modem as libc::c_int
        + 14i32
        + 2i32 * (*pkt).num_ts_metrics_stored as libc::c_int;
    /* Update the umber of packets currently stored in the rx_buffer */
    (*self_0).buffer_pkt_nb = ((*self_0).buffer_pkt_nb as libc::c_int - 1i32) as uint8_t;
    return 0i32;
}
/* -------------------------------------------------------------------------- */
/* --- DEBUG FUNCTIONS PROTOTYPES ------------------------------------------- */
/* *
@brief TODO
@param TODO
@return TODO
*/
/* -------------------------------------------------------------------------- */
/* --- DEBUG FUNCTIONS DEFINITION ------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_read_ptr_addr() -> uint16_t {
    let mut val: int32_t = 0; /* mandatory to read MSB first */
    let mut addr: uint16_t = 0;
    lgw_reg_r(716i32 as uint16_t, &mut val);
    addr = (val << 8i32) as uint16_t;
    lgw_reg_r(717i32 as uint16_t, &mut val);
    addr = (addr as libc::c_int | val as uint16_t as libc::c_int) as uint16_t;
    return addr;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_write_ptr_addr() -> uint16_t {
    let mut val: int32_t = 0; /* mandatory to read MSB first */
    let mut addr: uint16_t = 0;
    lgw_reg_r(718i32 as uint16_t, &mut val);
    addr = (val << 8i32) as uint16_t;
    lgw_reg_r(719i32 as uint16_t, &mut val);
    addr = (addr as libc::c_int | val as uint16_t as libc::c_int) as uint16_t;
    return addr;
}
/* *
@brief TODO
@param TODO
@return TODO
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn rx_buffer_dump(
    mut file: *mut FILE,
    mut start_addr: uint16_t,
    mut end_addr: uint16_t,
) {
    let mut i: libc::c_int = 0;
    let mut rx_buffer_debug: [uint8_t; 4096] = [0; 4096];
    printf(
        b"Dumping %u bytes, from 0x%X to 0x%X\n\x00" as *const u8 as *const libc::c_char,
        end_addr as libc::c_int - start_addr as libc::c_int + 1i32,
        start_addr as libc::c_int,
        end_addr as libc::c_int,
    );
    memset(
        rx_buffer_debug.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[uint8_t; 4096]>() as libc::c_ulong,
    );
    lgw_reg_w(709i32 as uint16_t, 1i32);
    lgw_mem_rb(
        (0x4000i32 + start_addr as libc::c_int) as uint16_t,
        rx_buffer_debug.as_mut_ptr(),
        (end_addr as libc::c_int - start_addr as libc::c_int + 1i32) as uint16_t,
        0i32 != 0,
    );
    lgw_reg_w(709i32 as uint16_t, 0i32);
    i = 0i32;
    while i < end_addr as libc::c_int - start_addr as libc::c_int + 1i32 {
        if file.is_null() {
            printf(
                b"%02X \x00" as *const u8 as *const libc::c_char,
                rx_buffer_debug[i as usize] as libc::c_int,
            );
        } else {
            fprintf(
                file,
                b"%02X \x00" as *const u8 as *const libc::c_char,
                rx_buffer_debug[i as usize] as libc::c_int,
            );
        }
        i += 1
    }
    if file.is_null() {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    } else {
        fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    /* Switching to direct-access memory could lead to corruption, so to be done only for debugging */
    if 0i32 != 0 {
    } else {
        __assert_fail(
            b"0\x00" as *const u8 as *const libc::c_char,
            b"src/loragw_sx1302_rx.c\x00" as *const u8 as *const libc::c_char,
            389i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void rx_buffer_dump(FILE *, uint16_t, uint16_t)\x00",
            ))
            .as_ptr(),
        );
    };
}
/* --- EOF ------------------------------------------------------------------ */
