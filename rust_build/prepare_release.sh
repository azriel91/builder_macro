#!/bin/sh

base_dir=$(dirname "${0}")
. "${base_dir}/log_functions.sh"
. "${base_dir}/prerequisite_checks.sh"

original_dir=$(pwd)
working_dir="${1-.}"
cd "${working_dir}"

# Parse changelog version
changelog=CHANGELOG.md
changelog_version=$(grep -o -m1 '[0-9]\.[0-9]\.[0-9]' $changelog)
changelog_major=$(echo $changelog_version | cut -d . -f 1)
changelog_minor=$(echo $changelog_version | cut -d . -f 2)
changelog_patch=$(echo $changelog_version | cut -d . -f 3)
log_debug "Parsed ${changelog} version: ${changelog_major}.${changelog_minor}.${changelog_patch}"

# Parse crate version from Cargo.toml
crate_version=$(grep -o -m1 'version\s*=\s*"[0-9]\.[0-9]\.[0-9]"' Cargo.toml | grep -o '[0-9]\.[0-9]\.[0-9]')
crate_major=$(echo $crate_version | cut -d . -f 1)
crate_minor=$(echo $crate_version | cut -d . -f 2)
crate_patch=$(echo $crate_version | cut -d . -f 3)
log_debug "Parsed crate version: ${crate_major}.${crate_minor}.${crate_patch}"

if test "${changelog_version}" != "${crate_version}"; then
  log_error "${changelog} version and Cargo.toml version mismatch: ${changelog_version} != ${crate_version}"
fi

# Update changelog to use current date
release_date=$(date --iso-8601)
sed -i "s/unreleased/${release_date}/" $changelog
if ! head -1 $changelog | grep -qFm 1 "## ${crate_version} (${release_date})"; then
  log_error "Failed to update changelog"
fi

# Commit
git add $changelog
git commit -m "Updated ${changelog} for ${crate_version} release"

# Tag the commit
if ! git tag $crate_version; then
  log_error "Failed to tag repository"
fi

cd "${original_dir}"

fail_if_error

message=$(cat <<-EOF
	Successfully updated ${changelog} and tagged repository.
	Please review the commit, then run:
	  git push origin master ${crate_version}
	
	If you are publishing to crates.io, also run:
	  cargo publish
	EOF
	)
log_notice "${message}"
