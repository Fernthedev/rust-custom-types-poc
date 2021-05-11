use crate::custom_types::*;
use proc_macros::custom_type_class;

// Example codegen class
trait SomeOtherClass {}

struct SomeOtherClassImpl {}

impl SomeOtherClass for SomeOtherClassImpl {}

impl Il2CppObject for SomeOtherClassImpl {
    const KLASS: Il2CppClass = Il2CppClass {
        namespace: "",
        name: "SomeOtherClassImpl",
    };
}

#[custom_type_class("", "", "SomeOtherClass")]
struct CustomTypeA {}
