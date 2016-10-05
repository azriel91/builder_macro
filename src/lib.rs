#![deny(missing_docs)]

//! This crate contains a `builder!` macro to declare a struct and a corresponding builder.
//!
//! The macro is inspired from [jadpole/builder-macro][1], and is designed to
//! remove duplication of field declaration, as well as generating appropriate setter methods.
//!
//! Include the macro inside your crate's `lib.rs` or `main.rs`.
//!
//! ```rust
//! #[macro_use]
//! extern crate builder_macro;
//! #
//! # fn main() {} // necessary to allow doc test to pass
//! ```
//!
//! The simplest usage format of the builder macro is:
//!
//! ```rust,ignore
//! builder!(BuilderName => StructName {
//!     fieldname: Type = Some(default_value), // or None if there is no sensible default
//! });
//! ```
//!
//! The above will generate a module private struct and builder with a single private field.
//!
//! The full macro usage format is:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! // We declare the builder insider a module simply to demonstrate scope
//! mod inner {
//!     builder! {
//!         /// StructName is an example struct.
//!         /// These docs are copied over to the generated struct.
//!         pub BuilderName => StructName {
//!             /// a_field is an i32 which must be between 0 and 100 inclusive
//!             // the trailing comma is mandatory due to how the macro is parsed
//!             pub a_field: i32 = Some(50),
//!
//!             // None means no default value, a value must be specified when building
//!             // meta attributes are copied over to the struct's fields
//!             #[allow(dead_code)]
//!             a_private_field: &'static str = None,
//!         }, assertions: {
//!             assert!(a_field >= 0);
//!             assert!(a_field <= 100);
//!             // Yes you can assert on private fields
//!             assert!(!a_private_field.is_empty());
//!         }
//!     }
//! }
//!
//! let my_struct = inner::BuilderName::new()
//!                     .a_private_field("I must set this to a non-empty string")
//!                     .build();
//! assert_eq!(50, my_struct.a_field);
//! # }
//! ```
//!
//! # Examples
//!
//! Generate a builder and struct with module private visibility:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! builder!(MyStructBuilder => MyStruct {
//!     field_i32: i32 = Some(123),
//!     field_str: &'static str = Some("abc"),
//! });
//!
//! let my_struct = MyStructBuilder::new()
//!     .field_i32(456)
//!     .build();
//! assert_eq!(my_struct.field_i32, 456);
//! assert_eq!(my_struct.field_str, "abc"); // uses default
//! # }
//! ```
//!
//! Generate a builder and struct with public visibility:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! mod inner {
//!     builder!(pub MyStructBuilder => MyStruct {
//!         pub field_i32: i32 = Some(123),
//!         field_str: &'static str = Some("abc"),
//!     });
//! }
//!
//! let my_struct = inner::MyStructBuilder::new()
//!     .field_i32(456)
//!     .build();
//! assert_eq!(my_struct.field_i32, 456);
//!
//! // The next line will fail compilation if uncommented as field_str is private
//! // assert_eq!(my_struct.field_str, "abc");
//! # }
//! ```
//!
//! [1]: http://jadpole.github.io/rust/builder-macro
//!

#[macro_use]
mod declare_struct_and_builder;
#[macro_use]
mod parse_struct;

#[macro_export]
/// Macro to declare a struct and a corresponding builder. See [the module documentation](index.html) for more.
macro_rules! builder {
    ( $( $SPEC:tt )* )
    =>
    {
        parse_struct! {
            meta: [],
            spec: $( $SPEC )*
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn generates_struct_and_builder_with_defaults() {
        builder!(MyStructBuilder -> MyStruct {
            field_i32: i32 = Some(123),
            field_str: &'static str = Some("abc"),
        });

        let my_struct = MyStructBuilder::new().build();
        assert_eq!(my_struct.field_i32, 123);
        assert_eq!(my_struct.field_str, "abc");
    }

    #[test]
    fn generates_struct_and_builder_with_parameters() {
        builder!(MyStructBuilder -> MyStruct {
            field_i32: i32 = Some(123),
            field_str: &'static str = Some("abc"),
        });

        let my_struct = MyStructBuilder::new()
            .field_i32(456)
            .field_str("str")
            .build();
        assert_eq!(my_struct.field_i32, 456);
        assert_eq!(my_struct.field_str, "str");
    }

    #[test]
    fn generates_struct_and_builder_with_generic_types() {
        builder!(MyStructBuilder -> MyStruct {
            field_vec: Vec<i32> = Some(vec![123]),
        });

        let my_struct = MyStructBuilder::new().build();
        let my_struct_2 = MyStructBuilder::new().field_vec(vec![234, 456]).build();

        assert_eq!(my_struct.field_vec, vec![123]);
        assert_eq!(my_struct_2.field_vec, vec![234, 456]);
    }

    #[test]
    fn generates_struct_and_builder_with_traits() {
        trait Magic {
            fn abracadabra(&mut self) -> i32;
        }
        struct Dust {
            value: i32,
        };
        impl Magic for Dust {
            fn abracadabra(&mut self) -> i32 {
                self.value
            }
        }

        // Note: we use => instead of -> for the consuming variant of the builder
        builder!(MyStructBuilder => MyStruct {
            field_trait: Box<Magic> = Some(Box::new(Dust { value: 1 })),
            field_vec: Vec<Box<Magic>> = Some(vec![Box::new(Dust { value: 2 })]),
        });

        let mut my_struct = MyStructBuilder::new().build();

        assert_eq!(my_struct.field_trait.abracadabra(), 1);
        assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn generated_build_method_uses_assertions() {
        builder!(MyStructBuilder -> MyStruct {
            field_i32: i32 = Some(123),
        },
        assertions: {
            assert!(field_i32 > 0);
        });

        let my_struct = MyStructBuilder::new()
            .field_i32(-1)
            .build();
        assert_eq!(my_struct.field_i32, -1);
    }

    mod visibility_test {
        builder!(OuterStructBuilder -> OuterStruct { field_i32: i32 = Some(1), });

        mod inner {
            builder!(MyStructBuilder -> MyStruct { field_i32: i32 = Some(1), });
            builder!(pub InnerStructBuilder -> InnerStruct { pub field_i32: i32 = Some(1), });

            #[test]
            fn can_access_private_struct_from_within_module() {
                let my_struct = MyStructBuilder::new().build();
                assert_eq!(my_struct.field_i32, 1);
            }

            #[test]
            fn can_access_private_outer_struct_from_inner_module() {
                let outer_struct = super::OuterStructBuilder::new().build();
                assert_eq!(outer_struct.field_i32, 1);
            }
        }

        #[test]
        fn can_access_public_struct_from_outside_module() {
            let inner_struct = inner::InnerStructBuilder::new().build();
            assert_eq!(inner_struct.field_i32, 1);
        }

        // The following causes a compilation failure if uncommented
        // #[test]
        // fn cannot_access_private_struct() {
        //     let my_struct = inner::MyStructBuilder::new().build();
        //     assert_eq!(my_struct.field_i32, 0);
        // }
    }
}
