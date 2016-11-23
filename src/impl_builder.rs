#[doc(hidden)]
#[macro_export]
/// Implements the setters and build method for the consuming variant of the builder.
macro_rules! impl_builder {
    (
        @constructor
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $( $FIELDS_SPEC:tt )*
        }
    )
    =>
    {
        impl_builder!(
            @constructor
            spec: $BUILDER -> $STRUCT,
            separator: [],
            params: [],
            assignments: [],
            fields: {
                $( $FIELDS_SPEC )*
            }
        );
    };

    // Declare parameters for constructor if it is mandatory
    // Specify the comma if declaring the first parameter, which may or may not be required, and
    // then subsequently always specify comma.
    //
    // Must skip over all non-required fields for constructor params.
    // Must not skip over any parameters in the assignment.
    (
        @constructor
        spec: $BUILDER:ident -> $STRUCT:ident,
        separator: [ $( $SEPARATOR:tt )* ],
        params: [ $( { $( $PARAMS:tt )* }, )* ],
        assignments: [ $( { $( $ASSIGNMENTS:tt )* }, )* ],
        fields: {
            {
                req: false,
                default: $FIELD_DEFAULT:expr,
                spec: $F_NAME:ident: $F_TY:ty
            },
            $( $FIELDS_SPEC:tt )*
        }
    )
    =>
    {
        impl_builder!(
            @constructor
            spec: $BUILDER -> $STRUCT,
            separator: [ $( $SEPARATOR )* ],
            params: [ $( { $( $PARAMS )* }, )* ],
            assignments: [ $( { $( $ASSIGNMENTS )* }, )* { $F_NAME: Some($FIELD_DEFAULT), }, ],
            fields: {
                $( $FIELDS_SPEC )*
            }
        );
    };
    (
        @constructor
        spec: $BUILDER:ident -> $STRUCT:ident,
        separator: [ $( $SEPARATOR:tt )* ],
        params: [ $( { $( $PARAMS:tt )* }, )* ],
        assignments: [ $( { $( $ASSIGNMENTS:tt )* }, )* ],
        fields: {
            {
                req: true,
                default: $FIELD_DEFAULT:expr,
                spec: $F_NAME:ident: $F_TY:ty
            },
            $( $FIELDS_SPEC:tt )*
        }
    )
    =>
    {
        impl_builder!(
            @constructor
            spec: $BUILDER -> $STRUCT,
            separator: [ , ],
            params: [ $( { $( $PARAMS )* }, )* { $( $SEPARATOR )* $F_NAME: $F_TY }, ],
            assignments: [ $( { $( $ASSIGNMENTS )* }, )* { $F_NAME: Some($F_NAME), }, ],
            fields: {
                $( $FIELDS_SPEC )*
            }
        );
    };
    (
        @constructor
        spec: $BUILDER:ident -> $STRUCT:ident,
        separator: [ $( $SEPARATOR:tt )* ],
        params: [ $( { $( $PARAMS:tt )* }, )* ],
        assignments: [ $( { $( $ASSIGNMENTS:tt )* }, )* ],
        fields: {}
    )
    =>
    {
        /// Construct the builder
        pub fn new( $( $( $PARAMS )* )* ) -> $BUILDER {
            $BUILDER {
                $( $( $ASSIGNMENTS )* )*
            }
        }
    };

    // Generate setter for non-mandatory fields
    (
        @setter
        variant: non_consuming,
        req: false,
        default: $FIELD_DEFAULT:expr,
        spec: $F_NAME:ident: $F_TY:ty
    ) => {
        // allow dead code because the user may be using the field default
        #[allow(dead_code)]
        /// Auto-generated setter
        pub fn $F_NAME(&mut self, value: $F_TY) -> &mut Self {
            self.$F_NAME = Some(value);
            self
        }
    };
    (
        @setter
        variant: consuming,
        req: false,
        default: $FIELD_DEFAULT:expr,
        spec: $F_NAME:ident: $F_TY:ty
    ) => {
        // allow dead code because the user may be using the field default
        #[allow(dead_code)]
        /// Auto-generated setter
        pub fn $F_NAME(mut self, value: $F_TY) -> Self {
            self.$F_NAME = Some(value);
            self
        }
    };
    (
        @setter
        variant: $VARIANT:ident,
        req: true,
        default: $FIELD_DEFAULT:expr,
        spec: $F_NAME:ident: $F_TY:ty
    ) => ();

    // Non-consuming
    (
        purpose: data,
        variant: non_consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $(
                {
                    req: $FIELD_REQ:ident,
                    default: $FIELD_DEFAULT:expr,
                    spec: $F_NAME:ident: $F_TY:ty
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            impl_builder!(
                @constructor
                spec: $BUILDER -> $STRUCT,
                fields: {
                    $(
                        {
                            req: $FIELD_REQ,
                            default: $FIELD_DEFAULT,
                            spec: $F_NAME: $F_TY
                        },
                    )*
                }
            );

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            pub fn build(&self) -> Result<$STRUCT, &'static str> {
                $( let $F_NAME = self.$F_NAME.clone().unwrap(); )*

                $(
                    use std::panic;
                    $(
                        try!(panic::catch_unwind(|| { $ASSERTION; }).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $F_NAME: $F_NAME ),*
                })
            }

            $(
                impl_builder!(
                    @setter
                    variant: non_consuming,
                    req: $FIELD_REQ,
                    default: $FIELD_DEFAULT,
                    spec: $F_NAME: $F_TY
                );
            )*
        }
    };
    (
        purpose: object,
        variant: non_consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $(
                {
                    req: $FIELD_REQ:ident,
                    default: $FIELD_DEFAULT:expr,
                    spec: $F_NAME:ident: $F_TY:ty
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            impl_builder!(
                @constructor
                spec: $BUILDER -> $STRUCT,
                fields: {
                    $(
                        {
                            req: $FIELD_REQ,
                            default: $FIELD_DEFAULT,
                            spec: $F_NAME: $F_TY
                        },
                    )*
                }
            );

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            pub fn build(&self) -> $STRUCT {
                $( let $F_NAME = self.$F_NAME.clone().unwrap(); )*

                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $F_NAME: $F_NAME ),*
                }
            }

            $(
                impl_builder!(
                    @setter
                    variant: non_consuming,
                    req: $FIELD_REQ,
                    default: $FIELD_DEFAULT,
                    spec: $F_NAME: $F_TY
                );
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
                    req: $FIELD_REQ:ident,
                    default: $FIELD_DEFAULT:expr,
                    spec: $F_NAME:ident: $F_TY:ty
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            impl_builder!(
                @constructor
                spec: $BUILDER -> $STRUCT,
                fields: {
                    $(
                        {
                            req: $FIELD_REQ,
                            default: $FIELD_DEFAULT,
                            spec: $F_NAME: $F_TY
                        },
                    )*
                }
            );

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            #[allow(unused_mut)]
            pub fn build(self) -> Result<$STRUCT, &'static str> {
                // mutability is necessary for assertions on trait fields to work, otherwise the
                // compiler fails with unwind safety not being satisfied
                $( let mut $F_NAME = self.$F_NAME.unwrap(); )*

                $(
                    use std::panic::{self, AssertUnwindSafe};
                    $(
                        try!(panic::catch_unwind(AssertUnwindSafe(|| { $ASSERTION; })).or(
                            Err(concat!("assertion failed: '", stringify!($ASSERTION), "'")) ) );
                    )*
                )*

                Ok($STRUCT {
                    $( $F_NAME: $F_NAME ),*
                })
            }

            $(
                impl_builder!(
                    @setter
                    variant: consuming,
                    req: $FIELD_REQ,
                    default: $FIELD_DEFAULT,
                    spec: $F_NAME: $F_TY
                );
            )*
        }
    };
    (
        purpose: object,
        variant: consuming,
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $(
                {
                    req: $FIELD_REQ:ident,
                    default: $FIELD_DEFAULT:expr,
                    spec: $F_NAME:ident: $F_TY:ty
                },
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl $BUILDER {
            impl_builder!(
                @constructor
                spec: $BUILDER -> $STRUCT,
                fields: {
                    $(
                        {
                            req: $FIELD_REQ,
                            default: $FIELD_DEFAULT,
                            spec: $F_NAME: $F_TY
                        },
                    )*
                }
            );

            // Nested macro call should be stable for format!
            // https://github.com/rust-lang/rust/blob/1.12.0/src/libsyntax_ext/format.rs#L684-L687
            /// Build the struct
            #[allow(unused_mut)]
            pub fn build(self) -> $STRUCT {
                // mutability is necessary for assertions on trait fields to work, otherwise the
                // compiler fails with unwind safety not being satisfied
                $( let mut $F_NAME = self.$F_NAME.unwrap(); )*

                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $F_NAME: $F_NAME ),*
                }
            }

            $(
                impl_builder!(
                    @setter
                    variant: consuming,
                    req: $FIELD_REQ,
                    default: $FIELD_DEFAULT,
                    spec: $F_NAME: $F_TY
                );
            )*
        }
    };
}
