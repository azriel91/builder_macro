#![deny(missing_docs)]

//! This crate contains two macros to declare a struct and a corresponding builder.
//!
//! * `data_struct!`: The builder returns a `Result<StructName, &'static str>`
//! * `object_struct!`: The builder returns the declared `StructName`
//!
//! The macro is inspired from [jadpole/builder-macro][1], and is designed to remove duplication of
//! field declaration, as well as generating appropriate setter methods.
//!
//! # Background
//!
//! _For usage, please skip ahead to the [Usage](#usage) section._
//!
//! There are two kinds of structs that this crate aims to support:
//!
//! * Data structs: Parameter values are only known at runtime, and failure to build should be
//!                 handled by the application.
//! * Object structs: Parameter values are largely known at compile time, and failure to build means
//!                   the application no longer works, and should panic.
//!
//! For data structs, returning a `Result` allows the caller to handle the failure gracefully.
//! For object structs, any `panic!`s should be caught by the developer before release. By removing
//! the intermediary `Result`, the developer also no longer needs to call `unwrap()`, which makes
//! the code _that_ much more concise.
//!
//! # Usage
//!
//! Specify the dependency in your crate's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! builder_macro = "0.5.0"
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
//! _**Disclaimer:** The examples use the `data_struct!` macro. They are equally valid for the
//! `object_struct!` macro, the difference being the return type is the struct itself and not a
//! `Result`._
//!
//! ## Non-consuming Builder
//!
//! The simplest usage of the builder macro to generate a [non-consuming builder][2] is:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! data_struct!(ItemBuilder -> Item {
//!     required_field: i32,
//!     defaulted_field: &'static str = "abc",
//! });
//!
//! let item = ItemBuilder::new(123).build().unwrap();
//! let another = ItemBuilder::new(456).defaulted_field("def").build().unwrap();
//!
//! assert_eq!(123, item.required_field);
//! assert_eq!("abc", item.defaulted_field);
//! assert_eq!(456, another.required_field);
//! assert_eq!("def", another.defaulted_field);
//! # }
//! ```
//!
//! The generated code functions as follows:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate builder_macro;
//! #
//! # fn main() {
//! struct Item {
//!     required_field: i32,
//!     defaulted_field: &'static str,
//! }
//!
//! /// Auto-generated builder
//! struct ItemBuilder {
//!     required_field: Option<i32>,
//!     defaulted_field: Option<&'static str>,
//! }
//!
//! impl ItemBuilder {
//!     /// Construct the builder
//!     pub fn new(required_field: i32) -> ItemBuilder {
//!         ItemBuilder { required_field: Some(required_field), defaulted_field: Some("abc"), }
//!     }
//!
//!     /// Build the struct
//!     pub fn build(&self) -> Result<Item, &'static str> {
//!         let required_field = try!(self.required_field.clone().ok_or(
//!             concat!("Must pass argument for field: '", stringify!(required_field), "'")));
//!         let defaulted_field = try!(self.defaulted_field.clone().ok_or(
//!             concat!("Must pass argument for field: '", stringify!(defaulted_field), "'")));
//!
//!         Ok(Item { required_field: required_field, defaulted_field: defaulted_field })
//!     }
//!
//!     #[allow(dead_code)]
//!     /// Auto-generated setter
//!     pub fn defaulted_field(&mut self, defaulted_field: &'static str) -> &mut Self {
//!         self.defaulted_field = Some(defaulted_field);
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
//! When the generated struct should own trait objects, they cannot be cloned, and so the builder
//! must transfer ownership to the constructed instance.
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
//! data_struct!(MyStructBuilder => MyStruct {
//!     field_trait: Box<Magic> = Box::new(Dust { value: 1 }),
//!     field_vec: Vec<Box<Magic>> = vec![Box::new(Dust { value: 2 })],
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
//! data_struct!(MyStructBuilder -> MyStruct {
//!     field_i32: i32 = 123,
//!     field_str: &'static str = "abc",
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
//!     data_struct!(pub MyStructBuilder -> MyStruct {
//!         pub field_i32: i32 = 123,
//!         field_str: &'static str = "abc",
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
//! data_struct! {
//!     pub BuilderName -> StructName {
//!         #[allow(dead_code)]
//!         a_private_field: &'static str,
//!         /// a_field is an i32 which must be between 0 and 100 inclusive
//!         pub a_field: i32 = 50,
//!     }, assertions: {
//!         assert!(a_field >= 0);
//!         assert!(a_field <= 100);
//!         // Yes you can assert on private fields
//!         assert!(!a_private_field.is_empty());
//!     }
//! }
//!
//! let result_1 = BuilderName::new("non-empty string").build();
//! let result_2 = BuilderName::new("").build();
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
//!     data_struct! {
//!         /// StructName is an example struct.
//!         /// These docs are copied over to the generated struct.
//!         pub BuilderName -> StructName {
//!             // meta attributes are copied over to the struct's fields
//!             #[allow(dead_code)]
//!             a_private_field: &'static str,
//!
//!             /// a_field is an i32 which must be between 0 and 100 inclusive
//!             pub a_field: i32 = 50,
//!         }, assertions: {
//!             assert!(a_field >= 0);
//!             assert!(a_field <= 100);
//!             // Yes you can assert on private fields
//!             assert!(!a_private_field.is_empty());
//!         }
//!     }
//! }
//!
//! let my_struct = inner::BuilderName::new("a_private_field must be non-empty")
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

