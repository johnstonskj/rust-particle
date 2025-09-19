#[macro_export]
macro_rules! impl_has_state {
    ($impl_type:ty => $state_type:ty) => {
        $crate::impl_has_state!($impl_type => $state_type, state);
    };
    ($impl_type:ty => $state_type:ty, $field_name:ident) => {
        impl $crate::properties::state::HasState for $impl_type {
            $crate::impl_has_state!(=> $state_type, $field_name);
        }
    };
    (=> $state_type:ty, $field_name:ident) => {
        type State = $state_type;

        fn current_state(&self) -> &Self::State {
            &self.$field_name
        }
    };
}

#[macro_export]
macro_rules! impl_has_state_mut {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_state_mut!($impl_type => $value_type, state);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::properties::state::HasStateMut for $impl_type {
            $crate::impl_has_state_mut!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        fn set_state(&mut self, state: Self::State) -> Result<(), PropertyError> {
            self.$field_name = state;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! impl_enum_state {
    (
        $name_prefix:literal {
            $(
                $kind:ident $variant_name:ident $(, $variant_name_str:literal )? => $( $meta:ident )? $value:literal
            ),+
        }
    ) => {
        pastey::paste! {
            $crate::impl_enum_state! {
                    [< $name_prefix:camel State >] ;
                    [< $name_prefix:camel Class >] => $name_prefix
                {
                    $(
                        $kind $variant_name $(, $variant_name_str )? => $( $meta )? $value
                    ),+
                }
            }
        }
    };
    (
        $state_enum_name:ident ;
        $class_name:ident => $class_name_str:literal {
            $(
                $kind:ident $variant_name:ident $(, $variant_name_str:literal )? => $( $meta:ident )? $value:literal
            ),+
        }
    ) => {
        pastey::paste! {
            // Public Types

            #[derive(Clone, Copy, ::std::fmt::Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $class_name;

            #[derive(Clone, Copy, ::std::fmt::Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(u32)]
            pub enum $state_enum_name {
                $(
                    $( #[$meta] )? $variant_name = $value,
                )+
            }

            // Implementations

            $crate::impl_class!(
                $class_name : $crate::properties::state::StateClass => [< $class_name_str:snake:upper _CLASS > ] ;
                $class_name_str => [< $class_name_str:snake:upper _CLASS_NAME >]
            );

            static VARIANT_NAMES: ::std::sync::LazyLock<
                    ::std::collections::HashMap<$state_enum_name,
                    $crate::properties::simple::name::SimpleName>
                > = ::std::sync::LazyLock::new(|| {
                    ::std::collections::HashMap::from([
                        $(
                            $crate::impl_enum_state!(@name $state_enum_name, $variant_name $(=> $variant_name_str)?),
                        )+
                    ])
                }
            );

            $crate::impl_has_class!($state_enum_name => $class_name, const [< $class_name_str:snake:upper _CLASS > ]);

            impl $crate::properties::name::HasName for $state_enum_name {
                type Name = $crate::properties::simple::name::SimpleName;

                fn name(&self) -> &Self::Name {
                    VARIANT_NAMES.get(self).unwrap()
                }
            }

            impl $crate::properties::aci::ProvidesAmznCommonIdentifier for $state_enum_name {
                type Name = $crate::properties::simple::name::SimpleName;

                fn provide_aci(&self) -> $crate::properties::aci::AmznCommonIdentifier<Self::Name> {
                    $crate::properties::aci::AmznCommonIdentifier::new_with_class(self.class()).named(self.name().clone())
                }
            }

            impl $crate::properties::state::State for $state_enum_name {
                fn kind(&self) -> $crate::properties::state::StateKind {
                    match self {
                        $(
                            Self::$variant_name => $crate::properties::state::StateKind::[< $kind:upper_camel >],
                        )+
                    }
                }
            }
        }
    };
    (@name $state_enum_name:ident, $variant_name:ident) => {
        (
            $state_enum_name::$variant_name,
            $crate::properties::simple::name::SimpleName::new_unchecked(stringify!($variant_name)),
        )
    };
    (@name $state_enum_name:ident, $variant_name:ident => $variant_name_str:literal) => {
        (
            $state_enum_name::$variant_name,
            $crate::properties::simple::name::SimpleName::new_unchecked($variant_name_str),
        )
    };
}

#[macro_export]
macro_rules! impl_has_potential_states {
    ($impl_type:ty => $set_type:ty, $state_type:ty) => {
        $crate::impl_has_potential_states!($impl_type => $value_type, state_set);
    };
    ($impl_type:ty => $set_type:ty, $state_type:ty, $field_name:ident) => {
        impl $crate::properties::state::HasPotentialStates for $impl_type {
            $crate::impl_has_potential_states!(=> $set_type:ty, $state_type:ty, $field_name);
        }
    };
    (=> $set_type:ty, $state_type:ty, $field_name:ident) => {
        type State = $state_type;
        type StateSet = $set_type;

        fn potential_state(&self) -> &Self::StateSet {
            &self.$field_name
        }
    };
}
