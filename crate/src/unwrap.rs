// until https://github.com/rustwasm/wasm-bindgen/issues/2066 lands properly
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

pub trait UnwrapExt<T>: Sized {
    #[track_caller]
    fn unwrap_ext(self) -> T {
        self.expect_ext("`unwrap_ext` failed")
    }

    #[track_caller]
    fn expect_ext(self, message: &str) -> T;
}

cfg_if! {
    if #[cfg(feature = "unwrap_verbose")] {
        impl<T> UnwrapExt<T> for Option<T> {
            fn expect_ext(self, message: &str) -> T {
                self.expect(message)
            }
        }

        impl<T, E> UnwrapExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            fn expect_ext(self, message: &str) -> T {
                self.expect(message)
            }
        }
    } else {

        impl<T> UnwrapExt<T> for Option<T> {
            fn expect_ext(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }

        impl<T, E> UnwrapExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            fn expect_ext(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }
    }
}
