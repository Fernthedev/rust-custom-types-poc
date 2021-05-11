mod test {
    use std::str::FromStr;
    use crate::custom_types::*;
    use proc_macros::custom_type_class;
    trait SomeOtherClass {}
    struct SomeOtherClassImpl {}
    impl SomeOtherClass for SomeOtherClassImpl {}
    impl Il2CppObject for SomeOtherClassImpl {
        const klass: Il2CppClass = Il2CppClass {
            namespace: "",
            name: "SomeOtherClassImpl",
        };
    }
    struct CustomTypeA {}
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
    impl Il2CppObject for CustomTypeA {
        const klass: Il2CppClass = Il2CppClass {
            namespace: "",
            name: "CustomTypeA",
        };
    }
    impl SomeOtherClass for CustomTypeA {}
}
