# Builder Macro

This crate contains a `builder!` macro to declare a struct and a corresponding builder.

The macro is inspired from [jadpole/builder-macro][1], and is designed to
remove duplication of field declaration, as well as generating appropriate setter methods.

Include the macro inside your crate's `lib.rs` or `main.rs`.

```rust
#[macro_use]
extern crate builder_macro;
```

## Examples

### Non-consuming Builder

The simplest usage of the builder macro to generate a [non-consuming builder][2] is:

```rust,ignore
builder!(BuilderName -> StructName {
    fieldname: Type = Some(default_value), // or None if there is no sensible default
});
```

The above will generate a module private struct and a non-consuming builder with a single private field.

For example, given the following declaration:

```rust
builder!(BuilderName -> StructName {
    value: i32 = Some(1),
});
```

The generated builder and struct will be:

```rust
struct StructName {
    value: i32,
}

#[doc = "Generated struct builder"]
struct BuilderName {
    value: Option<i32>,
}

impl BuilderName {
    /// Construct the builder
    pub fn new() -> BuilderName { BuilderName { value: Some(1), } }

    /// Build the struct
    pub fn build(&self) -> StructName {
        let value = self.value.clone().unwrap();
        StructName{value: value,}
    }

    #[allow(dead_code)]
    /// Specify a value for the $F_NAME field
    pub fn value(&mut self, value: i32) -> &mut Self {
        self.value = Some(value);
        self
    }
}
```

The full macro usage format is:

```rust
// We declare the builder insider a module simply to demonstrate scope
mod inner {
    builder! {
        /// StructName is an example struct.
        /// These docs are copied over to the generated struct.
        pub BuilderName -> StructName {
            /// a_field is an i32 which must be between 0 and 100 inclusive
            // the trailing comma is mandatory due to how the macro is parsed
            pub a_field: i32 = Some(50),

            // None means no default value, a value must be specified when building
            // meta attributes are copied over to the struct's fields
            #[allow(dead_code)]
            a_private_field: &'static str = None,
        }, assertions: {
            assert!(a_field >= 0);
            assert!(a_field <= 100);
            // Yes you can assert on private fields
            assert!(!a_private_field.is_empty());
        }
    }
}

let my_struct = inner::BuilderName::new()
                    .a_private_field("I must set this to a non-empty string")
                    .build();
assert_eq!(50, my_struct.a_field);
```

### Consuming Builder

To generate a [consuming builder][3], instead of using `->`, use `=>` between the builder name and the target struct
name.

```rust
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
```

### Visibility

Generate a builder and struct with module private visibility:

```rust
builder!(MyStructBuilder -> MyStruct {
    field_i32: i32 = Some(123),
    field_str: &'static str = Some("abc"),
});

let my_struct = MyStructBuilder::new()
    .field_i32(456)
    .build();
assert_eq!(my_struct.field_i32, 456);
assert_eq!(my_struct.field_str, "abc"); // uses default
```

Generate a builder and struct with public visibility:

```rust
mod inner {
    builder!(pub MyStructBuilder -> MyStruct {
        pub field_i32: i32 = Some(123),
        field_str: &'static str = Some("abc"),
    });
}

let my_struct = inner::MyStructBuilder::new()
    .field_i32(456)
    .build();
assert_eq!(my_struct.field_i32, 456);

// The next line will fail compilation if uncommented as field_str is private
// assert_eq!(my_struct.field_str, "abc");
```

# License

The MIT License

[1]: http://jadpole.github.io/rust/builder-macro
[2]: https://doc.rust-lang.org/style/ownership/builders.html#non-consuming-builders-preferred
[3]: https://doc.rust-lang.org/style/ownership/builders.html#consuming-builders
