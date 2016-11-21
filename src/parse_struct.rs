#[doc(hidden)]
#[macro_export]
macro_rules! parse_struct {
    // The way we determine visibility of the generated builder and struct is based on the pattern
    // in: https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    // Loop through each meta item in SPEC, extract it and prepend it to ITEM_META
    (
        purpose: $PURPOSE:ident,
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: #[$NEXT_META:meta] $( $SPEC:tt )+
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            meta: [ $( #[$ITEM_META] )* #[$NEXT_META] ],
            spec: $( $SPEC )+
        }
    };

    // When we reach here, we have parsed all of the meta items for the struct.
    // Next we have to extract the tokens for each field into a block, then parse the meta items for
    // each field. We have to do this because the rust compiler does not allow us to use a macro
    // within the struct body:
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
        purpose: $PURPOSE:ident,
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: pub $BUILDER:ident $MODE:tt $STRUCT:ident {
            $( $FIELD_SPEC:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [ pub ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {},
            optional_fields: {},
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
        purpose: $PURPOSE:ident,
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident {
            $( $FIELD_SPEC:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {},
            optional_fields: {},
            field_wip: { meta: [] },
            parser_wip: { $( $FIELD_SPEC )* }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };

    // Now we have to attempt to wrap each field inside braces {}
    // This macro looks for meta tokens and extracts them into field_wip
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
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
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
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
    // Mandatory field
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            $F_NAME:ident: $F_TY:ty = None,
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
                {
                    vis: [],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = None
                },
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
                    },
                )*
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
    // Optional field
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            $F_NAME:ident: $F_TY:ty = Some($F_DEFAULT:expr),
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
                    },
                )*
                {
                    vis: [],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = Some($F_DEFAULT)
                },
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
    // public mandatory field
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            pub $F_NAME:ident: $F_TY:ty = None,
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
                {
                    vis: [ pub ],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = None
                },
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
                    },
                )*
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
    // public optional field
    (
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            pub $F_NAME:ident: $F_TY:ty = Some($F_DEFAULT:expr),
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_struct! {
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
                    },
                )*
                {
                    vis: [ pub ],
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = Some($F_DEFAULT)
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
        purpose: $PURPOSE:ident,
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        mandatory_fields: {
            $(
                {
                    vis: [ $( $MAN_FIELD_VIS:ident )* ],
                    meta: [ $( #[$MAN_FIELD_META:meta] )* ],
                    spec: $( $MAN_FIELD_SPEC:tt )+
                },
            )*
        },
        optional_fields: {
            $(
                {
                    vis: [ $( $OPT_FIELD_VIS:ident )* ],
                    meta: [ $( #[$OPT_FIELD_META:meta] )* ],
                    spec: $( $OPT_FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: { meta: [] },
        parser_wip: {}
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        impl_struct_and_builder! {
            purpose: $PURPOSE,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER $MODE $STRUCT,
            mandatory_fields: {
                $(
                    {
                        vis: [ $( $MAN_FIELD_VIS )* ],
                        meta: [ $( #[$MAN_FIELD_META] )* ],
                        spec: $( $MAN_FIELD_SPEC )+
                    },
                )*
            },
            optional_fields: {
                $(
                    {
                        vis: [ $( $OPT_FIELD_VIS )* ],
                        meta: [ $( #[$OPT_FIELD_META] )* ],
                        spec: $( $OPT_FIELD_SPEC )+
                    },
                )*
            },
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
}
