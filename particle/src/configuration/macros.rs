#[macro_export]
macro_rules! impl_has_configuration {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_configuration!($impl_type => $value_type, configuration);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::configuration::HasConfiguration for $impl_type {
            $crate::impl_has_configuration!(=> $value_type, $field_name ; $validator);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type ConfigValue = $value_type;

        fn configuration(&self) -> &Self::ConfigValue {
            &self.$field_name
        }

        fn configuration_mut(&mut self) -> &mut Self::ConfigValue {
            &mut self.$field_name
        }

        fn set_configuration(
            &mut self,
            configuration: Self::ConfigValue
        ) -> core::result::Result<(), $crate::error::Error> {
            self.validate_configuration(&configuration)?;
            self.$field_name = configuration;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! impl_has_optional_configuration {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_optional_configuration!($impl_type => $value_type, configuration);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::configuration::HasOptionalConfiguration for $impl_type {
            $crate::impl_has_optional_configuration!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type ConfigValue = $value_type;

        fn has_configuration(&self) -> bool {
            self.$field_name.is_some()
        }

        fn configuration(&self) -> core::option::Option<&Self::ConfigValue> {
            self.$field_name.as_ref()
        }

        fn configuration_mut(&mut self) -> core::option::Option<&mut Self::ConfigValue> {
            self.$field_name.as_mut()
        }

        fn set_configuration(
            &mut self,
            configuration: Self::ConfigValue
        ) -> core::result::Result<(), $crate::error::Error> {
            <self as $crate::validation::HasValidation>.validate()?;
            self.$field_name = Some(configuration);
            Ok(())
        }

        fn unset_configuration(&mut self) -> core::result::Result<(), $crate::error::Error> {
            self.$field_name = None;
            Ok(())
        }
    };
}
