#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="md-viewer"

# ── Colors ──────────────────────────────────────────────────────────
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info()  { echo -e "${GREEN}[✓]${NC} $1"; }
warn()  { echo -e "${YELLOW}[!]${NC} $1"; }

# ── Build & install binary ──────────────────────────────────────────
echo "==> Building md-viewer in release mode..."
cd "$PROJECT_DIR"
cargo build --release

echo "==> Installing binary to ${INSTALL_DIR}..."
mkdir -p "$INSTALL_DIR"
cp "${PROJECT_DIR}/target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
info "Binary installed: ${INSTALL_DIR}/${BINARY_NAME}"

# ── Desktop entry (all DEs) ─────────────────────────────────────────
APPLICATIONS_DIR="${HOME}/.local/share/applications"
mkdir -p "$APPLICATIONS_DIR"
cat > "${APPLICATIONS_DIR}/md-viewer.desktop" << 'EOF'
[Desktop Entry]
Name=Markdown Preview
Comment=Preview markdown files in browser
Exec=md-viewer %f
Type=Application
Terminal=false
Icon=text-html
Categories=Utility;TextEditor;
MimeType=text/markdown;text/x-markdown;
EOF

# Register MIME defaults
xdg-mime default md-viewer.desktop text/markdown 2>/dev/null || true
xdg-mime default md-viewer.desktop text/x-markdown 2>/dev/null || true
info "Desktop entry installed"

# ── File manager integrations ───────────────────────────────────────

# Helper: check if a command exists
has_cmd() { command -v "$1" &>/dev/null; }

# ── Nemo (Cinnamon) ─────────────────────────────────────────────────
install_nemo() {
    local dir="${HOME}/.local/share/nemo/actions"
    mkdir -p "$dir"
    cat > "${dir}/md-viewer.nemo_action" << EOF
[Nemo Action]
Name=Open in Markdown Preview
Comment=Preview this markdown file in the browser
Exec=${INSTALL_DIR}/${BINARY_NAME} %F
Icon-Name=text-html
Selection=any
Extensions=md;mdown;markdown;mkd;
Separator=;
Quote=double
EOF
    info "Nemo (Cinnamon) context menu installed"
}

# ── Nautilus (GNOME) ────────────────────────────────────────────────
install_nautilus() {
    local ext_dir="${HOME}/.local/share/nautilus-python/extensions"
    mkdir -p "$ext_dir"
    cat > "${ext_dir}/md_viewer.py" << 'PYEOF'
import os
import gi
gi.require_version('Nautilus', '3.0')
from gi.repository import Nautilus, GObject

class MdViewerExtension(GObject.GObject, Nautilus.MenuProvider):
    def __init__(self):
        self.binary = os.path.expanduser("~/.local/bin/md-viewer")

    def _is_markdown(self, uri):
        if uri.startswith("file://"):
            path = uri[7:]
            return path.lower().endswith((".md", ".mdown", ".markdown", ".mkd"))
        return False

    def get_file_items(self, window, files):
        if len(files) != 1 or not self._is_markdown(files[0].get_uri()):
            return []
        item = Nautilus.MenuItem(
            name="MdViewerExtension::Preview",
            label="Open in Markdown Preview",
            tip="Preview markdown in browser",
            icon="text-html",
        )
        uri = files[0].get_uri()
        item.connect("activate", self._activate, uri)
        return [item]

    def _activate(self, menu, uri):
        import subprocess
        if uri.startswith("file://"):
            path = uri[7:]
            subprocess.Popen([self.binary, path])
PYEOF
    info "Nautilus (GNOME) context menu installed"
    warn "Restart Nautilus with: nautilus -q"
}

# ── Dolphin (KDE Plasma) ────────────────────────────────────────────
install_dolphin() {
    local dir="${HOME}/.local/share/kservices5/ServiceMenus"
    mkdir -p "$dir"
    cat > "${dir}/md-viewer.desktop" << EOF
[Desktop Entry]
Type=Service
ServiceTypes=KonqPopupMenu/Plugin
MimeType=text/markdown;text/x-markdown;
Actions=mdPreview;
X-KDE-Submenu=Markdown

[Desktop Action mdPreview]
Name=Preview in Browser
Icon=text-html
Exec=${INSTALL_DIR}/${BINARY_NAME} %F
EOF
    info "Dolphin (KDE Plasma) context menu installed"
}

# ── Thunar (XFCE) ───────────────────────────────────────────────────
install_thunar() {
    local config_dir="${HOME}/.config/Thunar"
    local uca_file="${config_dir}/uca.xml"
    mkdir -p "$config_dir"

    local new_action='  <action>
    <icon>text-html</icon>
    <name>Open in Markdown Preview</name>
    <tooltip>Preview this markdown file in the browser</tooltip>
    <command>'"${INSTALL_DIR}/${BINARY_NAME}"' %f</command>
    <description>Preview markdown in browser</description>
    <patterns>*.md;*.mdown;*.markdown;*.mkd</patterns>
    <other-files/>
    <directories/>
  </action>'

    # If uca.xml exists, insert action before </actions>
    if [[ -f "$uca_file" ]]; then
        if ! grep -q "md-viewer\|Markdown Preview" "$uca_file" 2>/dev/null; then
            sed -i "s|</actions>|${new_action}\n</actions>|" "$uca_file"
            info "Thunar (XFCE) context menu added to existing uca.xml"
        else
            info "Thunar (XFCE) context menu already exists"
        fi
    else
        cat > "$uca_file" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<actions>
${new_action}
</actions>
EOF
        info "Thunar (XFCE) context menu installed"
    fi
    warn "Restart Thunar with: thunar -q"
}

# ── Detect & install ────────────────────────────────────────────────
echo ""
echo "==> Installing file manager integrations..."

installed_any=false

if has_cmd nemo; then
    install_nemo
    installed_any=true
fi

if has_cmd nautilus; then
    install_nautilus
    installed_any=true
fi

if has_cmd dolphin; then
    install_dolphin
    installed_any=true
fi

if has_cmd thunar; then
    install_thunar
    installed_any=true
fi

if ! $installed_any; then
    warn "No supported file managers detected (nemo, nautilus, dolphin, thunar)."
    warn "The desktop entry and binary are still installed."
fi

# ── Done ────────────────────────────────────────────────────────────
echo ""
echo "========================================"
echo "  Installation complete!"
echo "========================================"
echo ""
echo "Usage:"
echo "  CLI:    md-viewer <file.md>"
echo "  GUI:    Right-click any .md file → 'Open in Markdown Preview'"
echo ""
echo "Installed to:"
echo "  Binary:      ${INSTALL_DIR}/${BINARY_NAME}"
echo "  Desktop:     ${APPLICATIONS_DIR}/md-viewer.desktop"
echo ""
