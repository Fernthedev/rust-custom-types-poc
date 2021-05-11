use crate::custom_types::*;
use proc_macros::custom_type_class;
use quest_hook::libil2cpp::*;

// Example codegen class
trait SomeOtherClass {}

struct SomeOtherClassImpl {}

impl SomeOtherClass for SomeOtherClassImpl {}

// Version 1?
// impl Il2CppObject for SomeOtherClassImpl {
//     const KLASS: Il2CppClass = Il2CppClass {
//         namespace: "",
//         name: "SomeOtherClassImpl",
//     };
// }

impl From<Il2CppObject> for SomeOtherClassImpl {
    fn from(_: Il2CppObject) -> Self {
        todo!()
    }
}

struct CustomTypeBStruct {
    base: SomeOtherClassImpl,
    va: i64,
}

trait CustomTypeB {
    // Optional function to override
    fn t(&self) {
        // RunMethod("t")
    }

    fn static_method() {}

    // Force override this
    fn ov(&self) -> i64;

    // Generated getter? getv_varname
    fn getv_va(&self) -> i64;

    // Generated setter? setv_varname
    fn setv_va(&mut self, set: i64);
}

impl CustomTypeB for CustomTypeBStruct {
    fn ov(&self) -> i64 {
        self.va + 2 // Some random method
    }

    fn getv_va(&self) -> i64 {
        self.va
    }

    fn setv_va(&mut self, set: i64) {
        self.va = set;
    }
}

// Allow down casting from Il2CppObject
impl From<Il2CppObject> for CustomTypeBStruct {
    fn from(_: Il2CppObject) -> Self {
        todo!()
    }
}

// Allow down casting from parent SomeOtherClassImpl
impl From<SomeOtherClassImpl> for CustomTypeBStruct {
    fn from(_: SomeOtherClassImpl) -> Self {
        todo!()
    }
}

// TODO: Implement Into<Il2CppObject> and Into<Parent>?

#[custom_type_class("", "", "SomeOtherClass")]
struct CustomTypeA {}
