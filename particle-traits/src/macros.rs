//!
//! Provides safe versions of macros from `particle-macros` without use of *pastey*.
//!

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ fn_with
// ------------------------------------------------------------------------------------------------

///
/// Generate a `with_` (*with underscore*) function with the conventional signatures used in builders
/// and builder fragments.
///
/// # Parameters
///
/// - `$doc_str`: Optional documentation string for the generated function.
/// - `$fn_name`: The name of the generated function.
/// - `$prop_name`: The name of the property parameter.
/// - `$prop_type`: The type of the property parameter.
/// - `#[optional]`: Indicates that the property is optional and should be wrapped in `Option`.
/// - `#[pseudoptional]`: Indicates that the property is pseudo-optional, meaning it may be treated
///   as optional in certain contexts but is not strictly wrapped in `Option`.
///
///
#[allow(unused_macro_rules)]
#[cfg_attr(all(doc, feature = "doc-macros"), macro_export)]
macro_rules! fn_with {
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[optional] $prop_type:path )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: Option<$prop_type>) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[optional] $prop_type:path )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: Option<$prop_type) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[optional] $prop_type:path ) => $base_fn_name:ident
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: Option<$prop_type>) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[optional] $prop_type:ty )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: Option<$prop_name>) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident $prop_name:ident : #[optional] $prop_type:ty
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: Option<$prop_name>) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident => $base_fn_name:ident $prop_name:ident : #[optional] $prop_type:ty
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: Option<$prop_name>) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $fn_name:ident $prop_name:ident : #[pseudoptional] $prop_type:path
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: Option<$prop_type>) -> Self fn_with!(@body $prop_name)
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    (
        $( $doc_str: literal => )?
        $fn_name:ident $prop_name:ident : #[pseudoptional] $prop_type:ty
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: Option<$prop_name>) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident => $base_fn_name:ident $prop_name:ident : #[pseudoptional] $prop_type:ty
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: Option<$prop_name>) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[iter] $prop_type:path )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: impl IntoIterator<Item = $prop_type>) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident $prop_name:ident : #[iter] $( $prop_type:ident )::+
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: impl IntoIterator<Item = $prop_type:path>) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident => $base_fn_name:ident $prop_name:ident : #[iter] $prop_type:path
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: impl IntoIterator<Item = $prop_type>) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[iter] $prop_type:ty )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: impl IntoIterator<Item = $prop_type>) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[iter] $prop_type:ty )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: impl IntoIterator<Item = $prop_type>) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : #[iter] $prop_type:ty ) => $base_fn_name:ident
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: impl IntoIterator<Item = $prop_type>) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:path )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: $prop_type) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:path )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: $prop_type) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:path ) => $base_fn_name:ident
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: $prop_type) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }

    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:ty )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: $prop_type) -> Self;
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:ty )
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(mut self, $prop_name: $prop_type) -> Self {
            fn_with!(@body self, $prop_name)
        }
    };
    (
        $( $doc_str: literal => )?
        $fn_name:ident ( $prop_name:ident : $prop_type:ty ) => $base_fn_name:ident
    ) => {
        $(
            #[doc = $doc_str]
        )?
        fn $fn_name(self, $prop_name: $prop_type) -> Self
        where
            Self: Sized,
        {
            fn_with!(@body self, $base_fn_name $prop_name)
        }
    };
    // ============================================================================================
    (@body $self:expr, $prop_name:ident) => {
            $self.$prop_name = $prop_name;
            $self
    };
    (@body $self:expr, $base_fn_name:ident $prop_name:ident) => {
            $self.$base_fn_name($prop_name)
   };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ builder_fragment
// ------------------------------------------------------------------------------------------------

