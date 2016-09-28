macro_rules! declare_struct_and_builder {
    // Implement struct and builder when all attributes have been filtered
    (
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
                    meta: [ $( #[$F_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
    )
    =>
    {
        $( #[$META] )*
        struct $STRUCT {
            $(
                $( #[$F_META] )*
                $F_NAME : $F_TY,
            )*
        }

        // stringify!($STRUCT)
        #[doc="Generated struct builder"]
        struct $BUILDER {}
    };
}

macro_rules! parse_item {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    // Loop through each meta item in SPEC, extract it and prepend it to ITEM_META
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: #[$NEXT_META:meta] $( $SPEC:tt )+
    )
    =>
    {
        parse_item! {
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
    // a_macro!($something, parse_item!($another_thing)); // fails because it doesn't attempt to evaluate parse_item!
    //

    // This macro adds 'fields:' around the block to make parsing easier
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident {
            $( $FIELD_SPEC:tt )*
        }
    )
    =>
    {
        parse_item! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {},
            field_wip: { meta: [] },
            parser_wip: { $( $FIELD_SPEC )* }
        }
    };

    // Now we have to attempt to wrap each field inside braces {}
    // This macro looks for meta tokens and extracts them into field_wip
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $( $FIELD_SPEC:tt )+
                },
            )*
        },
        field_wip: {
            meta: [ $( #[$FIELD_WIP_META:meta] )* ]
        },
        parser_wip: {
            #[$FIELD_WIP_NEXT_META:meta] $( $FIELD_TAIL:tt )+
        }
    )
    =>
    {
        parse_item! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {
                $(
                    {
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $( $FIELD_SPEC )+
                    },
                )*
            },
            field_wip: {
                meta: [ $( #[$FIELD_WIP_META] )* #[$FIELD_WIP_NEXT_META] ]
            },
            parser_wip: {
                $( $FIELD_TAIL )+
            }
        }
    };

    // When we reach here, the meta tokens for field_wip should have all been parsed
    // Therefore we should be able to match on the field_name: Type = Some(default), pattern
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
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
            $( $FIELD_TAIL:tt )*
        }
    )
    =>
    {
        parse_item! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {
                $(
                    {
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $( $FIELD_SPEC )+
                    },
                )*
                {
                    meta: [ $( #[$FIELD_WIP_META] )* ],
                    spec: $F_NAME: $F_TY = $F_DEFAULT
                },
            },
            field_wip: { meta: [] },
            parser_wip: {
                $( $FIELD_TAIL )*
            }
        }
    };

    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr $(,)*
                } $(,)*
            )*
        },
        field_wip: { meta: [] },
        parser_wip: {}
    )
    =>
    {
        declare_struct_and_builder! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {
                $(
                    {
                        meta: [ $( #[$FIELD_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
        }
    };
}

macro_rules! builder {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    (
        $( $SPEC:tt )*
        // $( assertions: { $( $ASSERTION:expr ),* $(,)* } )*
    )
    =>
    {
        parse_item! {
            meta: [],
            spec: $( $SPEC )*
        }
    };
}

fn main() {
    builder! {
        OneBuilder => One {
        }
    }

    builder! {
        TwoBuilder => Two {
            something: i32 = Some(0),
        }
    }

    builder! {
        ThreeBuilder => Three {
            /// here are some docs
            something: i32 = Some(0),
        }
    }

    builder! {
        FourBuilder => Four {
            something: i32 = Some(0),
            something_else: &'static str = Some("0"),
        }
    }

    builder! {
        /// hello everyone
        MyStructBuilder => MyStruct {
            /// doc for i32
            something: i32 = Some(0),
        }
    }
}
