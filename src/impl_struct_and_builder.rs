#[doc(hidden)]
#[macro_export]
/// The purpose can be either `data` or `object`.
///
/// The purpose should be `data` when the generated `build()` method should return
/// `Result<Struct, &'static str>`. This should be used when the generated struct is to be
/// constructed from input at runtime.
///
/// The purpose should be `object` when the generated `build()` method should return
/// `Struct`. This should be used when the application should fail / panic if construction of the
/// struct fails.
macro_rules! impl_struct_and_builder {
    // Implement struct and builder when all attributes have been filtered
    // Non-consuming builder variant
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident -> $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        },
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        declare_structs! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$META] )* ],
            spec: $BUILDER -> $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $MAN_F_NAME: $MAN_F_TY = $MAN_F_DEFAULT
                    },
                )*
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $OPT_F_NAME: $OPT_F_TY = $OPT_F_DEFAULT
                    },
                )*
            }
        }

        impl_builder! {
            purpose: $PURPOSE,
            variant: non_consuming,
            spec: $BUILDER -> $STRUCT,
            mandatory_fields: {
                $(
                    {
                        spec: $MAN_F_NAME: $MAN_F_TY = $MAN_F_DEFAULT
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        spec: $OPT_F_NAME: $OPT_F_TY = $OPT_F_DEFAULT
                    },
                )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };

    // Consuming builder variant
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $MAN_F_NAME:ident: $MAN_F_TY:ty = $MAN_F_DEFAULT:expr
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $OPT_F_NAME:ident: $OPT_F_TY:ty = $OPT_F_DEFAULT:expr
                },
            )*
        },
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        declare_structs! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $MAN_F_NAME: $MAN_F_TY = $MAN_F_DEFAULT
                    },
                )*
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $OPT_F_NAME: $OPT_F_TY = $OPT_F_DEFAULT
                    },
                )*
            }
        }

        impl_builder! {
            purpose: $PURPOSE,
            variant: consuming,
            spec: $BUILDER -> $STRUCT,
            mandatory_fields: {
                $(
                    {
                        spec: $MAN_F_NAME: $MAN_F_TY = $MAN_F_DEFAULT
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        spec: $OPT_F_NAME: $OPT_F_TY = $OPT_F_DEFAULT
                    },
                )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
}
