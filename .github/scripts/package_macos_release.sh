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

VERSION="${TAG_SUFFIX#v}"
APP_NAME="Mouse Cursor Mover"

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

APP_DIR="dist/${APP_NAME}.app"
rm -rf "${APP_DIR}"
mkdir -p "${APP_DIR}/Contents/MacOS"
mkdir -p "${APP_DIR}/Contents/Resources"

cp "${BIN_SRC}" "${APP_DIR}/Contents/MacOS/mouse-cursor-mover"
chmod +x "${APP_DIR}/Contents/MacOS/mouse-cursor-mover"

cat > "${APP_DIR}/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleExecutable</key>
  <string>mouse-cursor-mover</string>
  <key>CFBundleIdentifier</key>
  <string>com.roomjs.mouse-cursor-mover</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>${APP_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>LSUIElement</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

APP_ARCHIVE="${PKG_NAME}-${TAG_SUFFIX}-${TARGET}.app.zip"
rm -f "dist/${APP_ARCHIVE}"
ditto -c -k --sequesterRsrc --keepParent "${APP_DIR}" "dist/${APP_ARCHIVE}"

DMG_ROOT="dist/dmg-root-${TARGET}"
rm -rf "${DMG_ROOT}"
mkdir -p "${DMG_ROOT}"
cp -R "${APP_DIR}" "${DMG_ROOT}/"
ln -s /Applications "${DMG_ROOT}/Applications"

DMG_NAME="${PKG_NAME}-${TAG_SUFFIX}-${TARGET}.dmg"
rm -f "dist/${DMG_NAME}"
hdiutil create -volname "${APP_NAME}" -srcfolder "${DMG_ROOT}" -ov -format UDZO "dist/${DMG_NAME}"
