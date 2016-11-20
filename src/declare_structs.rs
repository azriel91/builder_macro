#[macro_export]
/// Declares the type struct and its corresponding builder struct.
macro_rules! declare_structs {
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident $MODE:tt $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$F_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
    )
    =>
    {
        $( #[$META] )*
        $( $VIS )* struct $STRUCT {
            $(
                $( #[$F_META] )*
                $( $FIELD_VIS )* $F_NAME : $F_TY,
            )*
        }

        // Unfortunately we cannot make the docs specific to the struct
        // e.g. passing stringify!($STRUCT)
        // See https://github.com/rust-lang/rust/issues/12404#issuecomment-35557322
        /// Auto-generated builder
        $( $VIS )* struct $BUILDER {
            // builder fields shouldn't have to be visible
            $(
                $( #[$F_META] )*
                $F_NAME : Option<$F_TY>,
            )*
        }
    };
}
