# ꝡizardTextGenerator

[中文文档](./README_CN.md) | English | [Ɯɨʐǟʀɖ](./README_WIZARD.md)

A simple, fast, and convenient terminal text converter that transforms regular letters into wizard characters.

Ǟ ֆɨʍքʟɛ, ʄǟֆȶ, ǟռɖ ƈօռʋɛռɨɛռȶ ȶɛʀʍɨռǟʟ ȶɛӼȶ ƈօռʋɛʀȶɛʀ ȶɦǟȶ ȶʀǟռֆʄօʀʍֆ ʀɛɢʊʟǟʀ ʟɛȶȶɛʀֆ ɨռȶօ աɨʐǟʀɖ ƈɦǟʀǟƈȶɛʀֆ.

## Character Mapping

### Lowercase (a-z)

| Normal | a | b | c | d | e | f | g | h | i | j | k | l | m | n | o | p | q | r | s | t | u | v | w | x | y | z |
|--------|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Wizard** | **ǟ** | **ɮ** | **ƈ** | **ɖ** | **ɛ** | **ʄ** | **ɢ** | **ɦ** | **ɨ** | **ʝ** | **ӄ** | **ʟ** | **ʍ** | **ռ** | **օ** | **ք** | **զ** | **ʀ** | **ֆ** | **ȶ** | **ʊ** | **ʋ** | **ա** | **Ӽ** | **ʏ** | **ʐ** |

### Uppercase (A-Z)

| Normal | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
|--------|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Wizard** | **Ǟ** | **Ɓ** | **Ƈ** | **Ɖ** | **Ɛ** | **Ƒ** | **Ɠ** | **Ӈ** | **Ɨ** | **Ɉ** | **Ӄ** | **Ł** | **Ɱ** | **Ռ** | **Ø** | **Ք** | **Զ** | **Ʀ** | **Ֆ** | **Ƭ** | **Ʊ** | **Ʋ** | **Ɯ** | **Ҳ** | **Ƴ** | **Ȥ** |

> **Note**: All mappings are **one-to-one** (bijective), ensuring perfect reversibility. Modified characters (B,F,G,J,L,M,O,T,W,X,Y,Z) use visually similar Unicode glyphs to eliminate ambiguity.

## Installation

### Compile from Source

```bash
cargo build --release
```

The compiled binary will be located at `target/release/wizard_text_generator`

### Install to System

#### Method 1: Using Cargo (Recommended)

```bash
cargo install --path .
```

This will install the binary to `~/.cargo/bin/wizard_text_generator` (make sure `~/.cargo/bin` is in your PATH)

```bash
# Optional: Create a shorter alias
ln -s ~/.cargo/bin/wizard_text_generator ~/.cargo/bin/wtg
```

#### Method 2: Manual Installation to `/usr/local/bin`

```bash
# After building the release binary
sudo cp target/release/wizard_text_generator /usr/local/bin/

# Or create a shorter alias (optional)
sudo ln -s /usr/local/bin/wizard_text_generator /usr/local/bin/wtg
```

#### Method 3: Add to PATH (User-specific)

```bash
# Copy to ~/.local/bin (create directory if it doesn't exist)
mkdir -p ~/.local/bin
cp target/release/wizard_text_generator ~/.local/bin/

# Add to PATH in ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Create a shorter alias (optional)
ln -s ~/.local/bin/wizard_text_generator ~/.local/bin/wtg
```

#### Verify Installation

```bash
# Test with full command name
wizard_text_generator "hello"

# Or with short alias (if created)
wtg "hello"
```

## Usage

### 1. Interactive Mode

Simply run the command without arguments to enter interactive mode:

```bash
wizard_text_generator
# Or use the short alias
wtg
```

Interactive mode features:
- **Auto-detection**: Automatically detects if input is normal text or wizard characters
  - Normal text (a-z, A-Z) → Converts to wizard characters
  - Wizard characters → Converts back to normal text
- **Simple**: No commands needed, just type and convert
- `Ctrl+D` - Exit

Example session:
```
$ wtg

    ╔══════════════════════════════════════════════╗
    ║   ꝡizardTextGenerator                        ║
    ║   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━     ║
    ║   Ƭʀǟռֆʄօʀʍ ȶɛӼȶ ɨռȶօ ʍǟɢɨƈǟʟ ƈɦǟʀǟƈȶɛʀֆ     ║
    ╚══════════════════════════════════════════════╝

Type text to convert (auto-detects direction), Ctrl+D to exit

> hello world
ɦɛʟʟօ աօʀʟɖ
> ɦɛʟʟօ աօʀʟɖ
hello world
> The Quick Brown Fox
Ƭɦɛ Զʊɨƈӄ Ɓʀօառ ƑօӼ
> Ƭɦɛ Զʊɨƈӄ Ɓʀօառ ƑօӼ
The Quick Brown Fox
> ^D
Goodbye!
```

### 2. Command Line Arguments

```bash
# Convert to wizard characters
wizard_text_generator "hello world"
# Output: ɦɛʟʟօ աօʀʟɖ

# Reverse conversion (wizard characters -> normal letters)
wizard_text_generator -r "ɦɛʟʟօ աօʀʟɖ"
# Output: hello world

# Uppercase conversion
wizard_text_generator "HELLO WORLD"
# Output: ӇƐŁŁØ ƜØƦŁƉ

# Mixed case
wizard_text_generator "Hello World"
# Output: Ӈɛʟʟօ Ɯօʀʟɖ
```

### 2. Pipe Input

```bash
# Read from standard input
echo "hello" | wizard_text_generator
# Output: ɦɛʟʟօ

# Convert file contents
cat file.txt | wizard_text_generator

# Combine with other commands
echo "Rust is awesome" | wizard_text_generator
# Output: Ʀʊֆȶ ɨֆ ǟաɛֆօʍɛ
```

### 3. Command Line Arguments

```
-r, --reverse    Reverse conversion (wizard characters -> normal letters)
-h, --help       Show help information
```

## Examples

```bash
# Full lowercase alphabet conversion
./target/release/wizard_text_generator "abcdefghijklmnopqrstuvwxyz"
# Output: ǟɮƈɖɛʄɢɦɨʝӄʟʍռօքզʀֆȶʊʋաӼʏʐ

# Full uppercase alphabet conversion
./target/release/wizard_text_generator "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
# Output: ǞƁƇƉƐƑƓӇƗɈӃŁⱮՌØՔԶƦՖƬƱƲƜҲƳȤ

# Mixed text (non-alphabetic characters remain unchanged)
./target/release/wizard_text_generator "Hello, World! 123"
# Output: Ӈɛʟʟօ, Ɯօʀʟɖ! 123

# Reverse conversion
./target/release/wizard_text_generator -r "ɦɛʟʟօ"
# Output: hello
```

## Tech Stack

- **Language**: Rust
- **Dependencies**: atty (for detecting stdin type)

## Features

✅ **Fast** - Native binary compiled with Rust, instant startup  
✅ **Simple** - Single executable, no external dependencies  
✅ **Flexible** - Supports command-line arguments and pipe input  
✅ **Bidirectional** - Perfect reversibility with one-to-one mapping  
✅ **Case-preserving** - Uppercase and lowercase are preserved during conversion  