///
/// Generate a builder fragment trait with optional default implementations for properties.
///
/// # Parameters
///
/// - `$trait_name`: The name of the trait to generate.
/// - `$fragment_trait`: The associated fragment trait to include.
/// - `default $default_fn_with($prop_name)`: Optional default implementation for a property.
/// - `{ ... }`: The body of the trait, containing additional methods or associated items.
///
#[allow(unused_macro_rules)]
#[cfg_attr(all(doc, feature = "doc-macros"), macro_export)]
macro_rules! builder_fragment {
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $trait_name1:ident impl $fragment_trait:ident
        $( default $default_fn_with:ident ( $prop_name:ident ) )?
        {
            $( $body:tt )*
        }
    ) => {
        $(
            #[doc = $doc_str]
        )?
        pub trait $trait_name1 {
            /// The $fragment_trait implementation to be included in the built object.
            type $fragment_trait: $fragment_trait;

            $(
                fn_with!{
                    $default_fn_with($prop_name : Self::$fragment_trait)
                }
            )?

            $(
                $body
            )*
        }
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    (
        $( $doc_str: literal => )?
        $trait_name2:ident $fragment_type:ident: $fragment_trait:ident
        $( default $default_fn_with:ident ( $prop_name:ident ) )?
        {
            $( $body:tt )*
        }
    ) => {
        $(
            #[doc = $doc_str]
        )?
        pub trait $trait_name2 {
            /// The $fragment_trait implementation to be included in the built object.
            type $fragment_type: $fragment_trait;

            $(
                fn_with!{
                    $default_fn_with($prop_name : Self::$fragment_type)
                }
            )?

            $(
                $body
            )*
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ has_immutable_trait
// ------------------------------------------------------------------------------------------------

///
/// Build a trait for immutable access to a property of an associated type.
///
/// # Generated Forms
///
/// Simple.
///
/// ```,ignore
/// /// This trait provides immutable access to the property of the associated
/// /// type [`Property`].
/// pub trait HasProperty {
///     type Property;
///
///     fn property(&self) -> &Self::Property;
/// }
/// ```
///
/// Optional.
///
/// ```,ignore
/// /// This trait provides immutable access to an optional property of the
/// /// associated type [`Property`].
/// pub trait HasOptionalProperty {
///     type Property;
///
///     fn property(&self) -> Option<&Self::Property>;
///     fn has_property(&self) -> bool;
/// }
/// ```
///
/// Collection.
///
/// ```,ignore
/// /// This trait provides immutable access to a collection property of the
/// /// associated type [`Property`].
/// pub trait HasProperties {
///     type Property;
///
///     fn properties(&self) -> impl Iterator<Item = &Self::Property>;
///     fn has_properties(&self) -> bool;
///     fn properties_count(&self) -> usize;
/// }
/// ```
///
/// # Syntax
///
/// ```bnf
/// has_immutable_trait! ::= simple_form | optional_form | iter_form
///
/// simple_form ::=
///     doc_str? trait_name "{"
///         doc_str? type_obj_name =>
///         doc_str? get_fn_name
///     "}"
///
/// optional_form ::=
///     "#[optional]"
///     doc_str? trait_name "{"
///         doc_str? type_obj_name =>
///         doc_str? get_fn_name
///         doc_str? has_fn_name
///
/// iter_form ::=
///     "#[iter]"
///     doc_str? trait_name "{"
///         doc_str? type_obj_name =>
///         doc_str? get_fn_name
///         doc_str? has_fn_name
///         doc_str? count_fn_name
///     "}"
/// ```
///
#[allow(unused_macro_rules)]
#[cfg_attr(all(doc, feature = "doc-macros"), macro_export)]
macro_rules! has_immutable_trait {
    // --------------------------------------------------------------------------------------------
    (
        $( $has_trait_doc: literal )? $has_trait:ident {
            $( $type_obj_doc: literal )? $type_obj_name:ident =>
            $( $prop_name_doc: literal )? $prop_name:ident
        }
    ) => {
        $( #[doc = $has_trait_doc] )?
        pub trait $has_trait {
            $( #[doc = $type_obj_doc] )?
            type $type_obj_name: $type_obj_name;

            $( #[doc = $prop_name_doc] )?
            fn $prop_name(&self) -> &Self::$type_obj_name;
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        #[optional]
        $( $has_trait_doc: literal )? $has_trait:ident {
            $( $type_obj_doc: literal )? $type_obj_name:ident =>
            $( $prop_name_doc: literal )? $prop_name:ident ,
            $( $has_fn_doc: literal )? $has_fn:ident
        }
    ) => {
        $( #[doc = $has_trait_doc] )?
        pub trait $has_trait {
            $( #[doc = $type_obj_doc] )?
            type $type_obj_name: $type_obj_name;

            $( #[doc = $prop_name_doc] )?
            fn $prop_name(&self) -> ::core::option::Option<&Self::$type_obj_name>;

            $( #[doc = $has_fn_doc] )?
            fn $has_fn(&self) -> bool;
        }
   };
    // --------------------------------------------------------------------------------------------
    (
        #[iter]
        $( $has_trait_doc:literal )? $has_trait:ident {
            $( $type_obj_doc:literal )? $type_obj_name:ident =>
            $( $prop_plural_name_doc:literal )? $prop_plural_name:ident ,
            $( $has_fn_doc:literal )? $has_fn:ident ,
            $( $count_fn_doc:literal )? $count_fn:ident
        }
    ) => {
        $( #[doc = $has_trait_doc] )?
        pub trait $has_trait {
            $( #[doc = $type_obj_doc] )?
            type $type_obj_name: $type_obj_name;

            $( #[doc = $prop_plural_name_doc] )?
            fn $prop_plural_name(&self) -> impl ::core::iter::Iterator<Item = &Self::$type_obj_name>;

            $( #[doc = $has_fn_doc] )?
            fn $has_fn(&self) -> bool;

            $( #[doc = $count_fn_doc] )?
            fn $count_fn(&self) -> usize;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ has_mutable_trait
// ------------------------------------------------------------------------------------------------

///
/// Build a trait for mutable access to a property of an associated type.
///
/// # Generated Forms
///
/// Simple.
///
/// ```,ignore
/// /// This trait provides mutable access to the property of the associated
/// /// type [`Property`].
/// pub trait HasPropertyMut {
///     type Property;
///
///     fn property_mut(&mut self) -> &mut Self::Property;
///
///     fn set_property(&mut self, property: Self::Property);
/// }
/// ```
///
/// Optional.
///
/// ```,ignore
/// /// This trait provides mutable access to an optional property of the
/// /// associated type [`Property`].
/// pub trait HasOptionalPropertyMut {
///     type Property;
///
///     fn property_mut(&mut self) -> Option<&mut Self::Property>;
///
///     fn set_property(&mut self, property: Option<Self::Property>);
///
///     fn unset_property(&mut self);
/// }
/// ```
///
/// Collection.
///
/// ```,ignore
/// /// This trait provides mutable access to a collection property of the
/// /// associated type [`Property`].
/// pub trait HasPropertiesMut {
///     type Property;
///
///     fn properties_mut(&mut self) -> impl Iterator<Item = &mut Self::Property>;
///
///     fn set_properties<I: IntoIterator<Item = Self::Property>>(&mut self, properties: I);
///         
///     fn add_property(&mut self, property: Self::Property);
///
///     fn extend_properties<I: IntoIterator<Item = Self::Property>>(&mut self, properties: I);
///
///     fn remove_property(&mut self, property: &Self::Property) -> Option<Self::Property>;
///
///     fn clear_properties(&mut self);
/// }
/// ```
///
/// # Syntax
///
/// ```bnf
/// has_immutable_trait! ::= simple_form | optional_form | iter_form
///
/// simple_form ::=
///     doc_str? trait_name: immutable_trait_name"{"
///         property_name: type_obj_name =>
///         doc_str? set_fn_name
///         ( doc_str? get_mut_fn_name )?
///     "}"
///
/// optional_form ::=
///     "#[optional]"
///     doc_str? trait_name: immutable_trait_name "{"
///         property_name: type_obj_name =>
///         doc_str? set_fn_name
///         doc_str? unset_fn_name
///         ( doc_str? get_mut_fn_name )?
///
/// iter_form ::=
///     "#[iter]"
///     doc_str? trait_name: immutable_trait_name "{"
///         property_name: type_obj_name =>
///         doc_str? set_fn_name
///         doc_str? add_fn_name
///         doc_str? extend_fn_name
///         doc_str? remove_fn_name
///         doc_str? clear_fn_name
///         ( doc_str? get_mut_fn_name )?
///     "}"
/// ```
///
#[allow(unused_macro_rules)]
#[cfg_attr(all(doc, feature = "doc-macros"), macro_export)]
macro_rules! has_mutable_trait {
    // --------------------------------------------------------------------------------------------
    (

        $( $has_mut_trait_doc:literal )? $has_mut_trait:ident : $has_trait:ident {
            $prop_name:ident : $type_obj_name:ident =>
            $( $set_fn_doc:literal )? $set_fn:ident
            $(
                , $( $get_mut_fn_doc:literal )? $get_mut_fn:ident
            )?
        }
    ) => {
        $( #[doc = $has_mut_trait_doc] )?
        pub trait $has_mut_trait: $has_trait {
            $(
                $( #[doc = $get_mut_fn_doc] )?
                fn $get_mut_fn(&mut self) -> &mut Self::$type_obj_name;
            )?

            $( #[doc = $set_fn_doc] )?
            fn $set_fn(&mut self, $prop_name: Self::$type_obj_name);
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        #[optional]
        $( $has_mut_trait_doc: literal )? $has_mut_trait:ident : $has_trait:ident {
            $prop_name:ident : $type_obj_name:ident =>
            $( $set_fn_doc:literal)? $set_fn:ident ,
            $( $unset_fn_doc:literal )? $unset_fn:ident
            $(
                , $( $get_mut_fn_doc:literal )? $get_mut_fn:ident
            )?
        }
    ) => {
        $( #[doc = $has_mut_trait_doc] )?
        pub trait $has_mut_trait: $has_trait {
            $(
                $( #[doc = $get_mut_fn_doc] )?
                fn $get_mut_fn(&mut self) -> ::core::option::Option<&mut Self::$type_obj_name>;
            )?

            $( #[doc = $set_fn_doc] )?
            fn $set_fn(&mut self, $prop_name: ::core::option::Option<Self::$type_obj_name>);

            $( #[doc = $unset_fn_doc] )?
            fn $unset_fn(&mut self);
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        #[iter]
        $( $has_mut_trait_doc:literal )? $has_mut_trait:ident : $has_trait:ident {
            $prop_plural:ident : $type_obj_name:ident =>
            $( $set_fn_doc:literal )? $set_fn:ident ,
            $( $add_fn_doc:literal )? $add_fn:ident ,
            $( $extend_fn_doc:literal )? $extend_fn:ident ,
            $( $remove_fn_doc:literal )? $remove_fn:ident ,
            $( $clear_fn_doc:literal )? $clear_fn:ident
            $(
                , $( $iter_mut_fn_doc:literal )? $iter_mut_fn:ident
            )?
        }
    ) => {
        $( #[doc = $has_mut_trait_doc] )?
        pub trait $has_mut_trait: $has_trait {
            $(
                $( #[doc = $iter_mut_fn_doc] )?
                fn $iter_mut_fn(&mut self) -> impl ::core::iter::Iterator<Item = &mut Self::$type_obj_name>;
            )?

            $( #[doc = $set_fn_doc] )?
            fn $set_fn<I: ::core::iter::IntoIterator<Item = Self::$type_obj_name>>(&mut self, $prop_plural: I);

            $( #[doc = $add_fn_doc] )?
            fn $add_fn(&mut self, $prop_plural: Self::$type_obj_name);

            $( #[doc = $extend_fn_doc] )?
            fn $extend_fn<I: ::core::iter::IntoIterator<Item = Self::$type_obj_name>>(&mut self, $prop_plural: I);

            $( #[doc = $remove_fn_doc] )?
            fn $remove_fn(&mut self, $prop_plural: &Self::$type_obj_name) -> ::core::option::Option<Self::$type_obj_name>;

            $( #[doc = $clear_fn_doc] )?
            fn $clear_fn(&mut self);
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ has_traits
// ------------------------------------------------------------------------------------------------

///
/// This is a shortcut macro for defining both immutable and mutable traits for a given property
/// object/trait.
///
/// As such, this macro attempts to follow the syntax and structure of `has_immutable_trait!` and
/// `has_mutable_trait!` but reduces redundancy where possible.
///
#[allow(unused_macro_rules)]
#[cfg_attr(all(doc, feature = "doc-macros"), macro_export)]
macro_rules! has_traits {
    // --------------------------------------------------------------------------------------------
    (
        $( $type_obj_doc:literal )? $type_obj_name:ident =>
        $( $has_trait_doc:literal )? $has_trait:ident {
            $( $prop_name_doc:literal )? $prop_name:ident
        }
        $( $has_mut_trait_doc:literal )? $has_mut_trait:ident {
            $( $set_fn_doc:literal )? $set_fn:ident
            $(
              ,  $( $get_mut_fn_doc:literal )? $get_mut_fn:ident
            )?
        }
    ) => {
        has_immutable_trait! {
            $( $has_trait_doc )? $has_trait {
                $( $type_obj_doc )? $type_obj_name =>
                $( $prop_name_doc )? $prop_name
            }
        }
        // ----------------------------------------------------------------------------------------
        has_mutable_trait! {
            $( $has_mut_trait_doc )? $has_mut_trait : $has_trait {
                $prop_name: $type_obj_name =>
                $( $set_fn_doc )? $set_fn
                $( , $( $get_mut_fn_doc )? $get_mut_fn  )?
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        #[optional] $( $type_obj_doc:literal )? $type_obj_name:ident =>
        $( $has_trait_doc:literal )? $has_trait:ident {
            $( $prop_name_doc:literal )? $prop_name:ident ,
            $( $has_fn_doc:literal )? $has_fn:ident
        }
        $( $has_mut_trait_doc:literal )? $has_mut_trait:ident {
            $( $set_fn_doc:literal )? $set_fn:ident ,
            $( $unset_fn_doc:literal )? $unset_fn:ident
            $(
                , $( $get_mut_fn_doc:literal => )? $get_mut_fn:ident
            )?
        }
    ) => {
        has_immutable_trait! {
            #[optional] $( $has_trait_doc )? $has_trait {
                $( $type_obj_doc )? $type_obj_name =>
                $( $prop_name_doc )? $prop_name , $( $has_fn_doc )? $has_fn
            }
        }
        // ----------------------------------------------------------------------------------------
        has_mutable_trait! {
            #[optional] $( $has_mut_trait_doc )? $has_mut_trait : $has_trait {
                $prop_name: $type_obj_name =>
                $( $set_fn_doc )? $set_fn ,
                $( $unset_fn_doc )? $unset_fn
                $( , $( $get_mut_fn_doc )? $get_mut_fn )?
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        #[iter] $( $type_obj_doc:literal )? $type_obj_name:ident =>
        $( $has_trait_doc:literal )? $has_trait:ident {
            $( $prop_plural_doc:literal )? $prop_plural:ident ,
            $( $has_fn_doc:literal )? $has_fn:ident ,
            $( $count_fn_doc:literal )? $count_fn:ident
        }
        $( $has_mut_trait_doc:literal )? $has_mut_trait:ident {
            $( $set_fn_doc:literal )? $set_fn:ident ,
            $( $add_fn_doc:literal )? $add_fn:ident ,
            $( $extend_fn_doc:literal )? $extend_fn:ident ,
            $( $remove_fn_doc:literal )? $remove_fn:ident ,
            $( $clear_fn_doc:literal )? $clear_fn:ident
            $(
                , $( $iter_mut_fn_doc:literal )? $iter_mut_fn:ident
            )?
        }
    ) => {
        has_immutable_trait! {
            #[iter] $( $has_trait_doc )? $has_trait {
                $( $type_obj_doc )? $type_obj_name =>
                $( $prop_plural_doc )? $prop_plural ,
                $( $has_fn_doc )? $has_fn ,
                $( $count_fn_doc )? $count_fn
            }
        }
        // ----------------------------------------------------------------------------------------
        has_mutable_trait! {
            #[iter] $( $has_mut_trait_doc )? $has_mut_trait : $has_trait {
                $prop_plural: $type_obj_name =>
                $( $set_fn_doc )? $set_fn ,
                $( $add_fn_doc )? $add_fn ,
                $( $extend_fn_doc )? $extend_fn ,
                $( $remove_fn_doc )? $remove_fn ,
                $( $clear_fn_doc )? $clear_fn
                $( , $( $iter_mut_fn_doc )? $iter_mut_fn  )?
            }
        }
    };
}
