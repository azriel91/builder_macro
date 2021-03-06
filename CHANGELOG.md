## 0.6.0 (unreleased)

## 0.5.1 (2016-11-27)

* Fixed a missing comma in parse struct

## 0.5.0 (2016-11-26)

* Updated generated builder to take in required parameters in the constructor
* Parameters without a default no longer use ` = None` when they are specified
* Default values are now specified using `default_value` instead of `Some(default_value)`
* Replace the `try!` macro with the `?` operator

## 0.4.0 (2016-10-23)

* Updated source code to adhere to Rustfmt defaults.

## 0.3.0 (2016-10-19)

* Return `Result<T, &'static str>` from the `build()` method instead of `Result<T, String>`

## 0.2.0 (2016-10-17)

* Return `Result<T, String>` from the `build()` method
* Return `Err(...)` when a parameter is not passed and there is no default value
* Return `Err(...)` when an assertion fails

## 0.1.1 (2016-10-10)

* Updated documentation
* Added automated generation of documentation to `gh-pages`, published at [azriel.im/builder_macro](http://azriel.im/builder_macro)
* Added automated coverage report at [coveralls.io](https://coveralls.io/github/azriel91/builder_macro)
* Use [rust_build](https://github.com/azriel91/rust_build) in travis build

## 0.1.0 (2016-10-07)

* Initial implementation which supports generating consuming and non-consuming builder variants
* Meta attributes are copied to generated struct
