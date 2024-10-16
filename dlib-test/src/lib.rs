use dlib::external_library;

external_library!(Mlib, "m",
    functions:
        fn cos(f64) -> f64,
        #[cfg(feature = "sin")]
        fn sin(f64) -> f64,
        #[cfg(feature = "nonexistant")]
        fn nonexistant_function(f64) -> f64,
);

#[cfg(feature = "dlopen")]
lazy_static::lazy_static! {
    pub static ref M_STATIC: Mlib =
        unsafe { Mlib::open("libm.so.6").expect("could not find libm") };
}

#[cfg(test)]
mod tests {
    use super::*;
    use dlib::ffi_dispatch;
    #[test]
    fn invoke_cos() {
        let angle = 1.8;
        let cosinus = unsafe { ffi_dispatch!(M_STATIC, cos, angle) };
        assert_eq!(cosinus, angle.cos());

    }

    #[cfg(feature = "sin")]
    #[test]
    fn invoke_sin() {
        let angle = 1.8;
        let sine = unsafe { ffi_dispatch!(M_STATIC, sin, angle) };
        assert_eq!(sine, angle.sin());
    }
}
