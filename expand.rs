mod test {
    use crate::custom_types::*;
    use proc_macros::custom_type_class;
    use quest_hook::libil2cpp::*;
    trait SomeOtherClass {}
    struct SomeOtherClassImpl {}
    impl SomeOtherClass for SomeOtherClassImpl {}
    impl From<Il2CppObject> for SomeOtherClassImpl {
        fn from(_: Il2CppObject) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
    }
    struct CustomTypeBStruct {
        base: SomeOtherClassImpl,
        va: i64,
    }
    trait CustomTypeB {
        fn t(&self) {}
        fn static_method() {}
        fn ov(&self) -> i64;
        fn getv_va(&self) -> i64;
        fn setv_va(&mut self, set: i64);
    }
    impl CustomTypeB for CustomTypeBStruct {
        fn ov(&self) -> i64 {
            self.va + 2
        }
        fn getv_va(&self) -> i64 {
            self.va
        }
        fn setv_va(&mut self, set: i64) {
            self.va = set;
        }
    }
    impl From<Il2CppObject> for CustomTypeBStruct {
        fn from(_: Il2CppObject) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl From<SomeOtherClassImpl> for CustomTypeBStruct {
        fn from(_: SomeOtherClassImpl) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
    }
    struct CustomTypeA {}
    trait CustomTypeATrait {
        fn get_custom_klass() -> Il2CppClass;
    }
    impl CustomTypeClassTrait for CustomTypeA {
        fn install() {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["Installing custom type ", "::", "!\n"],
                    &match (&"", &"CustomTypeA") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
            };
        }
    }
    impl From<Il2CppObject> for CustomTypeA {
        fn from(il2CppObject: Il2CppObject) -> Self {
            ::core::panicking::panic("not yet implemented");
        }
    }
    impl From<SomeOtherClassImpl> for CustomTypeA {
        fn from(base: SomeOtherClassImpl) -> Self {
            ::core::panicking::panic("not yet implemented");
        }
    }
}
