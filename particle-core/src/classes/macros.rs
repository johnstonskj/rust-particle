#[macro_export]
macro_rules! defn_class {
    ($name:ident) => {
        pastey::paste! {
            #[derive(
                ::core::clone::Clone,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
                ::core::cmp::PartialOrd,
                ::core::cmp::Ord,
                ::core::default::Default,
                ::core::fmt::Debug,
                ::core::hash::Hash,
                ::core::marker::Copy,
            )]
            #[doc = "Implementation of the `" $name "` class, specifically the `Classifier` trait."]
            pub struct [< $name:camel Class >];
        }
    };
}

#[macro_export]
macro_rules! impl_class {
    (
        $name:ident
    ) => {
        impl_class! {
            $name : $crate::classes::ClassClass
        }
    };
    (
        $name:ident : $super_class_type:ty
    ) => {
        pastey::paste! {
            impl_class! {
                @inner
                stringify!([< $name:camel >]),
                [< $name:snake:upper _CLASS_NAME_STR >],
                [< $name:snake:upper _CLASS_NAME >],
                [< $name:snake:upper _CLASS >],
                [< $name:camel Class >],
                $super_class_type
            }
        }
    };
    (
        @inner
        $class_name_str:expr,
        $const_class_name_str:ident,
        $const_class_name:ident,
        $const_class:ident,
        $impl_type:ident,
        $super_class_type:ty
    ) => {
        pastey::paste! {
            const $const_class_name_str: &str = $class_name_str;
            const $const_class_name: $crate::classes::ClassifierName
                = $crate::classes::ClassifierName::new_unchecked($const_class_name_str);
            const $const_class: $impl_type = $impl_type::new();

            impl $crate::classes::Classifier for $impl_type {
                type Super = $super_class_type;

                fn super_class(&self) -> Option<&Self::Super> {
                    Some(<Self::Super as $crate::properties::Singleton>::instance())
                }

                fn name(&self) -> &$crate::classes::ClassifierName {
                    &$const_class_name
                }
            }

            impl $crate::classes::HasClassification for $impl_type {
                type Classifier = $crate::classes::ClassClass;

                fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier> {
                    ::core::iter::once(
                        <Self::Classifier as $crate::properties::Singleton>::instance()
                    )
                }
            }

            impl $crate::properties::Singleton for $impl_type {
                fn instance() -> &'static Self {
                    &$const_class
                }
            }

            impl $impl_type {
                const fn new() -> Self {
                    Self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_has_class {
    ($impl_type:ty => $class_type:ty) => {
        impl $crate::classes::HasClassification for $impl_type {
            type Classifier = $class_type;

            fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier> {
                ::core::iter::once(<Self::Classifier as $crate::properties::Singleton>::instance())
            }
        }
    };
    ($impl_type:ty => $class_type:ty, const $const_array:ident) => {
        impl $crate::classes::HasClassification for $impl_type {
            type Classifier = $class_type;

            fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier> {
                $const_array.iter()
            }
        }
    };
    ($impl_type:ty => $class_type:ty, field $field_name:ident) => {
        impl $crate::classes::HasClassification for $impl_type {
            type Classifier = $class_type;

            fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier> {
                self.$field_name.iter()
            }
        }
    };
}
