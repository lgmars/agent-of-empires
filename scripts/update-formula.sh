#!/bin/bash
# Script to update Homebrew formula after a release
# Run this after creating a GitHub release to update the sha256 hashes

set -e

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

REPO="nbrake/agent-of-empires"
BASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}"

echo "Fetching sha256 hashes for v${VERSION}..."

declare -A SHAS

for ARTIFACT in aoe-darwin-arm64 aoe-darwin-amd64 aoe-linux-arm64 aoe-linux-amd64; do
    URL="${BASE_URL}/${ARTIFACT}.tar.gz"
    echo "Downloading ${ARTIFACT}..."
    SHA=$(curl -sL "${URL}" | shasum -a 256 | cut -d' ' -f1)
    SHAS[$ARTIFACT]=$SHA
    echo "  ${ARTIFACT}: ${SHA}"
done

echo ""
echo "=== Homebrew Formula SHA256 values ==="
echo ""
echo "Replace the sha256 lines in your Formula/aoe.rb with:"
echo ""
echo "# macOS ARM64"
echo "sha256 \"${SHAS[aoe-darwin-arm64]}\""
echo ""
echo "# macOS Intel"
echo "sha256 \"${SHAS[aoe-darwin-amd64]}\""
echo ""
echo "# Linux ARM64"
echo "sha256 \"${SHAS[aoe-linux-arm64]}\""
echo ""
echo "# Linux Intel"
echo "sha256 \"${SHAS[aoe-linux-amd64]}\""
