#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]

pub mod src {
    pub mod loragw_aux;
    pub mod loragw_fpga;
    pub mod loragw_gps;
    pub mod loragw_hal;
    pub mod loragw_lbt;
    pub mod loragw_radio;
    pub mod loragw_reg;
    pub mod loragw_spi_native;
} // mod src
