#[doc(hidden)]
#[macro_export]
/// Implements the setters and build method for the consuming variant of the builder.
macro_rules! impl_builder {
    // Non-consuming
    (
        purpose: data,
        variant: non_consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $(
                {
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new() -> $BUILDER {
                $BUILDER {
                    $(
                        $F_NAME : $F_DEFAULT
                    ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            pub fn build(&self) -> Result<$STRUCT, &'static str> {
                $(
                    let $F_NAME = try!(self.$F_NAME.clone().ok_or(
                        concat!("Must pass argument for field: '", stringify!($F_NAME), "'") ));
                )*

                $(
                    use std::panic;
                    $(
                        try!(panic::catch_unwind(|| { $ASSERTION; }).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $F_NAME : $F_NAME ),*
                })
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $F_NAME(&mut self, value: $F_TY) -> &mut Self {
                    self.$F_NAME = Some(value);
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
        fields: {
            $(
                {
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            /// Construct the builder
            pub fn new() -> $BUILDER {
                $BUILDER {
                    $(
                        $F_NAME : $F_DEFAULT
                    ),*
                }
            }

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            #[allow(unused_mut)]
            pub fn build(self) -> Result<$STRUCT, &'static str> {
                $(
                    // mutability is necessary for assertions on trait fields to work, otherwise the
                    // compiler fails with unwind safety not being satisfied
                    let mut $F_NAME = try!(self.$F_NAME.ok_or(
                        concat!("Must pass argument for field: '", stringify!($F_NAME), "'") ));
                )*

                $(
                    use std::panic::{self, AssertUnwindSafe};
                    $(
                        try!(panic::catch_unwind(AssertUnwindSafe(|| { $ASSERTION; })).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $F_NAME : $F_NAME ),*
                })
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $F_NAME(mut self, value: $F_TY) -> Self {
                    self.$F_NAME = Some(value);
                    self
                }
            )*
        }
    };
}
