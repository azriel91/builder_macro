#![deny(missing_docs)]

//! This crate contains a `builder!` macro to declare a struct and a corresponding builder.
//!
//! The macro is inspired from [jadpole/builder-macro][1], and is designed to remove duplication of
//! field declaration, as well as generating appropriate setter methods.
//!
//! Specify the dependency in your crate's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! builder_macro = "0.3.0"
//! ```
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
//! # Examples
//!
//! ## Non-consuming Builder
//!
//! The simplest usage of the builder macro to generate a [non-consuming builder][2] is:
//!
//! ```rust,ignore
//! builder!(BuilderName -> StructName {
//!     fieldname: Type = Some(default_value), // or None if there is no sensible default
//! });
//! ```
//!
//! The above will generate a module private struct and a non-consuming builder with a single
//! private field.
//!
//! For example, given the following declaration:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! builder!(BuilderName -> StructName {
//!     value: i32 = Some(1),
//! });
//! # }
//! ```
//!
//! The generated code will function as follows:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! struct StructName {
//!     value: i32,
//! }
//!
//! /// Auto-generated builder
//! struct BuilderName {
//!     value: Option<i32>,
//! }
//!
//! impl BuilderName {
//!     /// Construct the builder
//!     pub fn new() -> BuilderName { BuilderName { value: Some(1), } }
//!
//!     /// Build the struct
//!     pub fn build(&self) -> Result<StructName, &'static str> {
//!         let value = try!(self.value.clone()
//!             .ok_or(concat!("Must pass argument for field: '", stringify!(value), "'")));
//!         Ok(StructName { value: value, })
//!     }
//!
//!     #[allow(dead_code)]
//!     /// Auto-generated setter
//!     pub fn value(&mut self, value: i32) -> &mut Self {
//!         self.value = Some(value);
//!         self
//!     }
//! }
//! # }
//! ```
//!
//! To generate public structs and builders, see [visbility](#visibility).
//!
//! ## Consuming Builder
//!
//! To generate a [consuming builder][3], instead of using `->`, use `=>` between the builder name
//! and the target struct name.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! trait Magic {
//!     fn abracadabra(&mut self) -> i32;
//! }
//! struct Dust {
//!     value: i32,
//! }
//! impl Magic for Dust {
//!     fn abracadabra(&mut self) -> i32 {
//!         self.value
//!     }
//! }
//!
//! // Note: we use => instead of -> for the consuming variant of the builder
//! builder!(MyStructBuilder => MyStruct {
//!     field_trait: Box<Magic> = Some(Box::new(Dust { value: 1 })),
//!     field_vec: Vec<Box<Magic>> = Some(vec![Box::new(Dust { value: 2 })]),
//! });
//!
//! let mut my_struct = MyStructBuilder::new().build().unwrap();
//!
//! assert_eq!(my_struct.field_trait.abracadabra(), 1);
//! assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
//! # }
//! ```
//!
//! ## Visibility
//!
//! Generate a builder and struct with module private visibility:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! builder!(MyStructBuilder -> MyStruct {
//!     field_i32: i32 = Some(123),
//!     field_str: &'static str = Some("abc"),
//! });
//!
//! let my_struct = MyStructBuilder::new()
//!     .field_i32(456)
//!     .build()
//!     .unwrap();
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
//!     builder!(pub MyStructBuilder -> MyStruct {
//!         pub field_i32: i32 = Some(123),
//!         field_str: &'static str = Some("abc"),
//!     });
//! }
//!
//! let my_struct = inner::MyStructBuilder::new()
//!     .field_i32(456)
//!     .build()
//!     .unwrap();
//! assert_eq!(my_struct.field_i32, 456);
//!
//! // The next line will fail compilation if uncommented as field_str is private
//! // assert_eq!(my_struct.field_str, "abc");
//! # }
//! ```
//!
//! ## Assertions
//!
//! You may specify assertions after field declarations inside an `assertions: { ... }` block.
//!
//! If an assertion fails, the `build()` method will return an `Err(...)`.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! builder! {
//!     pub BuilderName -> StructName {
//!         /// a_field is an i32 which must be between 0 and 100 inclusive
//!         pub a_field: i32 = Some(50),
//!         #[allow(dead_code)]
//!         a_private_field: &'static str = None,
//!     }, assertions: {
//!         assert!(a_field >= 0);
//!         assert!(a_field <= 100);
//!         // Yes you can assert on private fields
//!         assert!(!a_private_field.is_empty());
//!     }
//! }
//!
//! let result_1 = BuilderName::new().a_private_field("non-empty string").build();
//! let result_2 = BuilderName::new().a_private_field("").build();
//!
//! assert!(result_1.is_ok());
//! assert_eq!(result_2.err(),
//!            Some("assertion failed: 'assert!(! a_private_field . is_empty (  ))'"));
//! # }
//! ```
//!
//! ## Full Usage Format
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
//!         pub BuilderName -> StructName {
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
//!     .a_private_field("I must set this to a non-empty string")
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(50, my_struct.a_field);
//! # }
//! ```
//!
//! The above will be similar to writing the following:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! mod inner {
//!     /// StructName is an example struct.
//!     /// These docs are copied over to the generated struct.
//!     pub struct StructName {
//!         /// a_field is an i32 which must be between 0 and 100 inclusive
//!         pub a_field: i32,
//!         #[allow(dead_code)]
//!         a_private_field: &'static str,
//!     }
//!
//!     /// Auto-generated builder
//!     pub struct BuilderName {
//!         /// a_field is an i32 which must be between 0 and 100 inclusive
//!         a_field: Option<i32>,
//!         #[allow(dead_code)]
//!         a_private_field: Option<&'static str>,
//!     }
//!
//!     impl BuilderName {
//!         /// Construct the builder
//!         pub fn new() -> BuilderName {
//!             BuilderName{a_field: Some(50), a_private_field: None,}
//!         }
//!
//!         /// Build the struct
//!         pub fn build(&self) -> Result<StructName, &'static str> {
//!             let a_field = try!(self.a_field.clone().ok_or(
//!                 concat!("Must pass argument for field: '", stringify!(a_field), "'") ));
//!             let a_private_field = try!(self.a_private_field.clone().ok_or(
//!                 concat!("Must pass argument for field: '", stringify!(a_private_field), "'") ));
//!
//!             use std::panic;
//!             try!(panic::catch_unwind(|| { assert!(a_field >= 0); }).or(
//!                 Err(concat!("assertion failed: '",
//!                             stringify!( assert!(a_field >= 0) ),
//!                             "'")) ) );
//!             try!(panic::catch_unwind(|| { assert!(a_field <= 100); }).or(
//!                 Err(concat!("assertion failed: '",
//!                             stringify!( assert!(a_field <= 100) ),
//!                             "'")) ) );
//!             try!(panic::catch_unwind(|| { assert!(!a_private_field.is_empty()); }).or(
//!                     Err(concat!("assertion failed: '",
//!                                 stringify!( assert!(!a_private_field.is_empty()) ),
//!                                 "'")) ) );
//!
//!             Ok(StructName {
//!                 a_field: a_field,
//!                 a_private_field: a_private_field,
//!             })
//!         }
//!
//!         #[allow(dead_code)]
//!         /// Auto-generated setter
//!         pub fn a_field(&mut self, value: i32) -> &mut Self {
//!             self.a_field = Some(value);
//!             self
//!         }
//!
//!         #[allow(dead_code)]
//!         /// Auto-generated setter
//!         pub fn a_private_field(&mut self, value: &'static str)
//!          -> &mut Self {
//!             self.a_private_field = Some(value);
//!             self
//!         }
//!     }
//! }
//!
//! let my_struct = inner::BuilderName::new()
//!     .a_private_field("I must set this to a non-empty string")
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(50, my_struct.a_field);
//! # }
//! ```
//!
//! [1]: http://jadpole.github.io/rust/builder-macro
//! [2]: https://doc.rust-lang.org/style/ownership/builders.html#non-consuming-builders-preferred
//! [3]: https://doc.rust-lang.org/style/ownership/builders.html#consuming-builders
//!

