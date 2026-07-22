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

echo "PKG_NAME='${pkg_name}'"
echo "APP_NAME='${app_name}'"
echo "TAG_SUFFIX='${tag_suffix}'"
echo "APP_ARCHIVE='${app_archive}'"
echo "DMG_NAME='${dmg_name}'"
echo "APP_DIR='dist/${app_name}.app'"
echo "APP_ARCHIVE_PATH='dist/${app_archive}'"
echo "DMG_PATH='dist/${dmg_name}'"
echo "DMG_ROOT='dist/dmg-root-${target}'"
