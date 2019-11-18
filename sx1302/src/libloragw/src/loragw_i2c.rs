use libc;
extern "C" {
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_msg {
    pub addr: __u16,
    pub flags: __u16,
    pub len: __u16,
    pub buf: *mut __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_rdwr_ioctl_data {
    pub msgs: *mut i2c_msg,
    pub nmsgs: __u32,
}
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS PROTOTYPES ------------------------------------------ */
/* *
@brief Open I2C port
@param path         Path to the I2C device driver (absolute or relative)
@param device_addr  Address of the device
@param i2c_fd      Pointer to receive I2C port file descriptor index
@return 0 if I2C port was open successfully, -1 else
*/
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    Host specific functions to address the LoRa concentrator I2C peripherals.

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* C99 types */
/* printf fprintf */
/* malloc free */
/* lseek, close */
/* open */
/* memset */
/* errno */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE CONSTANTS ---------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn i2c_linuxdev_open(
    mut path: *const libc::c_char,
    mut device_addr: uint8_t,
    mut i2c_fd: *mut libc::c_int,
) -> libc::c_int {
    let mut dev: libc::c_int = 0;
    /* Check input variables */
    if path.is_null() {
        return -1i32;
    }
    if i2c_fd.is_null() {
        return -1i32;
    }
    /* Open I2C device */
    dev = open(path, 0o2i32);
    if dev < 0i32 {
        return -1i32;
    }
    /* Setting I2C device mode to slave */
    if ioctl(dev, 0x703i32 as libc::c_ulong, device_addr as libc::c_int) < 0i32 {
        return -1i32;
    } /* return file descriptor index */
    *i2c_fd = dev;
    return 0i32;
}
/* *
@brief Read data from an I2C port
@param i2c_fd      I2C port file descriptor index
@param device_addr  I2C device address
@param reg_addr     Address of the register to be read
@param data         Pointer to a buffer to store read data
@return 0 if I2C data read is successful, -1 else
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn i2c_linuxdev_read(
    mut i2c_fd: libc::c_int,
    mut device_addr: uint8_t,
    mut reg_addr: uint8_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    let mut inbuff: *mut uint8_t = 0 as *mut uint8_t;
    let mut outbuff: uint8_t = 0;
    let mut packets: i2c_rdwr_ioctl_data = i2c_rdwr_ioctl_data {
        msgs: 0 as *mut i2c_msg,
        nmsgs: 0,
    };
    let mut messages: [i2c_msg; 2] = [i2c_msg {
        addr: 0,
        flags: 0,
        len: 0,
        buf: 0 as *mut __u8,
    }; 2];
    outbuff = reg_addr;
    messages[0].addr = device_addr as __u16;
    messages[0].flags = 0i32 as __u16;
    messages[0].len = ::std::mem::size_of::<uint8_t>() as libc::c_ulong as __u16;
    messages[0].buf = &mut outbuff;
    inbuff = data;
    messages[1].addr = device_addr as __u16;
    messages[1].flags = 0x1i32 as __u16;
    messages[1].len = ::std::mem::size_of::<uint8_t>() as libc::c_ulong as __u16;
    messages[1].buf = inbuff;
    packets.msgs = messages.as_mut_ptr();
    packets.nmsgs = 2i32 as __u32;
    if ioctl(
        i2c_fd,
        0x707i32 as libc::c_ulong,
        &mut packets as *mut i2c_rdwr_ioctl_data,
    ) < 0i32
    {
        return -1i32;
    }
    return 0i32;
}
/* *
@brief Write data to an I2C port
@param i2c_fd      I2C port file descriptor index
@param device_addr  I2C device address
@param reg_addr     Address of the register to write to
@param data         byte to write in the register
@return 0 if I2C data write is successful, -1 else
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn i2c_linuxdev_write(
    mut i2c_fd: libc::c_int,
    mut device_addr: uint8_t,
    mut reg_addr: uint8_t,
    mut data: uint8_t,
) -> libc::c_int {
    let mut buff: [libc::c_uchar; 2] = [0; 2];
    let mut packets: i2c_rdwr_ioctl_data = i2c_rdwr_ioctl_data {
        msgs: 0 as *mut i2c_msg,
        nmsgs: 0,
    };
    let mut messages: [i2c_msg; 1] = [i2c_msg {
        addr: 0,
        flags: 0,
        len: 0,
        buf: 0 as *mut __u8,
    }; 1];
    buff[0] = reg_addr;
    buff[1] = data;
    messages[0].addr = device_addr as __u16;
    messages[0].flags = 0i32 as __u16;
    messages[0].len = ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as __u16;
    messages[0].buf = buff.as_mut_ptr();
    packets.msgs = messages.as_mut_ptr();
    packets.nmsgs = 1i32 as __u32;
    if ioctl(
        i2c_fd,
        0x707i32 as libc::c_ulong,
        &mut packets as *mut i2c_rdwr_ioctl_data,
    ) < 0i32
    {
        return -1i32;
    }
    return 0i32;
}
/* *
@brief Close I2C port
@param i2c_fd      I2C port file descriptor index
@return 0 if I2C port was closed successfully, -1 else
*/
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
#[no_mangle]
pub unsafe extern "C" fn i2c_linuxdev_close(mut i2c_fd: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = close(i2c_fd);
    if i == 0i32 {
        return 0i32;
    } else {
        return -1i32;
    };
}
/* --- EOF ------------------------------------------------------------------ */
