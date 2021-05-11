mod test;

extern crate quest_hook;

pub use proc_macros::custom_type_class;

pub mod custom_types {
    pub trait CustomTypeClassTrait {
        fn install();
    }
}
