#[macro_export]
macro_rules! impl_class {
    (
        $impl_type:ty ;
        $class_name_str:literal
    ) => {
        $crate::impl_class!(
            $impl_type : $crate::classes::TopClass => CLASS_INSTANCE ;
            $class_name_str => CLASS_NAME : $crate::properties::simple::name::SimpleName
        );
    };
    (
        $impl_type:ty => $class_instance_name:ident ;
        $class_name_str:literal => $class_const_name:ident
    ) => {
        $crate::impl_class!(
            $impl_type : $crate::classes::TopClass => $class_instance_name ;
            $class_name_str => $class_const_name : $crate::properties::simple::name::SimpleName
        );
    };
    (
        $impl_type:ty => $class_instance_name:ident ;
        $class_name_str:literal => $class_const_name:ident : $name_type:ty
    ) => {
        $crate::impl_class!(
            $impl_type : $crate::properties::class::TopClass => $class_instance_name ;
            $class_name_str => $class_const_name : $name_type
        );
    };
    (
        $impl_type:ty : $super_type:ty => $class_instance_name:ident ;
        $class_name_str:literal => $class_const_name:ident
    ) => {
        $crate::impl_class!(
            $impl_type : $super_type => $class_instance_name ;
            $class_name_str => $class_const_name : $crate::properties::simple::name::SimpleName
        );
    };
    (
        $impl_type:ty : $super_type:ty => $class_instance_name:ident ;
        $class_name_str:literal => $class_const_name:ident : $name_type:ty
    ) => {
        static $class_const_name: ::std::sync::LazyLock<$name_type>
            = ::std::sync::LazyLock::new(||<$name_type as $crate::strings::UncheckedFromStr>::new_unchecked($class_name_str));

        impl $crate::names::HasName for $impl_type {
            type Name = $name_type;

            fn name(&self) -> &$name_type {
                &$class_const_name
            }
        }

        impl $crate::classes::Class for $impl_type {
            type Super = $super_type;

            fn super_class(&self) -> core::option::Option<Self::Super> {
                Some(<$super_type>::default())
            }
        }

        impl $crate::properties::aci::ProvidesAmznCommonIdentifier for $impl_type {
            type Name = $crate::properties::simple::name::SimpleName;

            fn provide_aci(&self) -> $crate::properties::aci::AmznCommonIdentifier<Self::Name> {
                $crate::properties::aci::AmznCommonIdentifier::new_unchecked(&["Class"])
                    .named(
                        <$crate::properties::names::SimpleName as $crate::strings::UncheckedFromStr>::new_unchecked(
                            $class_const_name.as_str()
                        )
                    )
            }
        }

        pub static $class_instance_name: ::std::sync::LazyLock<$impl_type>
            = ::std::sync::LazyLock::new(|| <$impl_type>::default());
    };
}

#[macro_export]
macro_rules! impl_has_class {
    ($impl_type:ty => $class_type:ty, const $instance_name:ident) => {
        impl $crate::classes::HasClass for $impl_type {
            type Class = $class_type;

            fn class(&self) -> &Self::Class {
                &$instance_name
            }
        }
    };
    ($impl_type:ty => $class_type:ty, field $field_name:ident) => {
        impl $crate::classes::HasClass for $impl_type {
            type Class = $class_type;

            fn class(&self) -> &Self::Class {
                &self.$field_name
            }
        }
    };
}
