use libc;
extern "C" {
    #[no_mangle]
    fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: libc::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
/*
 / _____)             _              | |
( (____  _____ ____ _| |_ _____  ____| |__
 \____ \| ___ |    (_   _) ___ |/ ___)  _ \
 _____) ) ____| | | || |_| ____( (___| | | |
(______/|_____)_|_|_| \__)_____)\____)_| |_|
  (C)2019 Semtech

Description:
    LoRa concentrator HAL auxiliary functions

License: Revised BSD License, see LICENSE.TXT file include in the project
*/
/* -------------------------------------------------------------------------- */
/* --- DEPENDANCIES --------------------------------------------------------- */
/* fix an issue between POSIX and C99 */
/* printf fprintf */
/* clock_nanosleep */
/* -------------------------------------------------------------------------- */
/* --- PRIVATE MACROS ------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* --- PUBLIC FUNCTIONS DEFINITION ------------------------------------------ */
/* This implementation is POSIX-pecific and require a fix to be compatible with C99 */
#[no_mangle]
pub unsafe extern "C" fn wait_ms(mut a: libc::c_ulong) {
    let mut dly: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rem: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    dly.tv_sec = a.wrapping_div(1000i32 as libc::c_ulong) as __time_t;
    dly.tv_nsec = a as libc::c_long % 1000i32 as libc::c_long * 1000000i32 as libc::c_long;
    if dly.tv_sec > 0i32 as libc::c_long
        || dly.tv_sec == 0i32 as libc::c_long && dly.tv_nsec > 100000i32 as libc::c_long
    {
        // clock_nanosleep(1i32, 0i32, &mut dly, &mut rem);
    };
}
/* --- EOF ------------------------------------------------------------------ */
