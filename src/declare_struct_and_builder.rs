#[doc(hidden)]
#[macro_export]
macro_rules! declare_struct_and_builder {
    // Implement struct and builder when all attributes have been filtered
    // Non-consuming builder variant
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident -> $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$F_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        declare_struct_and_builder! {
            MAKE TY,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$META] )* ],
            spec: $BUILDER -> $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$F_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
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
            pub fn build(&self) -> $STRUCT {
                $( let $F_NAME = self.$F_NAME.clone().unwrap(); )*
                $( $( $ASSERTION; )* )*

                $STRUCT {
                    $( $F_NAME : $F_NAME ),*
                }
            }

            $(
                // allow dead code because the user may be using the field default
                #[allow(dead_code)]
                /// Auto-generated setter
                pub fn $F_NAME(&mut self, value: $F_TY) -> &mut Self {
                    self.$F_NAME = Some(value);
                    self
                }
            )*
        }
    };

    // Consuming builder variant
    (
        vis: [ $( $VIS:ident )* ],
        meta: [ $( #[$META:meta] )* ],
        spec: $BUILDER:ident => $STRUCT:ident,
        fields: {
            $(
                {
                    vis: [ $( $FIELD_VIS:ident )* ],
                    meta: [ $( #[$F_META:meta] )* ],
                    spec: $F_NAME:ident: $F_TY:ty = $F_DEFAULT:expr
                } $(,)*
            )*
        }
        $(, assertions: { $( $ASSERTION:expr; )* } )*
    )
    =>
    {
        declare_struct_and_builder! {
            MAKE TY,
            vis: [ $( $VIS )* ],
            meta: [ $( #[$META] )* ],
            spec: $BUILDER => $STRUCT,
            fields: {
                $(
                    {
                        vis: [ $( $FIELD_VIS )* ],
                        meta: [ $( #[$F_META] )* ],
                        spec: $F_NAME: $F_TY = $F_DEFAULT
                    },
                )*
            }
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
                /// Auto-generated setter
                pub fn $F_NAME(mut self, value: $F_TY) -> Self {
                    self.$F_NAME = Some(value);
                    self
                }
            )*
        }
    };

    // Declare the struct and the builder
    (
        MAKE TY,
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

        // stringify!($STRUCT)
        #[doc="Generated struct builder"]
        $( $VIS )* struct $BUILDER {
            // builder fields shouldn't have to be visible
            $(
                $( #[$F_META] )*
                $F_NAME : Option<$F_TY>,
            )*
        }
    };
}
