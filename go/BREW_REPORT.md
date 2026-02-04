# Brew Analysis Report

*Auto-generated analysis of installed Homebrew packages*

## Executive Summary

| Metric | Count |
|--------|-------|
| **Total Formulae** | 160 |
| **User-Installed (Leaves)** | 27 |
| **Dependencies** | 133 |
| **Casks (GUI Apps)** | 11 |
| **Cleanup Candidates** | 0 |

## Cleanup Candidates

✅ No suspicious packages found. Your setup looks clean!

## Leaves (User Installed)

These are packages you explicitly installed (not pulled in as dependencies):

### Build Tools

- **bison** (v3.8.2): Parser generator [https://www.gnu.org/software/bison/]
- **cmake** (v4.1.2): Cross-platform make [https://www.cmake.org/]

### Dev Tools

- **bat** (v0.26.1): Clone of cat(1) with syntax highlighting and Git integration [https://github.com/sharkdp/bat]
- **dtc** (v1.7.2): Device tree compiler [https://git.kernel.org/pub/scm/utils/dtc/dtc.git]
- **gh** (v2.83.2): GitHub command-line tool [https://cli.github.com/]
- **git-xet** (v0.2.0): Git LFS plugin that uploads and downloads using the Xet protocol [https://github.com/huggingface/xet-core]
- **helix** (v25.07.1): Post-modern modal text editor [https://helix-editor.com]
- **htop** (v3.4.1): Improved top (interactive process viewer) [https://htop.dev/]
- **huggingface-cli** (v1.2.3_1): Client library for huggingface.co hub [https://huggingface.co/docs/huggingface_hub/guides/cli]
- **tmux** (v3.6a): Terminal multiplexer [https://tmux.github.io/]
- **tree** (v2.2.1): Display directories as trees (with optional color/HTML output) [https://oldmanprogrammer.net/source.php?dir=projects/tree]
- **zoxide** (v0.9.8): Shell extension to navigate your filesystem faster [https://github.com/ajeetdsouza/zoxide]

### Languages & Runtimes

- **fzf** (v0.67.0): Command-line fuzzy finder written in Go [https://github.com/junegunn/fzf]
- **gemini-cli** (v0.27.0): Interact with Google Gemini AI models from the command-line [https://github.com/google-gemini/gemini-cli]
- **go** (v1.25.3): Open source programming language to build simple/reliable/efficient software [https://go.dev/]
- **node** (v25.5.0): Open-source, cross-platform JavaScript runtime environment [https://nodejs.org/]
- **openjdk@17** (v17.0.15): Development kit for the Java programming language [https://openjdk.org/]
- **pipx** (v1.8.0): Execute binaries from Python packages in isolated environments [https://pipx.pypa.io]
- **ruby** (v3.4.8): Powerful, clean, object-oriented scripting language [https://www.ruby-lang.org/]
- **uv** (v0.9.15): Extremely fast Python package installer and resolver, written in Rust [https://docs.astral.sh/uv/]
- **yarn** (v1.22.22): JavaScript package manager [https://yarnpkg.com/]

### Media & Graphics

- **ffmpeg** (v8.0.1): Play, record, convert, and stream select audio and video codecs [https://ffmpeg.org/]
- **imagemagick** (v7.1.1-47): Tools and libraries to manipulate images in select formats [https://imagemagick.org/index.php]

### Network & Security

- **wget** (v1.25.0): Internet file retriever [https://www.gnu.org/software/wget/]

### Other Tools & Utilities

- **mole** (v1.23.2): Deep clean and optimize your Mac [https://github.com/tw93/Mole]
- **opencode** (v1.1.51): The AI coding agent built for the terminal. [https://github.com/anomalyco/opencode]
- **riscv-isa-sim** (vmain): RISC-V ISA simulator (spike) [http://riscv.org]

### Package Managers

- **buf** (v1.65.0): New way of working with Protocol Buffers [https://github.com/bufbuild/buf]
- **cocoapods** (v1.16.2_1): Dependency manager for Cocoa projects [https://cocoapods.org/]

### Virtualization & Containers

- **qemu** (v10.2.0): Generic machine emulator and virtualizer [https://www.qemu.org/]

## Dependencies

These packages were installed automatically as dependencies:

### Build Tools

- **m4** (v1.4.20): Macro processing language

### Databases

- **sqlite** (v3.51.2): Command-line interface for SQLite

### Dev Tools

- **cjson** (v1.7.19): Ultralightweight JSON parser in ANSI C
- **git-lfs** (v3.7.1): Git extension for versioning large files
- **highway** (v1.3.0): Performance-portable, length-agnostic SIMD with runtime dispatch
- **openexr** (v3.4.4): High dynamic-range image file format
- **ripgrep** (v15.1.0): Search tool like grep and The Silver Searcher
- **snappy** (v1.2.2): Compression/decompression library aiming for high speed
- **xorgproto** (v2024.1): X.Org: Protocol Headers
- **xvid** (v1.3.7): High-performance, high-quality MPEG-4 video library
- **xz** (v5.8.2): General-purpose data compression with high compression ratio
- **zeromq** (v4.3.5_2): High-performance, asynchronous messaging library

### Languages & Runtimes

- **brotli** (v1.2.0): Generic-purpose lossless compression algorithm by Google
- **certifi** (v2025.11.12): Mozilla CA bundle for Python
- **fribidi** (v1.0.16): Implementation of the Unicode BiDi algorithm
- **icu4c@77** (v77.1): C/C++ and Java libraries for Unicode and globalization
- **icu4c@78** (v78.2): C/C++ and Java libraries for Unicode and globalization
- **lz4** (v1.10.0): Extremely Fast Compression algorithm
- **pango** (v1.57.0_1): Framework for layout and rendering of i18n text
- **pcre2** (v10.47): Perl compatible regular expressions library with a new API
- **python@3.14** (v3.14.2): Interpreted, interactive, object-oriented programming language
- **zstd** (v1.5.7): Zstandard is a real-time compression algorithm

### Libraries

- **glib** (v2.86.3): Core application library for C
- **libarchive** (v3.8.4): Multi-format archive and compression library
- **libass** (v0.17.4): Subtitle renderer for the ASS/SSA subtitle format
- **libb2** (v0.98.1): Secure hashing function
- **libbluray** (v1.4.0_1): Blu-Ray disc playback library for media players like VLC
- **libde265** (v1.0.16): Open h.265 video codec implementation
- **libdeflate** (v1.25): Heavily optimized DEFLATE/zlib/gzip compression and decompression
- **libevent** (v2.1.12_1): Asynchronous event library
- **libgit2** (v1.9.2): C library of Git core methods that is re-entrant and linkable
- **libheif** (v1.19.8): ISO/IEC 23008-12:2017 HEIF file format decoder and encoder
- **libidn2** (v2.3.8): International domain name library (IDNA2008, Punycode and TR46)
- **liblqr** (v0.4.3): C/C++ seam carving library
- **libmicrohttpd** (v1.0.2): Light HTTP/1.1 server library
- **libnghttp2** (v1.68.0): HTTP/2 C Library
- **libnghttp3** (v1.15.0): HTTP/3 library written in C
- **libngtcp2** (v1.20.0): IETF QUIC protocol implementation
- **libogg** (v1.3.6): Ogg Bitstream Library
- **libomp** (v20.1.5): LLVM's OpenMP runtime library
- **libpng** (v1.6.53): Library for manipulating PNG images
- **libraw** (v0.21.4): Library for reading RAW files from digital photo cameras
- **librist** (v0.2.11_1): Reliable Internet Stream Transport (RIST)
- **libsamplerate** (v0.2.2): Library for sample rate conversion of audio data
- **libslirp** (v4.9.1): General purpose TCP-IP emulator
- **libsndfile** (v1.2.2_1): C library for files containing sampled sound
- **libsodium** (v1.0.20): NaCl networking and cryptography library
- **libsoxr** (v0.1.3): High quality, one-dimensional sample-rate conversion library
- **libssh** (v0.11.3): C library SSHv1/SSHv2 client and server protocols
- **libssh2** (v1.11.1): C library implementing the SSH2 protocol
- **libtasn1** (v4.20.0): ASN.1 structure parser library
- **libtiff** (v4.7.1): TIFF library and utilities
- **libtool** (v2.5.4): Generic library support script
- **libudfread** (v1.2.0): Universal Disk Format reader
- **libunibreak** (v6.1): Implementation of the Unicode line- and word-breaking algorithms
- **libunistring** (v1.4.1): C string library for manipulating Unicode strings
- **libusb** (v1.0.29): Library for USB device access
- **libuv** (v1.51.0): Multi-platform support library with a focus on asynchronous I/O
- **libvidstab** (v1.1.1): Transcode video stabilization plugin
- **libvmaf** (v3.0.0): Perceptual video quality assessment based on multi-method fusion
- **libvorbis** (v1.3.7): Vorbis general audio compression codec
- **libvpx** (v1.15.2): VP8/VP9 video codec
- **libx11** (v1.8.12): X.Org: Core X11 protocol client library
- **libxau** (v1.0.12): X.Org: A Sample Authorization Protocol for X
- **libxcb** (v1.17.0): X.Org: Interface to the X Window System protocol
- **libxdmcp** (v1.1.5): X.Org: X Display Manager Control Protocol library
- **libxext** (v1.3.6): X.Org: Library for common extensions to the X11 protocol
- **libxrender** (v0.9.12): X.Org: Library for the Render Extension to the X11 protocol
- **libyaml** (v0.2.5): YAML Parser

### Media & Graphics

- **aom** (v3.13.1): Codec library for encoding and decoding AV1 video streams
- **cairo** (v1.18.4): Vector graphics library with cross-device output support
- **dav1d** (v1.5.2): AV1 decoder targeted to be small and fast
- **fontconfig** (v2.17.1): XML-based font configuration API for X Windows
- **freetype** (v2.14.1_1): Software library to render fonts
- **giflib** (v5.2.2): Library and utilities for processing GIFs
- **harfbuzz** (v12.2.0_1): OpenType text shaping engine
- **jasper** (v4.2.5): Library for manipulating JPEG-2000 images
- **jpeg-turbo** (v3.1.3): JPEG image codec that aids compression and decompression
- **jpeg-xl** (v0.11.1_3): New file format for still image compression
- **lame** (v3.100): High quality MPEG Audio Layer III (MP3) encoder
- **openjpeg** (v2.5.4): Library for JPEG-2000 image manipulation
- **openjph** (v0.25.3): Open-source implementation of JPEG2000 Part-15 (or JPH or HTJ2K)
- **opus** (v1.5.2): Audio codec
- **rav1e** (v0.8.1): Fastest and safest AV1 video encoder
- **svt-av1** (v3.1.2): AV1 encoder
- **theora** (v1.2.0): Open video compression format
- **webp** (v1.6.0): Image format providing lossless and lossy compression for web images
- **x264** (vr3222): H.264/AVC encoder
- **x265** (v4.1): H.265/HEVC encoder

### Network & Security

- **flac** (v1.5.0): Free lossless audio codec
- **gnutls** (v3.8.11): GNU Transport Layer Security (TLS) Library
- **mbedtls@3** (v3.6.5): Cryptographic & SSL/TLS library
- **openssl@3** (v3.6.1): Cryptography and SSL/TLS Toolkit

### Other Tools & Utilities

- **ada-url** (v3.4.2): WHATWG-compliant and fast URL parser written in modern C++
- **aribb24** (v1.0.4): Library for ARIB STD-B24, decoding JIS 8 bit characters and parsing MPEG-TS
- **c-ares** (v1.34.6): Asynchronous DNS library
- **ca-certificates** (v2025-12-02): Mozilla CA certificate store
- **capstone** (v5.0.6): Multi-platform, multi-architecture disassembly framework
- **fmt** (v12.1.0): Open-source formatting library for C++
- **frei0r** (v2.5.1): Minimalistic plugin API for video effects
- **gettext** (v0.26_1): GNU internationalization (i18n) and localization (l10n) library
- **gmp** (v6.3.0): GNU multiple precision arithmetic library
- **graphite2** (v1.3.14): Smart font renderer for non-Roman scripts
- **hdrhistogram_c** (v0.11.9): C port of the HdrHistogram
- **imath** (v3.2.2): Library of 2D and 3D vector, matrix, and math operations
- **leptonica** (v1.86.0): Image processing and image analysis library
- **llhttp** (v9.3.0): Port of http_parser to llparse
- **lzo** (v2.10): Real-time data compression library
- **mpdecimal** (v4.0.1): Library for decimal floating point arithmetic
- **mpg123** (v1.33.3): MP3 player for Linux and UNIX
- **ncurses** (v6.5): Text-based UI library
- **nettle** (v3.10.2): Low-level cryptographic library
- **oniguruma** (v6.9.10): Regular expressions library
- **opencore-amr** (v0.1.6): Audio codecs extracted from Android open source project
- **p11-kit** (v0.25.10): Library to load and enumerate PKCS#11 modules
- **pixman** (v0.46.4): Low-level library for pixel manipulation
- **readline** (v8.3.3): Library for command-line editing
- **rubberband** (v4.0.0): Audio time stretcher tool and library
- **sdl2** (v2.32.10): Low-level access to audio, keyboard, mouse, joystick, and graphics
- **shared-mime-info** (v2.4): Database of common MIME types
- **simdjson** (v4.2.4): SIMD-accelerated C++ JSON parser
- **speex** (v1.2.1): Audio codec designed for speech
- **srt** (v1.5.4): Secure Reliable Transport
- **tesseract** (v5.5.1_1): OCR (Optical Character Recognition) engine
- **unbound** (v1.24.2): Validating, recursive, caching DNS resolver
- **utf8proc** (v2.11.3): Clean C library for processing UTF-8 Unicode data
- **vde** (v2.3.3): Ethernet compliant virtual network
- **zimg** (v3.0.6): Scaling, colorspace conversion, and dithering library

### Package Managers

- **little-cms2** (v2.17): Color management engine supporting ICC profiles
- **uvwasi** (v0.0.23): WASI syscall API built atop libuv

## Casks (GUI Applications)

- **Android Studio** (`android-studio`): Tools for building Android applications
- **BetterDisplay** (`betterdisplay`): Display management tool
- **Claude Code** (`claude-code`): Terminal-based AI coding assistant
- **Codex** (`codex`): OpenAI's coding agent that runs in your terminal
- **IINA** (`iina`): Free and open-source media player
- **iTerm2** (`iterm2`): Terminal emulator as alternative to Apple's Terminal app
- **Ice** (`jordanbaird-ice@beta`): Menu bar manager
- **Mos** (`mos`): Smooths scrolling and set mouse scroll directions independently
- **Obsidian** (`obsidian`): Knowledge base that works on top of a local folder of plain text Markdown files
- **Pika** (`pika`): Colour picker for colours onscreen
- **Stats** (`stats`): System monitor for the menu bar

---

*Report generated by analyze_brew.py*