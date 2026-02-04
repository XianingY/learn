#!/usr/bin/env python3
"""
Brew Analysis Script
Generates a categorized report of installed Homebrew packages.
"""

import json
from collections import defaultdict
from pathlib import Path

# Category keywords for classification
CATEGORIES = {
    "Languages & Runtimes": [
        "go",
        "python",
        "node",
        "ruby",
        "rust",
        "java",
        "openjdk",
        "perl",
        "lua",
        "erlang",
        "elixir",
        "scala",
        "kotlin",
        "swift",
        "clang",
        "llvm",
        "gcc",
    ],
    "Build Tools": [
        "cmake",
        "make",
        "autoconf",
        "automake",
        "pkg-config",
        "libtool",
        "meson",
        "ninja",
        "gradle",
        "maven",
        "bazel",
        "scons",
        "bison",
        "flex",
        "m4",
    ],
    "Media & Graphics": [
        "ffmpeg",
        "imagemagick",
        "jpeg",
        "png",
        "gif",
        "webp",
        "heif",
        "av1",
        "x264",
        "x265",
        "opus",
        "lame",
        "vorbis",
        "theora",
        "cairo",
        "pango",
        "harfbuzz",
        "freetype",
        "fontconfig",
    ],
    "Network & Security": [
        "openssl",
        "curl",
        "wget",
        "gnupg",
        "gpg",
        "ssh",
        "ssl",
        "tls",
        "libssh",
        "nmap",
        "netcat",
        "socat",
        "mtr",
        "tcpdump",
        "wireshark",
    ],
    "Databases": [
        "sqlite",
        "postgresql",
        "mysql",
        "redis",
        "mongodb",
        "mariadb",
        "leveldb",
        "rocksdb",
        "lmdb",
    ],
    "Dev Tools": [
        "git",
        "vim",
        "neovim",
        "helix",
        "tmux",
        "fzf",
        "ripgrep",
        "rg",
        "fd",
        "bat",
        "exa",
        "eza",
        "jq",
        "yq",
        "gh",
        "hub",
        "tree",
        "htop",
        "watch",
        "entr",
        "zoxide",
        "starship",
        "direnv",
    ],
    "Package Managers": [
        "pipx",
        "uv",
        "yarn",
        "npm",
        "pnpm",
        "cargo",
        "gem",
        "cocoapods",
        "buf",
    ],
    "Virtualization & Containers": [
        "qemu",
        "docker",
        "podman",
        "lima",
        "colima",
        "hyperkit",
        "virtualbox",
    ],
    "AI & ML Tools": ["huggingface-cli", "gemini-cli", "ollama"],
    "Libraries": [],  # Will catch lib* prefixes
}


def load_data():
    """Load brew data and leaves list."""
    with open("brew_data.json", "r") as f:
        data = json.load(f)

    leaves = set()
    if Path("brew_leaves.txt").exists():
        with open("brew_leaves.txt", "r") as f:
            leaves = {line.strip() for line in f if line.strip()}

    return data, leaves


def categorize_package(name, desc):
    """Categorize a package based on name and description."""
    name_lower = name.lower()
    desc_lower = (desc or "").lower()

    # Check for lib* prefix first
    if name_lower.startswith("lib") or name_lower.startswith("glib"):
        return "Libraries"

    # Check each category
    for category, keywords in CATEGORIES.items():
        if category == "Libraries":
            continue
        for keyword in keywords:
            if keyword in name_lower or keyword in desc_lower:
                return category

    return "Other Tools & Utilities"


def analyze_formulae(formulae, leaves):
    """Analyze formulae and return categorized data."""
    categorized = defaultdict(list)
    cleanup_candidates = []

    for formula in formulae:
        name = formula.get("name", "unknown")
        desc = formula.get("desc", "No description available")
        homepage = formula.get("homepage", "")

        # Get installed version
        installed = formula.get("installed", [])
        version = installed[-1].get("version", "?") if installed else "?"

        # Check if it's a leaf (user-installed) or dependency
        is_leaf = name in leaves or any(name in leaf for leaf in leaves)

        # Check installed_on_request from the latest install
        installed_on_request = False
        if installed:
            installed_on_request = installed[-1].get("installed_on_request", False)

        category = categorize_package(name, desc)

        pkg_info = {
            "name": name,
            "version": version,
            "desc": desc,
            "homepage": homepage,
            "is_leaf": is_leaf or installed_on_request,
            "installed_on_request": installed_on_request,
        }

        categorized[category].append(pkg_info)

        # Cleanup candidates: leaves that look like libraries (unusual)
        if (is_leaf or installed_on_request) and category == "Libraries":
            cleanup_candidates.append(pkg_info)

    return categorized, cleanup_candidates


