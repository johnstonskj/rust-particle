#[macro_export]
macro_rules! impl_has_validation {
    ($impl_type:ty => $validator:expr) => {
        impl $crate::validation::HasValidation for $impl_type {
            $crate::impl_has_validation!(=> $validator);
        }
    };
    ($impl_type:ty => $is_valid:expr ; $validator:expr) => {
        impl $crate::validation::HasValidation for $impl_type {
            $crate::impl_has_validation!(=> $is_valid ; $validator);
        }
    };
    (=> $validator:expr) => {
        fn is_valid(&self) -> bool {
            self.validate().is_ok()
        }

        fn validate(&self) -> core::result::Result<(), $crate::validation::ValidationError> {
            ($validator)()
        }
    };
    (=> $is_valid:expr ; $validator:expr) => {
        fn is_valid(&self) -> bool {
            ($is_valid)()
        }

        fn validate(&self) -> core::result::Result<(), $crate::validation::ValidationError> {
            ($validator)()
        }
    };
}
