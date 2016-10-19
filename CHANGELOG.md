## 0.3.0 (unreleased)

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