#[macro_use]
mod declare_struct_and_builder;
#[macro_use]
mod parse_struct;

#[macro_export]
/// Macro to declare a struct and a corresponding builder. See
/// [the module documentation](index.html) for more.
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
    // used in consuming builder tests
    trait Magic {
        fn abracadabra(&mut self) -> i32;
    }
    struct Dust {
        value: i32,
    }
    impl Magic for Dust {
        fn abracadabra(&mut self) -> i32 {
            self.value
        }
    }

    #[test]
    fn generates_struct_and_builder_with_defaults() {
        builder!(MyStructBuilder -> MyStruct {
            field_i32: i32 = Some(123),
            field_str: &'static str = Some("abc"),
        });

        let my_struct = MyStructBuilder::new().build().unwrap();
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
            .build()
            .unwrap();
        assert_eq!(my_struct.field_i32, 456);
        assert_eq!(my_struct.field_str, "str");
    }

    #[test]
    fn generates_struct_and_builder_with_generic_types() {
        builder!(MyStructBuilder -> MyStruct {
            field_vec: Vec<i32> = Some(vec![123]),
        });

        let my_struct = MyStructBuilder::new().build().unwrap();
        let my_struct_2 = MyStructBuilder::new()
            .field_vec(vec![234, 456])
            .build()
            .unwrap();

        assert_eq!(my_struct.field_vec, vec![123]);
        assert_eq!(my_struct_2.field_vec, vec![234, 456]);
    }

    #[test]
    fn generates_struct_and_builder_with_traits_using_default_values() {
        // Note: we use => instead of -> for the consuming variant of the builder
        builder!(MyStructBuilder => MyStruct {
            field_trait: Box<Magic> = Some(Box::new(Dust { value: 1 })),
            field_vec: Vec<Box<Magic>> = Some(vec![Box::new(Dust { value: 2 })]),
        });

        let mut my_struct = MyStructBuilder::new().build().unwrap();

        assert_eq!(my_struct.field_trait.abracadabra(), 1);
        assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
    }

    #[test]
    fn generates_struct_and_builder_with_traits_specifying_parameters() {
        // Note: we use => instead of -> for the consuming variant of the builder
        builder!(MyStructBuilder => MyStruct {
            field_trait: Box<Magic> = None,
            field_vec: Vec<Box<Magic>> = None,
        });

        let mut my_struct = MyStructBuilder::new()
            .field_trait(Box::new(Dust { value: 1 }))
            .field_vec(vec![Box::new(Dust { value: 2 })])
            .build()
            .unwrap();

        assert_eq!(my_struct.field_trait.abracadabra(), 1);
        assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
    }

    #[test]
    fn generated_build_method_uses_assertions() {
        builder!(MyStructBuilder -> MyStruct {
            #[allow(dead_code)]
            field_i32: i32 = Some(123),
        },
        assertions: {
            assert!(field_i32 > 0);
        });

        let result = MyStructBuilder::new().field_i32(-1).build();

        match result {
            Ok(_) => panic!("Expected Err() caused by assertion failure"),
            Err(msg) => assert_eq!(msg, "assertion failed: 'assert!(field_i32 > 0)'"),
        }
    }

    #[test]
    fn generated_consuming_build_method_uses_assertions() {
        builder!(MyStructBuilder => MyStruct {
            #[allow(dead_code)]
            field_i32: i32 = Some(123),
        },
        assertions: {
            assert!(field_i32 == 99);
        });

        let result = MyStructBuilder::new().build();

        let expected = "assertion failed: 'assert!(field_i32 == 99)'";
        match result {
            Ok(_) => panic!("Expected Err() caused by assertion failure"),
            Err(msg) => assert_eq!(msg, expected),
        }
    }

    #[test]
    fn generated_consuming_build_method_asserts_on_trait_fields() {
        builder!(MyStructBuilder => MyStruct {
            #[allow(dead_code)]
            field_trait: Box<Magic> = Some(Box::new(Dust { value: 1 })),
        },
        assertions: {
            assert_eq!(field_trait.abracadabra(), 99);
        });

        let result = MyStructBuilder::new().build();

        match result {
            Ok(_) => panic!("Expected Err() caused by assertion failure"),
            Err(msg) => {
                assert_eq!(msg,
                           "assertion failed: 'assert_eq!(field_trait . abracadabra (  ) , 99)'")
            }
        }
    }

    mod visibility_test {
        builder!(OuterStructBuilder -> OuterStruct { field_i32: i32 = Some(1), });

        mod inner {
            builder!(MyStructBuilder -> MyStruct { field_i32: i32 = Some(1), });
            builder!(pub InnerStructBuilder -> InnerStruct { pub field_i32: i32 = Some(1), });

            #[test]
            fn can_access_private_struct_from_within_module() {
                let my_struct = MyStructBuilder::new().build().unwrap();
                assert_eq!(my_struct.field_i32, 1);
            }

            #[test]
            fn can_access_private_outer_struct_from_inner_module() {
                let outer_struct = super::OuterStructBuilder::new().build().unwrap();
                assert_eq!(outer_struct.field_i32, 1);
            }
        }

        #[test]
        fn can_access_public_struct_from_outside_module() {
            let inner_struct = inner::InnerStructBuilder::new().build().unwrap();
            assert_eq!(inner_struct.field_i32, 1);
        }

        // The following causes a compilation failure if uncommented
        // #[test]
        // fn cannot_access_private_struct() {
        //     let my_struct = inner::MyStructBuilder::new().build().unwrap();
        //     assert_eq!(my_struct.field_i32, 0);
        // }
    }
}
