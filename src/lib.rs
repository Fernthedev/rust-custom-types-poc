mod test;

pub use proc_macros::custom_type_class;

pub mod custom_types {
    pub struct Il2CppClass {
        pub namespace: &'static str,
        pub name: &'static str,
    }

    pub trait Il2CppObject {
        const klass: Il2CppClass;
    }

    // Example implementation of codegen
    pub struct Il2CppObjectImpl {
        pub klazz: Il2CppClass,
    }

    // Represents a method
    pub struct SampleMethod {
        pub returnType: Il2CppClass,
        pub parameters: Vec<Il2CppClass>, // TODO: Support generics?
    }

    pub trait CustomTypeClassTrait {
        fn install();
    }
}
