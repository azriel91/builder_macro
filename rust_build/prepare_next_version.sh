#!/bin/sh

base_dir=$(dirname "${0}")
. "${base_dir}/log_functions.sh"
. "${base_dir}/prerequisite_checks.sh"

original_dir=$(pwd)
working_dir="${1-.}"
cd "${working_dir}"

changelog=CHANGELOG.md

# Parse crate version from Cargo.toml
crate_version=$(grep -o -m1 'version\s*=\s*"[0-9]\.[0-9]\.[0-9]"' Cargo.toml | grep -o '[0-9]\.[0-9]\.[0-9]')
crate_major=$(echo $crate_version | cut -d . -f 1)
crate_minor=$(echo $crate_version | cut -d . -f 2)
crate_patch=$(echo $crate_version | cut -d . -f 3)
log_debug "Parsed crate version: ${crate_major}.${crate_minor}.${crate_patch}"

# Check if user has passed us any arguments. If not we default to incrementing the minor version
next_major=$crate_major
next_minor=$crate_minor
next_patch=$crate_patch
for arg in "$@"
do
  if test $arg = '--major'; then
    next_major=$(($crate_major + 1))
    next_minor=0
    next_patch=0
  elif test $arg = '--minor'; then
    next_minor=$(($crate_minor + 1))
    next_patch=0
  elif test $arg = '--patch'; then
    next_patch=$(($crate_patch + 1))
  fi
done
next_version="${next_major}.${next_minor}.${next_patch}"
test $next_version = $crate_version && next_minor=$(($crate_minor + 1)) next_patch=0
next_version="${next_major}.${next_minor}.${next_patch}"

# Update Cargo.toml to use next version
sed -i "s/\(version\\s*=\\s*\)\"${crate_version}\"/\\1\"${next_version}\"/" Cargo.toml

# Update changelog for next version
sed -i "1i ## ${next_version} (unreleased)\n" $changelog

# Commit
git add $changelog Cargo.toml
git commit -m "Updated ${changelog} and Cargo.toml for ${next_version}"

cd "${original_dir}"

fail_if_error

message=$(cat <<-EOF
	Successfully updated ${changelog} and Cargo.toml.
	Please review the commit, then run:
	  git push
	EOF
	)
log_notice "${message}"
