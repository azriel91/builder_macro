macro_rules! declare_struct_and_builder {
    // Implement struct and builder when all attributes have been filtered
    (
        meta: [ $( #[$STRUCT_META:meta] )* ],
        declaration: $BUILDER:ident => $STRUCT:ident
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
        $callback:ident,
        meta: [ $( #[$ITEM_META:meta] )* ],
        declaration: #[$NEXT_META:meta] $( $STRUCT_DEC:tt )+
    )
    =>
    {
        parse_item! {
            $callback,
            meta: [ #[$NEXT_META] $( #[$ITEM_META] )* ],
            declaration: $( $STRUCT_DEC )+
        }
    };

    (
        $callback:ident,
        meta: [ $( #[$ITEM_META:meta] )* ],
        declaration: $BUILDER:ident => $STRUCT:ident
    )
    =>
    {
        $callback! {
            meta: [ $( #[$ITEM_META] )* ],
            declaration: $BUILDER => $STRUCT
        }
    };
}

macro_rules! builder {
    // The way we determine visibility of the generated builder and struct is based on the pattern in:
    // https://github.com/rust-lang-nursery/lazy-static.rs/blob/v0.2.1/src/lib.rs

    (
        $( $STRUCT_DEC:tt )*
        // fields: { $( $FIELD_DEC:tt )* } $(,)*
        // $( assertions: { $( $ASSERTION:expr ),* $(,)* } )*
    )
    =>
    {
        parse_item! {
            declare_struct_and_builder,
            meta: [],
            declaration: $( $STRUCT_DEC )*
        }
    };
}

fn main() {
    builder! {
        OneBuilder => One
    }

    builder! {
        /// hello everyone
        MyStructBuilder => MyStruct
    }
}
