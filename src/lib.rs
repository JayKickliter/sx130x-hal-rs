// #![crate_type = "staticlib"]

pub mod sx1301 {
    pub use sx1301_hal_rs::src::*;
}

pub mod sx1302 {
    pub use sx1302_hal_rs::src::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sx1301_start() {
        use crate::*;
        unsafe {
            sx1301::loragw_hal::lgw_start();
        }
    }

    #[test]
    fn test_sx1302_start() {
        use crate::*;
        unsafe {
            sx1302::libloragw::src::loragw_hal::lgw_start();
        }
    }
}
