#[doc(hidden)]
#[macro_export]
/// Implements the setters and build method for the consuming variant of the builder.
macro_rules! impl_builder {
    // Non-consuming
    (
        purpose: data,
        variant: non_consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new($( $MAN_F_NAME: $MAN_F_TY ),*) -> $BUILDER {
                $BUILDER {
                    $( $MAN_F_NAME: Some($MAN_F_NAME) ),*
                    $( $OPT_F_NAME: $OPT_F_DEFAULT ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            pub fn build(&self) -> Result<$STRUCT, &'static str> {
                $( let $MAN_F_NAME = self.$MAN_F_NAME.clone().unwrap(); )*
                $( let $OPT_F_NAME = self.$OPT_F_NAME.clone().unwrap(); )*

                $(
                    use std::panic;
                    $(
                        try!(panic::catch_unwind(|| { $ASSERTION; }).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $MAN_F_NAME: $MAN_F_NAME ),*
                    $( $OPT_F_NAME: $OPT_F_NAME ),*
                })
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $OPT_F_NAME(&mut self, value: $OPT_F_TY) -> &mut Self {
                    self.$OPT_F_NAME = Some(value);
                    self
                }
            )*
        }
    };
    (
        purpose: object,
        variant: non_consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new($( $MAN_F_NAME: $MAN_F_TY ),*) -> $BUILDER {
                $BUILDER {
                    $( $MAN_F_NAME: Some($MAN_F_NAME) ),*
                    $( $OPT_F_NAME: $OPT_F_DEFAULT ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            pub fn build(&self) -> $STRUCT {
                $( let $MAN_F_NAME = self.$MAN_F_NAME.clone().unwrap(); )*
                $( let $OPT_F_NAME = self.$OPT_F_NAME.clone().unwrap(); )*

                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $MAN_F_NAME: $MAN_F_NAME ),*
                    $( $OPT_F_NAME: $OPT_F_NAME ),*
                }
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $OPT_F_NAME(&mut self, value: $OPT_F_TY) -> &mut Self {
                    self.$OPT_F_NAME = Some(value);
                    self
                }
            )*
        }
    };

    // Consuming variant
    (
        purpose: data,
        variant: consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new($( $MAN_F_NAME: $MAN_F_TY ),*) -> $BUILDER {
                $BUILDER {
                    $( $MAN_F_NAME: Some($MAN_F_NAME) ),*
                    $( $OPT_F_NAME: $OPT_F_DEFAULT ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            #[allow(unused_mut)]
            pub fn build(self) -> Result<$STRUCT, &'static str> {
                // mutability is necessary for assertions on trait fields to work, otherwise the
                // compiler fails with unwind safety not being satisfied
                $( let mut $MAN_F_NAME = self.$MAN_F_NAME.unwrap(); )*
                $( let mut $OPT_F_NAME = self.$OPT_F_NAME.unwrap(); )*

                $(
                    use std::panic::{self, AssertUnwindSafe};
                    $(
                        try!(panic::catch_unwind(AssertUnwindSafe(|| { $ASSERTION; })).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $MAN_F_NAME: $MAN_F_NAME ),*
                    $( $OPT_F_NAME: $OPT_F_NAME ),*
                })
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $OPT_F_NAME(mut self, value: $OPT_F_TY) -> Self {
                    self.$OPT_F_NAME = Some(value);
                    self
                }
            )*
        }
    };
    (
        purpose: object,
        variant: consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new($( $MAN_F_NAME: $MAN_F_TY ),*) -> $BUILDER {
                $BUILDER {
                    $( $MAN_F_NAME: Some($MAN_F_NAME) ),*
                    $( $OPT_F_NAME: $OPT_F_DEFAULT ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            #[allow(unused_mut)]
            pub fn build(self) -> $STRUCT {
                // mutability is necessary for assertions on trait fields to work, otherwise the
                // compiler fails with unwind safety not being satisfied
                $( let mut $MAN_F_NAME = self.$MAN_F_NAME.unwrap(); )*
                $( let mut $OPT_F_NAME = self.$OPT_F_NAME.unwrap(); )*

                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $MAN_F_NAME: $MAN_F_NAME ),*
                    $( $OPT_F_NAME: $OPT_F_NAME ),*
                }
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $OPT_F_NAME(mut self, value: $OPT_F_TY) -> Self {
                    self.$OPT_F_NAME = Some(value);
                    self
                }
            )*
        }
    };
}
