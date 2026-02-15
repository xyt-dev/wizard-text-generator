# ꝡizardTextGenerator

中文 | [English](./README.md)

一个简单、快速、方便的终端文本转换工具，将普通字母转换为ꝡɨʐǟʀɖ字符。

## 字符映射

### 小写 (a-z)

| 普通字母 | a | b | c | d | e | f | g | h | i | j | k | l | m | n | o | p | q | r | s | t | u | v | w | x | y | z |
|---------|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **魔法字符** | **ǟ** | **ɮ** | **ƈ** | **ɖ** | **ɛ** | **ʄ** | **ɢ** | **ɦ** | **ɨ** | **ʝ** | **ӄ** | **ʟ** | **ʍ** | **ռ** | **օ** | **ք** | **զ** | **ʀ** | **ֆ** | **ȶ** | **ʊ** | **ʋ** | **ա** | **Ӽ** | **ʏ** | **ʐ** |

### 大写 (A-Z)

| 普通字母 | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
|---------|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **魔法字符** | **Ǟ** | **Ɓ** | **Ƈ** | **Ɖ** | **Ɛ** | **Ƒ** | **Ɠ** | **Ɦ** | **Ɨ** | **Ɉ** | **Ӄ** | **Ł** | **Ɱ** | **Ռ** | **Ø** | **Ք** | **Զ** | **Ʀ** | **Ֆ** | **Ŧ** | **Ʊ** | **Ʋ** | **Ꝡ** | **Ӿ** | **Ƴ** | **Ƶ** |

> **说明**：所有映射都是**一对一**（双射）的，确保完美可逆。修改的字符（B,F,G,J,L,M,O,T,W,X,Y,Z）使用视觉相似的 Unicode 字形以消除歧义。

## 安装

### 从源码编译

```bash
cargo build --release
```

编译后的可执行文件位于 `target/release/wizard_text_generator`

### 安装到系统

#### 方法 1：使用 Cargo（推荐）

```bash
cargo install --path .
```

这会将二进制文件安装到 `~/.cargo/bin/wizard_text_generator`（确保 `~/.cargo/bin` 在你的 PATH 中）

```bash
# 可选：创建更短的别名
ln -s ~/.cargo/bin/wizard_text_generator ~/.cargo/bin/wtg
```

#### 方法 2：手动安装到 `/usr/local/bin`

```bash
# 编译完成后
sudo cp target/release/wizard_text_generator /usr/local/bin/

# 可选：创建更短的别名
sudo ln -s /usr/local/bin/wizard_text_generator /usr/local/bin/wtg
```

#### 方法 3：添加到 PATH（用户级别）

```bash
# 复制到 ~/.local/bin（如果目录不存在则创建）
mkdir -p ~/.local/bin
cp target/release/wizard_text_generator ~/.local/bin/

# 将 ~/.local/bin 添加到 PATH（在 ~/.bashrc 或 ~/.zshrc 中）
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# 可选：创建更短的别名
ln -s ~/.local/bin/wizard_text_generator ~/.local/bin/wtg
```

#### 验证安装

```bash
# 使用完整命令名测试
wizard_text_generator "hello"

# 或使用短别名（如果已创建）
wtg "hello"
```

## 使用方法

### 1. 交互模式

直接运行命令而不带任何参数即可进入交互模式：

```bash
wizard_text_generator
# 或使用短命令
wtg
```

交互模式功能：
- **自动检测**：自动识别输入是普通文本还是魔法字符
  - 普通文本（a-z, A-Z）→ 转换为魔法字符
  - 魔法字符 → 转换回普通文本
- **简单**：无需命令，直接输入即可转换
- `Ctrl+D` - 退出

示例会话：
```
$ wtg

    ╔══════════════════════════════════════════════╗
    ║   ꝡizardTextGenerator                        ║
    ║   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━     ║
    ║   Ŧʀǟռֆʄօʀʍ ȶɛӼȶ ɨռȶօ ʍǟɢɨƈǟʟ ƈɦǟʀǟƈȶɛʀֆ     ║
    ╚══════════════════════════════════════════════╝

Type text to convert (auto-detects direction), Ctrl+D to exit

> hello world
ɦɛʟʟօ աօʀʟɖ
> ɦɛʟʟօ աօʀʟɖ
hello world
> The Quick Brown Fox
Ŧɦɛ Զʊɨƈӄ Ɓʀօառ ƑօӼ
> Ŧɦɛ Զʊɨƈӄ Ɓʀօառ ƑօӼ
The Quick Brown Fox
> ^D
Goodbye!
```

### 2. 管道输入

```bash
# 从标准输入读取
echo "hello" | wizard_text_generator
# 输出: ɦɛʟʟօ

# 转换文件内容
cat file.txt | wizard_text_generator

# 结合其他命令
echo "Rust is awesome" | wizard_text_generator
# 输出: Ʀʊֆȶ ɨֆ ǟաɛֆօʍɛ
```

### 3. 命令行参数

```bash
# 转换为魔法字符
wizard_text_generator "hello world"
# 输出: ɦɛʟʟօ աօʀʟɖ

# 反向转换（魔法字符 -> 普通字母）
wizard_text_generator -r "ɦɛʟʟօ աօʀʟɖ"
# 输出: hello world

# 大写转换
wizard_text_generator "HELLO WORLD"
# 输出: ꞪƐŁŁØ ꝠØƦŁƉ

# 混合大小写
wizard_text_generator "Hello World"
# 输出: Ɦɛʟʟօ Ꝡօʀʟɖ
```

### 4. 选项

```
-r, --reverse    反向转换（魔法字符 -> 普通字母）
-h, --help       显示帮助信息
```

## 示例

```bash
# 完整小写字母表转换
./target/release/wizard_text_generator "abcdefghijklmnopqrstuvwxyz"
# 输出: ǟɮƈɖɛʄɢɦɨʝӄʟʍռօքզʀֆȶʊʋաӼʏʐ

# 完整大写字母表转换
./target/release/wizard_text_generator "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
# 输出: ǞƁƇƉƐƑƓꞪƗɈӃŁⱮՌØՔԶƦՖŦƱƲꝠӾƳƵ

# 混合文本（非字母字符保持不变）
./target/release/wizard_text_generator "Hello, World! 123"
# 输出: Ɦɛʟʟօ, Ꝡօʀʟɖ! 123

# 反向转换
./target/release/wizard_text_generator -r "ɦɛʟʟօ"
# 输出: hello
```

## 技术栈

- **语言**: Rust
- **依赖**: atty (用于检测标准输入类型)

## 特点

✅ **快速** - Rust 编译的原生二进制，启动迅速  
✅ **简单** - 单个可执行文件，无需额外依赖  
✅ **灵活** - 支持命令行参数和管道输入  
✅ **双向** - 一对一映射，完美可逆  
✅ **保留大小写** - 转换过程保留大小写信息  

