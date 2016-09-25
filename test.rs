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
                $( #[$META] )*
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
            meta: [ #[$NEXT_META] $( #[$ITEM_META] )* ],
            spec: $( $SPEC )+
        }
    };

    // When we reach here, we have parsed all of the meta items for the struct.
    // Next we have to parse the meta items for each field.
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident {
            $(
                $FIELD_SPEC:tt
                $(, $FIELDS_SPEC_ADDITIONAL:tt )*
                $(,)*
            )*
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
                        meta: [],
                        spec: $FIELD_SPEC
                    }
                    $(,
                        {
                            meta: [],
                            spec: $( $FIELDS_SPEC_ADDITIONAL )*
                        }
                    )*
                )*
            }
        }
    };

    // Loop through each meta item in FIELD_SPEC, extract it and prepend it to FIELD_META
    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
                    meta: [ $( #[$FIELD_META:meta] )* ],
                    spec: #[$NEXT_FIELD_META:meta] $( $FIELD_SPEC:tt )*
                } $(,)*
            )*
        }
    )
    =>
    {
        yay
        // parse_item! {
        //     meta: [ $( #[$ITEM_META] )* ],
        //     spec: $BUILDER => $STRUCT,
        //     fields: {
        //         $(
        //             {
        //                 meta: [ #[$NEXT_FIELD_META] $( #[$FIELD_META] )* ],
        //                 spec: $( $FIELD_SPEC )*
        //             },
        //         )*
        //     }
        // }
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
        }
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
            something: i32 = Some(0)
        }
    }

    // builder! {
    //     /// hello everyone
    //     MyStructBuilder => MyStruct {
    //         /// doc for i32
    //         something: i32 = Some(0)
    //     }
    // }
}