def analyze_casks(casks):
    """Analyze casks (GUI applications)."""
    result = []
    for cask in casks:
        name = cask.get("token", "unknown")
        names = cask.get("name", [])
        display_name = names[0] if names else name
        desc = cask.get("desc", "No description available")
        homepage = cask.get("homepage", "")

        result.append(
            {
                "name": name,
                "display_name": display_name,
                "desc": desc,
                "homepage": homepage,
            }
        )

    return result


def generate_report(categorized, cleanup_candidates, casks, leaves, total_formulae):
    """Generate the Markdown report."""
    lines = []

    # Header
    lines.append("# Brew Analysis Report")
    lines.append("")
    lines.append("*Auto-generated analysis of installed Homebrew packages*")
    lines.append("")

    # Executive Summary
    lines.append("## Executive Summary")
    lines.append("")
    lines.append(f"| Metric | Count |")
    lines.append(f"|--------|-------|")
    lines.append(f"| **Total Formulae** | {total_formulae} |")
    lines.append(f"| **User-Installed (Leaves)** | {len(leaves)} |")
    lines.append(f"| **Dependencies** | {total_formulae - len(leaves)} |")
    lines.append(f"| **Casks (GUI Apps)** | {len(casks)} |")
    lines.append(f"| **Cleanup Candidates** | {len(cleanup_candidates)} |")
    lines.append("")

    # Cleanup Candidates
    lines.append("## Cleanup Candidates")
    lines.append("")
    if cleanup_candidates:
        lines.append(
            "These are packages you explicitly installed that look like libraries (unusual for direct installation):"
        )
        lines.append("")
        for pkg in cleanup_candidates:
            lines.append(f"- **{pkg['name']}** (v{pkg['version']}): {pkg['desc']}")
        lines.append("")
        lines.append(
            "> **Note**: Review these before removing. They may be needed for development."
        )
    else:
        lines.append("✅ No suspicious packages found. Your setup looks clean!")
    lines.append("")

    # Leaves (User Installed) - Categorized
    lines.append("## Leaves (User Installed)")
    lines.append("")
    lines.append(
        "These are packages you explicitly installed (not pulled in as dependencies):"
    )
    lines.append("")

    for category in sorted(categorized.keys()):
        pkgs = [p for p in categorized[category] if p["is_leaf"]]
        if pkgs:
            lines.append(f"### {category}")
            lines.append("")
            for pkg in sorted(pkgs, key=lambda x: x["name"]):
                homepage_link = f" [{pkg['homepage']}]" if pkg["homepage"] else ""
                lines.append(
                    f"- **{pkg['name']}** (v{pkg['version']}): {pkg['desc']}{homepage_link}"
                )
            lines.append("")

    # Dependencies - Categorized
    lines.append("## Dependencies")
    lines.append("")
    lines.append("These packages were installed automatically as dependencies:")
    lines.append("")

    for category in sorted(categorized.keys()):
        pkgs = [p for p in categorized[category] if not p["is_leaf"]]
        if pkgs:
            lines.append(f"### {category}")
            lines.append("")
            for pkg in sorted(pkgs, key=lambda x: x["name"]):
                lines.append(f"- **{pkg['name']}** (v{pkg['version']}): {pkg['desc']}")
            lines.append("")

    # Casks
    if casks:
        lines.append("## Casks (GUI Applications)")
        lines.append("")
        for cask in sorted(casks, key=lambda x: x["name"]):
            lines.append(
                f"- **{cask['display_name']}** (`{cask['name']}`): {cask['desc']}"
            )
        lines.append("")

    # Footer
    lines.append("---")
    lines.append("")
    lines.append("*Report generated by analyze_brew.py*")

    return "\n".join(lines)


def main():
    print("Loading brew data...")
    data, leaves = load_data()

    formulae = data.get("formulae", [])
    casks = data.get("casks", [])

    print(f"Found {len(formulae)} formulae and {len(casks)} casks")
    print(f"User-installed leaves: {len(leaves)}")

    print("Analyzing formulae...")
    categorized, cleanup_candidates = analyze_formulae(formulae, leaves)

    print("Analyzing casks...")
    cask_info = analyze_casks(casks)

    print("Generating report...")
    report = generate_report(
        categorized, cleanup_candidates, cask_info, leaves, len(formulae)
    )

    with open("BREW_REPORT.md", "w") as f:
        f.write(report)

    print(f"✅ Report written to BREW_REPORT.md")
    print(f"   - {len(formulae)} formulae analyzed")
    print(f"   - {len(leaves)} leaves identified")
    print(f"   - {len(cleanup_candidates)} cleanup candidates found")


if __name__ == "__main__":
    main()
