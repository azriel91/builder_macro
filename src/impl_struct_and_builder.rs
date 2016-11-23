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
        fields: {
            $(
                {
                    req: $FIELD_REQ:ident,
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                },
            )*
        }
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
                        req: $FIELD_REQ,
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
        }

        impl_builder! {
            purpose: $PURPOSE,
            variant: non_consuming,
            spec: $BUILDER -> $STRUCT,
            fields: {
                $(
                    {
                        req: $FIELD_REQ,
                        spec: $F_NAME: $F_TY = $F_DEFAULT
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
        fields: {
            $(
                {
                    req: $FIELD_REQ:ident,
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                },
            )*
        }
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
                        req: $FIELD_REQ,
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
        }

        impl_builder! {
            purpose: $PURPOSE,
            variant: consuming,
            spec: $BUILDER -> $STRUCT,
            fields: {
                $(
                    {
                        req: $FIELD_REQ,
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
}
