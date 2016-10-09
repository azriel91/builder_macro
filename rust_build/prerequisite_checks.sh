#!/bin/sh

# Checks if pre-requisites are installed

base_dir=$(dirname "${0}")
. "${base_dir}/log_functions.sh"

prerequisite_check() {
  exit_code=$1
  failure_message="${2}"

  if [ ! $exit_code -eq 0 ]; then log_error "${failure_message}"; fi
}

# Cargo
log_debug "Checking pre-requisite: Cargo"
# Note: the following is tab indented because bash heredocs can unindent tabs but not spaces
message=$(cat <<-EOF
	Cargo must be installed. This should come with your Rust distribution.
	Instructions can be found at https://www.rust-lang.org/downloads.html
	EOF
	)
which cargo > /dev/null 2>&1
prerequisite_check $? "${message}"

# Rustfmt
log_debug "Checking pre-requisite: Rustfmt"
message=$(cat <<-EOF
	rustfmt must be installed. Instructions can be found at https://github.com/rust-lang-nursery/rustfmt
	EOF
	)
cargo --list | grep '\Wfmt$' > /dev/null 2>&1
prerequisite_check $? "${message}"
