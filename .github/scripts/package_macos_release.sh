#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <tag> <target>" >&2
  exit 1
fi

TAG="$1"
TARGET="$2"
PKG_NAME="mouse-cursor-mover"

# Release Please may produce tags like "v0.1.2" or "mouse-cursor-mover-v0.1.2".
# Normalize to avoid duplicating the component prefix in artifact names.
if [[ "${TAG}" == "${PKG_NAME}-"* ]]; then
  TAG_SUFFIX="${TAG#${PKG_NAME}-}"
else
  TAG_SUFFIX="${TAG}"
fi

APP_NAME="Mouse Cursor Mover"

cargo bundle --release --target "${TARGET}" --format osx --bin mouse-cursor-mover

BIN_SRC="target/${TARGET}/release/mouse-cursor-mover"
if [[ ! -f "${BIN_SRC}" ]]; then
  echo "Missing binary: ${BIN_SRC}" >&2
  exit 1
fi

mkdir -p dist

BIN_NAME="${PKG_NAME}-${TAG_SUFFIX}-${TARGET}"
cp "${BIN_SRC}" "dist/${BIN_NAME}"
chmod +x "dist/${BIN_NAME}"
tar -czf "dist/${BIN_NAME}.tar.gz" -C dist "${BIN_NAME}"

BUNDLE_ROOT="target/${TARGET}/release/bundle/osx"
APP_BUNDLE_SRC="${BUNDLE_ROOT}/${APP_NAME}.app"
if [[ ! -d "${APP_BUNDLE_SRC}" ]]; then
  APP_BUNDLE_SRC="$(find "${BUNDLE_ROOT}" -maxdepth 1 -name "*.app" -print -quit)"
fi

if [[ -z "${APP_BUNDLE_SRC}" || ! -d "${APP_BUNDLE_SRC}" ]]; then
  echo "Missing app bundle in ${BUNDLE_ROOT}" >&2
  exit 1
fi

APP_DIR="dist/${APP_NAME}.app"
rm -rf "${APP_DIR}"
cp -R "${APP_BUNDLE_SRC}" "${APP_DIR}"

ICONSET_DIR="dist/AppIcon.iconset"
rm -rf "${ICONSET_DIR}"
mkdir -p "${ICONSET_DIR}"

have_prebuilt_iconset=true
for name in \
  icon_16x16.png \
  icon_16x16@2x.png \
  icon_32x32.png \
  icon_32x32@2x.png \
  icon_128x128.png \
  icon_128x128@2x.png \
  icon_256x256.png \
  icon_256x256@2x.png \
  icon_512x512.png \
  icon_512x512@2x.png; do
  if [[ ! -f "assets/macos/${name}" ]]; then
    have_prebuilt_iconset=false
    break
  fi
done

if [[ "${have_prebuilt_iconset}" == "true" ]]; then
  cp assets/macos/icon_*.png "${ICONSET_DIR}/"
else
  ICON_SOURCE="assets/icon-running@2x.png"
  if [[ ! -f "${ICON_SOURCE}" ]]; then
    ICON_SOURCE="assets/icon-running.png"
  fi

  # Fallback for local/dev builds when dedicated iconset files are absent.
  for size in 16 32 128 256 512; do
    sips -z "${size}" "${size}" "${ICON_SOURCE}" --out "${ICONSET_DIR}/icon_${size}x${size}.png" >/dev/null
    sips -z "$((size * 2))" "$((size * 2))" "${ICON_SOURCE}" --out "${ICONSET_DIR}/icon_${size}x${size}@2x.png" >/dev/null
  done
fi

mkdir -p "${APP_DIR}/Contents/Resources"
iconutil -c icns "${ICONSET_DIR}" -o "${APP_DIR}/Contents/Resources/AppIcon.icns"
plutil -replace CFBundleIconFile -string "AppIcon" "${APP_DIR}/Contents/Info.plist"
plutil -replace CFBundleIconName -string "AppIcon" "${APP_DIR}/Contents/Info.plist"
