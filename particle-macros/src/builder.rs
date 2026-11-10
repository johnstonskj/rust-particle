///
/// TBD
/// 
/// # Syntax
/// 
/// ```bnf
/// fn_with! ::= 
///     $vis 
///     $prop_name ":" 
///     ( ( "optional" | "pseudoptional" | "iter" ) "," )?
///     ( $prop_type  | $type_path )
///     ( 
///       ( "=>" $base_fn )?
///     | "{}"
///     | ";"
///     )
/// ```
/// 
/// - `$vis`; visibility specifier
/// - `$prop_name`; name of the property itself
/// - `optional`; keyword denoting an optional property, uses `Option` in the function signature
/// - ` pseudoptional`; keyword denoting an optional property, no `Option` on signature, uses
///   `Some` internally
/// - `iter`; keyword denoting a multi-valued property, takes an iterator in the function signature
/// - `$prop_type` | `$prop_path`; the property's type
/// - `$base_fn`; another *with_* constructor to call.
/// 
#[macro_export]
macro_rules! fn_with {
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : optional $( $prop_type:ident )::+ ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, $prop_name: Option<$prop_type)::+>) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : optional $( $prop_type:ident )::+ => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: Option<$prop_type)::+>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : optional $prop_type:ty ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, $prop_name: Option<$prop_type>) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : optional $prop_type:ty => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: Option<$prop_type>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : pseudoptional $( $prop_type:ident )::+ => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: Option<$prop_type)::+>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
   (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : pseudoptional $prop_type:ty => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: Option<$prop_name>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : iter $( $prop_type:ident )::+ ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, $prop_name: impl IntoIterator<Item = $prop_type)::+>) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : iter $( $prop_type:ident )::+ => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: impl IntoIterator<Item = $prop_type)::+>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : iter $prop_type:ty ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, i$prop_name: mpl IntoIterator<Item = $prop_name>) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : iter $prop_type:ty => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](mut self, $prop_name: impl IntoIterator<Item = $prop_name>) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : $( $prop_type:ident )::+ ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, $prop_name: $prop_type)::+) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : $( $prop_type:ident )::+ => $base_fn:ident
    ) => {
        pastey::paste! {
            $vis fn [< with_ $prop_name >](mut self, $prop_name: $prop_type)::+) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : $prop_type:ty ;
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
            $vis fn [< with_ $prop_name >](self, $prop_name: $prop_type) -> Self;
        }
    };
    (
        $( $doc_str: literal => )?
        $vis:vis $prop_name:ident : $prop_type:ty => $base_fn:ident
    ) => {
        pastey::paste! {
            $(
                #[doc = $doc_str]
            )?
           $vis fn [< with_ $prop_name >](mut self, $prop_name: $prop_type) -> Self 
            where 
                Self: Sized,
            {
                self.$base_fn($prop_name)
            }
        }
    };
}


///
/// TBD
/// 
/// # Syntax
/// 
/// ```bnf
/// builder_fragment! ::=
///     ( $display_name "=>" )?
///     $vis ( $prop_name ":" )? 
///     ( ( impl $fragment_trait_name ) | $fragment_type_name )
/// ```
/// 
/// - `display_name`; inserted into default help text
/// - `$vis`
/// `$prop_name`; default is `$fragment_type_name:lower:snake_case`
/// `$fragment_trait_name`; name of the trait
/// `$fragment_type_name`; default is `$fragment_trait_name`
/// 
/// 
macro_rules! builder_fragment {
    (
        $( $display_name:literal => )?
        $vis:vis impl $fragment_trait:ident { 
            $( $body:tt )+ 
        }
    ) => {
        pastey::paste! {
            $(
                #[doc = "Provides a builder mix-in fragment for types that can have " $display_name " properties."]
            )?
            $vis trait [< $fragment_trait BuilderFragment>] {
                #[doc = "The " $display_name " implementation to be included in the built object."]
                type $fragment_trait: $fragment_trait;

                fn_with! {
                    $fragment_trait:lower:snake: Self::$fragment_trait;
                }

                $( 
                    $body
                )+
            }
        }
    };
    (
        $( $display_name:literal => )?
        $vis:vis $prop_name:ident : impl $fragment_trait:ident { 
            $( $body:tt )+ 
        }
    ) => {
        pastey::paste! {
            $(
                #[doc = "Provides a builder mix-in fragment for types that can have " $display_name ". properties"]
            )?
            $vis trait [< $fragment_trait BuilderFragment>] {
                #[doc = "The " $display_name " implementation to be included in the built object."]
                type $fragment_trait: $fragment_trait;

                fn_with! {
                    $prop_name: Self::$fragment_trait;
                }

                $( 
                    $body
                )+
            }
        }
    };
    (
        $( $display_name:literal => )?
        $vis:vis $fragment_type:ident { 
            $( $body:tt )+ 
        }
    ) => {
        pastey::paste! {
            $(
                #[doc = "Provides a builder mix-in fragment for types that can have " $display_name " properties."]
            )?
            $vis trait [< $fragment_trait BuilderFragment>] {
                #[doc = "The " $display_name " type to be included in the built object."]
                type $fragment_type;

                fn_with! {
                    $fragment_type:lower:snake: Self::$fragment_type;
                }

                $( 
                    $body
                )+
            }
        }
    };
    (
        $( $display_name:literal => )?
        $vis:vis $prop_name:ident : $fragment_type:ident { 
            $( $body:tt )+ 
        }
    ) => {
        pastey::paste! {
            $(
                #[doc = "Provides a builder mix-in fragment for types that can have " $display_name " properties."]
            )?
            $vis trait [< $fragment_trait BuilderFragment>] {
                #[doc = "The " $display_name " type to be included in the built object."]
                type $fragment_type;

                fn_with! {
                    $prop_name: Self::$fragment_type;
                }

                $( 
                    $body
                )+
            }
        }
    };
}