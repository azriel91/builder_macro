## 0.2.1 (2016-10-22)

* Updated README.md to match code format command.

## 0.2.0 (2016-10-22)

* Use Rustfmt exit code to determine if code formatting standards are met.
* Use Rustfmt default for line length limit (100). Previously this was overridden to be 120.

## 0.1.0 (2016-10-09)

* `build.sh` script provides linting, syntax checking, and compilation.
* `prepare_release.sh` updates the repository changelog, and commits and tags the commit.
* `prepare_next_version.sh` updates the repository changelog and Cargo.toml, and commits.
