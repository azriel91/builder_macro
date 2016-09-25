macro_rules! declare_struct_and_builder {
    // Implement struct and builder when all attributes have been filtered
    (
        meta: [ $( #[$STRUCT_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident
    )
    =>
    {
        $( #[$STRUCT_META] )*
        struct $STRUCT {}

        // stringify!($STRUCT)
        #[doc="Generated struct builder"]
        struct $BUILDER {}
    };
}

macro_rules! parse_item {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

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

    (
        meta: [ $( #[$ITEM_META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident $BODY:block
    )
    =>
    {
        declare_struct_and_builder! {
            meta: [ $( #[$ITEM_META] )* ],
            spec: $BUILDER => $STRUCT
        }
    };
}

macro_rules! builder {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    (
        $( $SPEC:tt )*
        // fields: { $( $FIELD_DEC:tt )* } $(,)*
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
        OneBuilder => One {}
    }

    builder! {
        TwoBuilder => Two {
            something: i32 = Some(0)
        }
    }

    builder! {
        /// hello everyone
        MyStructBuilder => MyStruct {
            /// doc for i32
            something: i32 = Some(0)
        }
    }
}
