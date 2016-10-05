#[doc(hidden)]
#[macro_export]
macro_rules! parse_struct {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    // Loop through each meta item in SPEC, extract it and prepend it to ITEM_META
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: #[$NEXT_META:meta] $( $SPEC:tt )+
    )
    =>
    {
        parse_struct! {
            meta: [ $( #[$ITEM_META] )* #[$NEXT_META] ],
            spec: $( $SPEC )+
        }
    };

    // When we reach here, we have parsed all of the meta items for the struct.
    // Next we have to extract the tokens for each field into a block, then parse the meta items for each field.
    // We have to do this because the rust compiler does not allow us to use a macro within the struct body:
    //
    // struct Something {
    //     parse_fields!( $( $FIELD_SPEC )* ); // compilation failure
    // }
    //
    // It also does not allow us to call a macro as part of another macro:
    //
    // // fails because it doesn't attempt to evaluate parse_struct!
    // a_macro!($something, parse_struct!($another_thing));
    //

    // This macro adds additional blocks to make parsing easier
    // We match on 'pub' in case the struct and builder should be public
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: pub $BUILDER:ident $MODE:tt $STRUCT:ident {
            $( $FIELD_SPEC:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            vis: [ pub ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {},
            field_wip: { meta: [] },
            parser_wip: { $( $FIELD_SPEC )* }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
    // We must have the private scope match happen after the rule for pub scope.
    // This is because if we have it the other way around, the following happens:
    //
    // * $BUILDER:ident matches `pub`
    // * $MODE:tt matches the builder name
    // * $STRUCT:ident attempts to match the -> or => arrow and fails
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident {
            $( $FIELD_SPEC:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            vis: [],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {},
            field_wip: { meta: [] },
            parser_wip: { $( $FIELD_SPEC )* }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };

    // Now we have to attempt to wrap each field inside braces {}
    // This macro looks for meta tokens and extracts them into field_wip
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $( $FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            #[$FIELD_WIP_NEXT_META:meta] $( $SPEC_TAIL:tt )+
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $( $FIELD_SPEC )+
                    },
                )*
            },
            field_wip: {
                meta: [ $( #[$FIELD_WIP_META] )* #[$FIELD_WIP_NEXT_META] ]
            },
            parser_wip: {
                $( $SPEC_TAIL )+
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };

    // When we reach here, the meta tokens for field_wip should have all been parsed
    // Therefore we should be able to match on the [pub] field_name: Type = Some(default), pattern
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $( $FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr,
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $( $FIELD_SPEC )+
                    },
                )*
                {
                    vis: [],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = $F_DEFAULT
                },
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
    // public field
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $( $FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            pub $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr,
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $( $FIELD_SPEC )+
                    },
                )*
                {
                    vis: [ pub ],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = $F_DEFAULT
                },
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };

    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr $(,)*
                } $(,)*
            )*
        },
        field_wip: { meta: [] },
        parser_wip: {}
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        declare_struct_and_builder! {
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
}
