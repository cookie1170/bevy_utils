#!/usr/bin/env fish

if test (count $argv) -lt 1
    set target bevy_utils
else
    set target "$argv[1]"
end

set root (status filename | path resolve | path dirname)
set crates "$root/crates"

echo "copying '$crates' to '$target'"

mkdir -p "$target"
cp --recursive "$crates"/* "$target"

echo "done! add \"$target/*\" to your workspace's Cargo.toml"
