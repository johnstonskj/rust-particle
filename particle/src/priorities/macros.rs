#[macro_export]
macro_rules! impl_has_priority {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_priority!($impl_type => $value_type, priority);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::priorities::HasPriority for $impl_type {
            $crate::impl_has_priority!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        type Priority = $value_type;

        fn with_priority(mut self, priority: Self::Priority) -> Self {
            self.$field_name = priority;
            self
        }

        fn current_priority(&self) -> Self::Priority {
            self.$field_name
        }
    };
}

#[macro_export]
macro_rules! impl_has_priority_mut {
    ($impl_type:ty => $value_type:ty) => {
        $crate::impl_has_priority_mut!($impl_type => $value_type, priority);
    };
    ($impl_type:ty => $value_type:ty, $field_name:ident) => {
        impl $crate::priorities::HasPriorityMut for $impl_type {
            $crate::impl_has_priority_mut!(=> $value_type, $field_name);
        }
    };
    (=> $value_type:ty, $field_name:ident) => {
        fn set_priority(&mut self, priority: Self::Priority) {
            self.$field_name = priority;
        }
    };
}
