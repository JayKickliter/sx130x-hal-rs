use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    /* *
    @brief Read data from an I2C port
    @param i2c_fd      I2C port file descriptor index
    @param device_addr  I2C device address
    @param reg_addr     Address of the register to be read
    @param data         Pointer to a buffer to store read data
    @return 0 if I2C data read is successful, -1 else
    */
    #[no_mangle]
    fn i2c_linuxdev_read(
        i2c_fd: libc::c_int,
        device_addr: uint8_t,
        reg_addr: uint8_t,
        data: *mut uint8_t,
    ) -> libc::c_int;
    /* *
    @brief Write data to an I2C port
    @param i2c_fd      I2C port file descriptor index
    @param device_addr  I2C device address
    @param reg_addr     Address of the register to write to
    @param data         byte to write in the register
    @return 0 if I2C data write is successful, -1 else
    */
    #[no_mangle]
    fn i2c_linuxdev_write(
        i2c_fd: libc::c_int,
        device_addr: uint8_t,
        reg_addr: uint8_t,
        data: uint8_t,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Basic driver for ST ts751 temperature sensor

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* bool type */
/* library configuration options (dynamically generated) */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED TYPES ------------------------------------------------ */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED FUNCTIONS -------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC CONSTANTS ----------------------------------------------------- */
/* STTS751-0DP3F */
/* STTS751-1DP3F */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS ----------------------------------------------------- */
/* *
@brief TODO
@param TODO
@return TODO
*/
/* -------------------------------------------------------------------------- */
/* --- PRIVATE VARIABLES ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- INTERNAL SHARED VARIABLES -------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE FUNCTIONS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn stts751_configure(
    mut i2c_fd: libc::c_int,
    mut i2c_addr: uint8_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut val: uint8_t = 0;
    /* Check Input Params */
    if i2c_fd <= 0i32 {
        printf(b"ERROR: invalid I2C file descriptor\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    /* Get product ID  and test which sensor is mounted */
    err = i2c_linuxdev_read(i2c_fd, i2c_addr, 0xfdi32 as uint8_t, &mut val);
    if err != 0i32 {
        return -1i32;
    }
    match val as libc::c_int {
        0 | 1 => {}
        _ => {
            printf(b"ERROR: Product ID: UNKNOWN\n\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    }
    /* Get Manufacturer ID */
    err = i2c_linuxdev_read(i2c_fd, i2c_addr, 0xfei32 as uint8_t, &mut val);
    if err != 0i32 {
        return -1i32;
    }
    if val as libc::c_int != 0x53i32 {
        printf(b"ERROR: Manufacturer ID: UNKNOWN\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    /* Get revision number */
    err = i2c_linuxdev_read(i2c_fd, i2c_addr, 0xffi32 as uint8_t, &mut val);
    if err != 0i32 {
        return -1i32;
    }
    /* Set conversion resolution to 12 bits */
    err = i2c_linuxdev_write(i2c_fd, i2c_addr, 0x3i32 as uint8_t, 0x8ci32 as uint8_t); /* TODO: do not hardcode the whole byte */
    if err != 0i32 {
        return -1i32;
    }
    /* Set conversion rate to 1 / second */
    err = i2c_linuxdev_write(i2c_fd, i2c_addr, 0x4i32 as uint8_t, 0x4i32 as uint8_t);
    if err != 0i32 {
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
pub unsafe extern "C" fn stts751_get_temperature(
    mut i2c_fd: libc::c_int,
    mut i2c_addr: uint8_t,
    mut temperature: *mut libc::c_float,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut high_byte: uint8_t = 0;
    let mut low_byte: uint8_t = 0;
    let mut h: int8_t = 0;
    /* Check Input Params */
    if i2c_fd <= 0i32 {
        printf(b"ERROR: invalid I2C file descriptor\n\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    /* Read Temperature LSB */
    err = i2c_linuxdev_read(i2c_fd, i2c_addr, 0x2i32 as uint8_t, &mut low_byte);
    if err != 0i32 {
        printf(
            b"ERROR: failed to read I2C device 0x%02X (err=%i)\n\x00" as *const u8
                as *const libc::c_char,
            i2c_addr as libc::c_int,
            err,
        );
        return -1i32;
    }
    /* Read Temperature MSB */
    err = i2c_linuxdev_read(i2c_fd, i2c_addr, 0i32 as uint8_t, &mut high_byte);
    if err != 0i32 {
        printf(
            b"ERROR: failed to read I2C device 0x%02X (err=%i)\n\x00" as *const u8
                as *const libc::c_char,
            i2c_addr as libc::c_int,
            err,
        );
        return -1i32;
    }
    h = high_byte as int8_t;
    *temperature = (((h as libc::c_int) << 8i32 | low_byte as libc::c_int) as libc::c_double
        / 256.0f64) as libc::c_float;
    return 0i32;
}
/* --- EOF ------------------------------------------------------------------ */
