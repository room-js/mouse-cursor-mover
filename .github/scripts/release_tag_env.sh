#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <tag> <target>" >&2
  exit 1
fi

tag_name="$1"
target="$2"
pkg_name="mouse-cursor-mover"
app_name="Mouse Cursor Mover"

if [[ "${tag_name}" == "${pkg_name}-"* ]]; then
  tag_suffix="${tag_name#${pkg_name}-}"
else
  tag_suffix="${tag_name}"
fi

app_archive="${pkg_name}-${tag_suffix}-${target}.app.zip"
dmg_name="${pkg_name}-${tag_suffix}-${target}.dmg"

printf "PKG_NAME=%q\n" "${pkg_name}"
printf "APP_NAME=%q\n" "${app_name}"
printf "TAG_SUFFIX=%q\n" "${tag_suffix}"
printf "APP_ARCHIVE=%q\n" "${app_archive}"
printf "DMG_NAME=%q\n" "${dmg_name}"
printf "APP_DIR=%q\n" "dist/${app_name}.app"
printf "APP_ARCHIVE_PATH=%q\n" "dist/${app_archive}"
printf "DMG_PATH=%q\n" "dist/${dmg_name}"
printf "DMG_ROOT=%q\n" "dist/dmg-root-${target}"
