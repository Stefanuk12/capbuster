pub mod client;
pub mod endpoints;
pub mod proxy;
pub mod response;
pub mod services;

#[macro_export]
macro_rules! import {
    ($($module:ident),*) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}
