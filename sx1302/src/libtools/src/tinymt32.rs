use libc;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TINYMT32_T {
    pub status: [uint32_t; 4],
    pub mat1: uint32_t,
    pub mat2: uint32_t,
    pub tmat: uint32_t,
}
pub type tinymt32_t = TINYMT32_T;
#[inline]
unsafe extern "C" fn tinymt32_next_state(mut random: *mut tinymt32_t) {
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    y = (*random).status[3];
    x = (*random).status[0] & 0x7fffffffu32 ^ (*random).status[1] ^ (*random).status[2];
    x ^= x << 1i32;
    y ^= y >> 1i32 ^ x;
    (*random).status[0] = (*random).status[1];
    (*random).status[1] = (*random).status[2];
    (*random).status[2] = x ^ y << 10i32;
    (*random).status[3] = y;
    (*random).status[1] ^=
        -((y & 1i32 as libc::c_uint) as int32_t) as libc::c_uint & (*random).mat1;
    (*random).status[2] ^=
        -((y & 1i32 as libc::c_uint) as int32_t) as libc::c_uint & (*random).mat2;
}
/* *
 * This function represents a function used in the initialization
 * by init_by_array
 * @param x 32-bit integer
 * @return 32-bit integer
 */
unsafe extern "C" fn ini_func1(mut x: uint32_t) -> uint32_t {
    return (x ^ x >> 27i32).wrapping_mul(1664525u32);
}
/* *
 * This function represents a function used in the initialization
 * by init_by_array
 * @param x 32-bit integer
 * @return 32-bit integer
 */
unsafe extern "C" fn ini_func2(mut x: uint32_t) -> uint32_t {
    return (x ^ x >> 27i32).wrapping_mul(1566083941u32);
}
/* *
 * This function certificate the period of 2^127-1.
 * @param random tinymt state vector.
 */
unsafe extern "C" fn period_certification(mut random: *mut tinymt32_t) {
    if (*random).status[0] & 0x7fffffffu32 == 0i32 as libc::c_uint
        && (*random).status[1] == 0i32 as libc::c_uint
        && (*random).status[2] == 0i32 as libc::c_uint
        && (*random).status[3] == 0i32 as libc::c_uint
    {
        (*random).status[0] = 'T' as i32 as uint32_t;
        (*random).status[1] = 'I' as i32 as uint32_t;
        (*random).status[2] = 'N' as i32 as uint32_t;
        (*random).status[3] = 'Y' as i32 as uint32_t
    };
}
/* *
 * This function initializes the internal state array with a 32-bit
 * unsigned integer seed.
 * @param random tinymt state vector.
 * @param seed a 32-bit unsigned integer used as a seed.
 */
#[no_mangle]
pub unsafe extern "C" fn tinymt32_init(mut random: *mut tinymt32_t, mut seed: uint32_t) {
    (*random).status[0] = seed;
    (*random).status[1] = (*random).mat1;
    (*random).status[2] = (*random).mat2;
    (*random).status[3] = (*random).tmat;
    let mut i: libc::c_int = 1i32;
    while i < 8i32 {
        (*random).status[(i & 3i32) as usize] ^=
            (i as libc::c_uint).wrapping_add(1812433253u32.wrapping_mul(
                (*random).status[(i - 1i32 & 3i32) as usize]
                    ^ (*random).status[(i - 1i32 & 3i32) as usize] >> 30i32,
            ));
        i += 1
    }
    period_certification(random);
    let mut i_0: libc::c_int = 0i32;
    while i_0 < 8i32 {
        tinymt32_next_state(random);
        i_0 += 1
    }
}
/* *
 * This function initializes the internal state array,
 * with an array of 32-bit unsigned integers used as seeds
 * @param random tinymt state vector.
 * @param init_key the array of 32-bit integers, used as a seed.
 * @param key_length the length of init_key.
 */
#[no_mangle]
pub unsafe extern "C" fn tinymt32_init_by_array(
    mut random: *mut tinymt32_t,
    mut init_key: *mut uint32_t,
    mut key_length: libc::c_int,
) {
    let lag: libc::c_int = 1i32;
    let mid: libc::c_int = 1i32;
    let size: libc::c_int = 4i32;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut r: uint32_t = 0;
    let mut st: *mut uint32_t = &mut *(*random).status.as_mut_ptr().offset(0) as *mut uint32_t;
    *st.offset(0) = 0i32 as uint32_t;
    *st.offset(1) = (*random).mat1;
    *st.offset(2) = (*random).mat2;
    *st.offset(3) = (*random).tmat;
    if key_length + 1i32 > 8i32 {
        count = key_length + 1i32
    } else {
        count = 8i32
    }
    r = ini_func1(
        *st.offset(0)
            ^ *st.offset((mid % size) as isize)
            ^ *st.offset(((size - 1i32) % size) as isize),
    );
    let ref mut fresh0 = *st.offset((mid % size) as isize);
    *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
    r = (r as libc::c_uint).wrapping_add(key_length as libc::c_uint) as uint32_t as uint32_t;
    let ref mut fresh1 = *st.offset(((mid + lag) % size) as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
    *st.offset(0) = r;
    count -= 1;
    i = 1i32;
    j = 0i32;
    while j < count && j < key_length {
        r = ini_func1(
            *st.offset((i % size) as isize)
                ^ *st.offset(((i + mid) % size) as isize)
                ^ *st.offset(((i + size - 1i32) % size) as isize),
        );
        let ref mut fresh2 = *st.offset(((i + mid) % size) as isize);
        *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
        r = (r as libc::c_uint)
            .wrapping_add((*init_key.offset(j as isize)).wrapping_add(i as libc::c_uint))
            as uint32_t as uint32_t;
        let ref mut fresh3 = *st.offset(((i + mid + lag) % size) as isize);
        *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
        *st.offset((i % size) as isize) = r;
        i = (i + 1i32) % size;
        j += 1
    }
    while j < count {
        r = ini_func1(
            *st.offset((i % size) as isize)
                ^ *st.offset(((i + mid) % size) as isize)
                ^ *st.offset(((i + size - 1i32) % size) as isize),
        );
        let ref mut fresh4 = *st.offset(((i + mid) % size) as isize);
        *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
        r = (r as libc::c_uint).wrapping_add(i as libc::c_uint) as uint32_t as uint32_t;
        let ref mut fresh5 = *st.offset(((i + mid + lag) % size) as isize);
        *fresh5 = (*fresh5 as libc::c_uint).wrapping_add(r) as uint32_t as uint32_t;
        *st.offset((i % size) as isize) = r;
        i = (i + 1i32) % size;
        j += 1
    }
    j = 0i32;
    while j < size {
        r = ini_func2(
            (*st.offset((i % size) as isize))
                .wrapping_add(*st.offset(((i + mid) % size) as isize))
                .wrapping_add(*st.offset(((i + size - 1i32) % size) as isize)),
        );
        let ref mut fresh6 = *st.offset(((i + mid) % size) as isize);
        *fresh6 ^= r;
        r = (r as libc::c_uint).wrapping_sub(i as libc::c_uint) as uint32_t as uint32_t;
        let ref mut fresh7 = *st.offset(((i + mid + lag) % size) as isize);
        *fresh7 ^= r;
        *st.offset((i % size) as isize) = r;
        i = (i + 1i32) % size;
        j += 1
    }
    period_certification(random);
    i = 0i32;
    while i < 8i32 {
        tinymt32_next_state(random);
        i += 1
    }
}
