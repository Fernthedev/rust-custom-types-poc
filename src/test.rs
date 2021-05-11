use std::str::FromStr;

use crate::custom_types::*;
use proc_macros::custom_type_class;

// Example codegen class
trait SomeOtherClass {}

struct SomeOtherClassImpl {}

impl SomeOtherClass for SomeOtherClassImpl {}

impl Il2CppObject for SomeOtherClassImpl {
    const klass: Il2CppClass = Il2CppClass {
        namespace: String::from_str("").unwrap(),
        name: String::from_str("SomeOtherClass").unwrap(),
    };
}

#[custom_type_class("", "", "SomeOtherClass")]
struct CustomTypeA {}
