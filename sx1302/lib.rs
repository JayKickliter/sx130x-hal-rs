#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(ptr_wrapping_offset_from)]

pub mod src {
    pub mod libloragw {
        pub mod src {
            pub mod loragw_aux;
            pub mod loragw_cal;
            pub mod loragw_debug;
            pub mod loragw_gps;
            pub mod loragw_hal;
            pub mod loragw_i2c;
            pub mod loragw_reg;
            pub mod loragw_spi;
            pub mod loragw_stts751;
            pub mod loragw_sx1250;
            pub mod loragw_sx125x;
            pub mod loragw_sx1302;
            pub mod loragw_sx1302_rx;
            pub mod loragw_sx1302_timestamp;
        } // mod src
    } // mod libloragw
    pub mod libtools {
        pub mod src {
            pub mod base64;
            pub mod parson;
            pub mod tinymt32;
        } // mod src
    } // mod libtools
} // mod src
