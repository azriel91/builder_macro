#!/bin/sh

base_dir=$(dirname "${0}")
. "${base_dir}/log_functions.sh"
. "${base_dir}/prerequisite_checks.sh"

# Use a subshell so that cd doesn't affect current shell
base_dir_absolute="$(cd "$(dirname "${0}")" && pwd)"

original_dir=$(pwd)
working_dir="${1-.}"
cd "${working_dir}"

# === Code format === #
log_info "Verifying source meets code formatting standards"
log_info "Running: ! TERM=dumb cargo fmt -- --config-path=\"${base_dir_absolute}\" --write-mode=diff 2>&1 | grep -e \"^\(\+\|-\)\|\(Rustfmt failed\)\" -m 1 > /dev/null"
syntax_output=$(TERM=dumb cargo fmt -- --config-path="${base_dir_absolute}" --write-mode=diff 2>&1)
log_debug "${syntax_output}"
# we negate the result of the next command because a positive grep result indicates a failure
! echo "${syntax_output}" | grep -e "^\(\+\|-\)\|\(Rustfmt failed\)" -m 1 > /dev/null
syntax_check_result=$?
if [ "${syntax_check_result}" -ne "0" ]; then
  log_error "Code format check failed. Please adhere to the rustfmt coding standards" false
  log_error "More info can be found at https://github.com/rust-lang-nursery/rustfmt" false
else
  log_notice "Code format check successful"
fi

# === Compile === #
log_info "Compiling project"
log_info "Running: cargo build"
cargo build
compile_result=$?
if [ "${compile_result}" -ne "0" ]; then
  log_error "Compilation failed, please scroll up to find details of the failure"
else
  log_notice "Compilation successful"
fi

# === Test === #
log_info "Compiling tests"
log_info "Running: cargo test"
cargo test
test_result=$?
if [ "${test_result}" -ne "0" ]; then
  log_error "Tests failed, please scroll up to find details of the failure"
else
  log_notice "Tests successful"
fi

fail_if_error

cd "${original_dir}"

log_notice "Build successful"
