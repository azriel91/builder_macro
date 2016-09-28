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
        $(, assertions: { $( $ASSERTION:expr; )* } )*
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
        struct $BUILDER {
            $(
                $( #[$F_META] )*
                $F_NAME : Option<$F_TY>,
            )*
        }

        impl $BUILDER {
            /// Construct the builder
            pub fn new() -> $BUILDER {
                $BUILDER {
                    $(
                        $F_NAME : $F_DEFAULT
                    ),*
                }
            }

            /// Build the struct
            pub fn build(self) -> $STRUCT {
                $( let $F_NAME = self.$F_NAME.unwrap(); )*
                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $F_NAME : $F_NAME ),*
                }
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Specify a value for the $F_NAME field
                fn $F_NAME(mut self, value: $F_TY) -> Self {
                    self.$F_NAME = Some(value);
                    self
                }
            )*
        }
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
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        parse_item! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {},
            field_wip: { meta: [] },
            parser_wip: { $( $FIELD_SPEC )* }
            $(, assertions: { $( $ASSERTION; )* } )*
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
            #[$FIELD_WIP_NEXT_META:meta] $( $SPEC_TAIL:tt )+
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
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
                $( $SPEC_TAIL )+
            }
            $(, assertions: { $( $ASSERTION; )* } )*
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
            $( $SPEC_TAIL:tt )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
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
                $( $SPEC_TAIL )*
            }
            $(, assertions: { $( $ASSERTION; )* } )*
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
        $(, assertions: { $( $ASSERTION:expr; )* } )*
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
            $(, assertions: { $( $ASSERTION; )* } )*
        }
    };
}

macro_rules! builder {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    (
        $( $SPEC:tt )*
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
        FiveBuilder => Five {
            /// doc for i32
            something: i32 = Some(0),
        }
    }

    builder! {
        /// hello everyone
        SixBuilder => Six {
            /// doc for i32
            something: i32 = Some(0),
        },
        assertions: {
            assert!(something > -1);
        }
    }

    SixBuilder::new().build();
}