// Order is important
#[macro_use]
mod declare_structs;
#[macro_use]
mod impl_builder;
#[macro_use]
mod impl_struct_and_builder;
#[macro_use]
mod parse_struct;

// We cannot put these macros into submodules because they cannot be re-exported. See discussion:
// https://github.com/rust-lang/rust/issues/29638
// https://github.com/rust-lang/rfcs/blob/master/text/0453-macro-reform.md

#[macro_export]
/// Macro to declare a struct and a corresponding builder that returns a `Result<T, &'static str>`.
/// See [the module documentation](index.html) for more.
macro_rules! data_struct {
    ( $( $SPEC:tt )* )
    =>
    {
        parse_struct! {
            purpose: data,
            meta: [],
            spec: $( $SPEC )*
        }
    };
}

#[macro_export]
/// Macro to declare a struct and a corresponding builder that returns a `Result<T, &'static str>`.
/// See [the module documentation](index.html) for more.
macro_rules! object_struct {
    ( $( $SPEC:tt )* )
    =>
    {
        parse_struct! {
            purpose: object,
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

    mod data_struct {
        use test::{Dust, Magic};

        #[test]
        fn generates_struct_and_builder_with_defaults() {
            data_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32 = 123,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new().build().unwrap();
            assert_eq!(my_struct.field_i32, 123);
            assert_eq!(my_struct.field_str, "abc");
        }

        #[test]
        fn generates_struct_and_builder_with_no_defaults_and_parameters() {
            data_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str,
            });

            let my_struct = MyStructBuilder::new(456, "str")
                .build()
                .unwrap();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "str");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_and_parameters() {
            data_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new(456).build().unwrap();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "abc");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_and_specified_parameters() {
            data_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new(456).field_str("str").build().unwrap();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "str");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_maintains_order() {
            data_struct!(
                #[derive(Debug)]
                MyStructBuilder -> MyStruct {
                field_a: i32,
                field_b: &'static str = "abc",
                field_c: i32 = 456,
                field_d: &'static str,
            });

            let my_struct = MyStructBuilder::new(123, "def").build().unwrap();
            assert_eq!(my_struct.field_a, 123);
            assert_eq!(my_struct.field_b, "abc");
            assert_eq!(my_struct.field_c, 456);
            assert_eq!(my_struct.field_d, "def");

            assert_eq!("MyStruct { field_a: 123, field_b: \"abc\", field_c: 456, field_d: \
                        \"def\" }",
                       format!("{:?}", my_struct));
        }

        #[test]
        fn generates_struct_and_builder_with_defaults_and_parameters() {
            data_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32 = 123,
                field_str: &'static str = "abc",
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
            data_struct!(MyStructBuilder -> MyStruct {
                field_vec: Vec<i32> = vec![123],
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
            data_struct!(MyStructBuilder => MyStruct {
                field_trait: Box<Magic> = Box::new(Dust { value: 1 }),
                field_vec: Vec<Box<Magic>> = vec![Box::new(Dust { value: 2 })],
            });

            let mut my_struct = MyStructBuilder::new().build().unwrap();

            assert_eq!(my_struct.field_trait.abracadabra(), 1);
            assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
        }

        #[test]
        fn generates_struct_and_builder_with_traits_specifying_parameters() {
            // Note: we use => instead of -> for the consuming variant of the builder
            data_struct!(MyStructBuilder => MyStruct {
                field_trait: Box<Magic>,
                field_vec: Vec<Box<Magic>>,
            });

            let mut my_struct = MyStructBuilder::new(Box::new(Dust { value: 1 }),
                                                     vec![Box::new(Dust { value: 2 })])
                .build()
                .unwrap();

            assert_eq!(my_struct.field_trait.abracadabra(), 1);
            assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
        }

        #[test]
        fn generated_build_method_uses_assertions() {
            data_struct!(MyStructBuilder -> MyStruct {
                #[allow(dead_code)]
                field_i32: i32 = 123,
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
            data_struct!(MyStructBuilder => MyStruct {
                #[allow(dead_code)]
                field_i32: i32 = 123,
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
            data_struct!(MyStructBuilder => MyStruct {
                #[allow(dead_code)]
                field_trait: Box<Magic> = Box::new(Dust { value: 1 }),
            },
            assertions: {
                assert_eq!(field_trait.abracadabra(), 99);
            });

            let result = MyStructBuilder::new().build();

            match result {
                Ok(_) => panic!("Expected Err() caused by assertion failure"),
                Err(msg) => {
                    assert_eq!(msg,
                               "assertion failed: 'assert_eq!(field_trait . abracadabra (  ) , \
                                99)'")
                }
            }
        }

        mod visibility_test {
            data_struct!(OuterStructBuilder -> OuterStruct { field_i32: i32 = 1, });

            mod inner {
                data_struct!(MyStructBuilder -> MyStruct { field_i32: i32 = 1, });
                data_struct!(pub InnerStructBuilder -> InnerStruct { pub field_i32: i32 = 1, });

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

    mod object_struct {
        use test::{Dust, Magic};

        #[test]
        fn generates_struct_and_builder_with_defaults() {
            object_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32 = 123,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new().build();
            assert_eq!(my_struct.field_i32, 123);
            assert_eq!(my_struct.field_str, "abc");
        }

        #[test]
        fn generates_struct_and_builder_with_no_defaults_and_parameters() {
            object_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str,
            });

            let my_struct = MyStructBuilder::new(456, "str").build();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "str");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_and_parameters() {
            object_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new(456).build();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "abc");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_and_specified_parameters() {
            object_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32,
                field_str: &'static str = "abc",
            });

            let my_struct = MyStructBuilder::new(456).field_str("str").build();
            assert_eq!(my_struct.field_i32, 456);
            assert_eq!(my_struct.field_str, "str");
        }

        #[test]
        fn generates_struct_and_builder_with_mixed_defaults_maintains_order() {
            object_struct!(
                #[derive(Debug)]
                MyStructBuilder -> MyStruct {
                field_a: i32,
                field_b: &'static str = "abc",
                field_c: i32 = 456,
                field_d: &'static str,
            });

            let my_struct = MyStructBuilder::new(123, "def").build();
            assert_eq!(my_struct.field_a, 123);
            assert_eq!(my_struct.field_b, "abc");
            assert_eq!(my_struct.field_c, 456);
            assert_eq!(my_struct.field_d, "def");

            assert_eq!(
                "MyStruct { field_a: 123, field_b: \"abc\", field_c: 456, field_d: \"def\" }",
                format!("{:?}", my_struct));
        }

        #[test]
        fn generates_struct_and_builder_with_defaults_and_parameters() {
            object_struct!(MyStructBuilder -> MyStruct {
                field_i32: i32 = 123,
                field_str: &'static str = "abc",
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
            object_struct!(MyStructBuilder -> MyStruct {
                field_vec: Vec<i32> = vec![123],
            });

            let my_struct = MyStructBuilder::new().build();
            let my_struct_2 = MyStructBuilder::new()
                .field_vec(vec![234, 456])
                .build();

            assert_eq!(my_struct.field_vec, vec![123]);
            assert_eq!(my_struct_2.field_vec, vec![234, 456]);
        }

        #[test]
        fn generates_struct_and_builder_with_traits_using_default_values() {
            // Note: we use => instead of -> for the consuming variant of the builder
            object_struct!(MyStructBuilder => MyStruct {
                field_trait: Box<Magic> = Box::new(Dust { value: 1 }),
                field_vec: Vec<Box<Magic>> = vec![Box::new(Dust { value: 2 })],
            });

            let mut my_struct = MyStructBuilder::new().build();

            assert_eq!(my_struct.field_trait.abracadabra(), 1);
            assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
        }

        #[test]
        fn generates_struct_and_builder_with_traits_specifying_parameters() {
            // Note: we use => instead of -> for the consuming variant of the builder
            object_struct!(MyStructBuilder => MyStruct {
                field_trait: Box<Magic>,
                field_vec: Vec<Box<Magic>>,
            });

            let field_trait: Box<Magic> = Box::new(Dust { value: 1 });
            let field_vec: Vec<Box<Magic>> = vec![Box::new(Dust { value: 2 })];
            let mut my_struct = MyStructBuilder::new(field_trait, field_vec).build();

            assert_eq!(my_struct.field_trait.abracadabra(), 1);
            assert_eq!(my_struct.field_vec[0].abracadabra(), 2);
        }

        #[test]
        #[should_panic(expected = "assertion failed")]
        fn generated_build_method_uses_assertions() {
            object_struct!(MyStructBuilder -> MyStruct {
                #[allow(dead_code)]
                field_i32: i32 = 123,
            },
            assertions: {
                assert!(field_i32 > 0);
            });

            MyStructBuilder::new().field_i32(-1).build();
        }

        #[test]
        #[should_panic(expected = "assertion failed")]
        fn generated_consuming_build_method_uses_assertions() {
            object_struct!(MyStructBuilder => MyStruct {
                #[allow(dead_code)]
                field_i32: i32 = 123,
            },
            assertions: {
                assert!(field_i32 == 99);
            });

            MyStructBuilder::new().build();
        }

        #[test]
        #[should_panic(expected = "assertion failed")]
        fn generated_consuming_build_method_asserts_on_trait_fields() {
            object_struct!(MyStructBuilder => MyStruct {
                #[allow(dead_code)]
                field_trait: Box<Magic> = Box::new(Dust { value: 1 }),
            },
            assertions: {
                assert_eq!(field_trait.abracadabra(), 99);
            });

            MyStructBuilder::new().build();
        }

        mod visibility_test {
            object_struct!(OuterStructBuilder -> OuterStruct { field_i32: i32 = 1, });

            mod inner {
                object_struct!(MyStructBuilder -> MyStruct { field_i32: i32 = 1, });
                object_struct!(pub InnerStructBuilder -> InnerStruct { pub field_i32: i32 = 1, });

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
}
