#[macro_export]
macro_rules! impl_has_just_identifier {
    ($impl_type:ty => $value_type:ty { $( $init_name:ident : $init_value:expr ),* }) => {
        $crate::impl_has_just_identifier!($impl_type => $value_type, id);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident { $( $init_name:ident : $init_value:expr ),* }) => {
        impl $crate::identifiers::HasJustIdentifier for $impl_type {
            $crate::impl_has_just_identifier!(=> $value_type, $field_name { $( $init_name : $init_value ),* });
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Identifier = $value_type;

        fn new(id: Self::Identifier) -> Self {
            Self {
                $field_name: id,
                $(
                    $init_name : $init_value,
                )*
            }
        }

        fn id(&self) -> &Self::Identifier {
            &self.$field_name
        }
    };
}

#[macro_export]
macro_rules! impl_has_identifier {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_identifier!($impl_type => $value_type, id);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::identifiers::HasIdentifier for $impl_type {
            $crate::impl_has_identifier!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Identifier = $value_type;

        fn id(&self) -> &Self::Identifier {
            &self.$field_name
        }
    };
}

#[macro_export]
macro_rules! impl_has_optional_identifier {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_optional_identifier!($impl_type => $value_type, id);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::identifiers::HasOptionalIdentifier for $impl_type {
            $crate::impl_has_optional_identifier!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Identifier = $value_type;

        fn id(&self) -> Option<&Self::Identifier> {
            self.$field_name.as_ref()
        }

        fn set_id(&mut self, id: Self::Identifier) {
            self.$field_name = Some(id);
        }

        fn unset_id(&mut self) {
            self.$field_name = None;
        }
    };
}
