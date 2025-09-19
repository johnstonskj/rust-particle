#[macro_export]
macro_rules! impl_has_name {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_name!($impl_type => $value_type, name);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::names::HasName for $impl_type {
            $crate::impl_has_name!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Name = $value_type;

        fn name(&self) -> &Self::Name {
            &self.$field_name
        }
    };
}

#[macro_export]
macro_rules! impl_has_name_mut {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_name_mut!($impl_type => $value_type, name);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::names::HasNameMut for $impl_type {
            $crate::impl_has_name_mut!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        fn set_name(&mut self, name: Self::Name) {
            self.$field_name = name;
        }
    };
}

#[macro_export]
macro_rules! impl_has_optional_name {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_optional_name!($impl_type => $value_type, name);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::names::HasOptionalName for $impl_type {
            $crate::impl_has_optional_name!(=> $value_type, name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Name = $value_type;

        fn name(&self) -> core::option::Option<&Self::Name> {
            self.$field_name.as_ref()
        }

        fn set_name(&mut self, name: Self::Name) {
            self.$field_name = Some(name);
        }

        fn unset_name(&mut self) {
            self.$field_name = None;
        }
    };
}
