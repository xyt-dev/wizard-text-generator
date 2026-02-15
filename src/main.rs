use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn create_mapping() -> (HashMap<char, char>, HashMap<char, char>) {
    // a-z 映射到魔法字符（小写）
    let normal_lower = "abcdefghijklmnopqrstuvwxyz";
    let wizard_lower = "ǟɮƈɖɛʄɢɦɨʝӄʟʍռօքզʀֆȶʊʋաӼʏʐ";
    
    // A-Z 映射到魔法字符（大写）- 消除歧义的一对一映射
    // 修改了 B,F,G,J,L,M,T,X,Y,Z 以确保每个字符都有唯一映射且跨平台兼容
    let normal_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let wizard_upper = "ǞƁƇƉƐƑƓꞪƗɈӃŁⱮՌՕՔԶƦՖŦƱƲԱӾƳƵ";
    
    let mut to_wizard = HashMap::new();
    let mut to_normal = HashMap::new();
    
    let wizard_lower_chars: Vec<char> = wizard_lower.chars().collect();
    let wizard_upper_chars: Vec<char> = wizard_upper.chars().collect();
    
    // 小写映射
    for (i, c) in normal_lower.chars().enumerate() {
        to_wizard.insert(c, wizard_lower_chars[i]);
        to_normal.insert(wizard_lower_chars[i], c);
    }
    
    // 大写映射
    for (i, c) in normal_upper.chars().enumerate() {
        to_wizard.insert(c, wizard_upper_chars[i]);
        to_normal.insert(wizard_upper_chars[i], c);
    }
    
    (to_wizard, to_normal)
}

fn convert_to_wizard(text: &str, mapping: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *mapping.get(&c).unwrap_or(&c))
        .collect()
}

fn convert_to_normal(text: &str, mapping: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *mapping.get(&c).unwrap_or(&c))
        .collect()
}

fn contains_wizard_chars(text: &str, to_normal: &HashMap<char, char>) -> bool {
    text.chars().any(|c| to_normal.contains_key(&c))
}

fn print_usage() {
    eprintln!("WizardTextGenerator - Wizard Text Converter / 魔法文本转换器");
    eprintln!();
    eprintln!("Usage / 用法:");
    eprintln!("  {} [options] <text>", env::args().next().unwrap());
    eprintln!("  echo 'text' | {}", env::args().next().unwrap());
    eprintln!();
    eprintln!("Options / 选项:");
    eprintln!("  -r, --reverse    Reverse conversion (wizard → normal) / 反向转换（魔法字符 → 普通字母）");
    eprintln!("  -h, --help       Show this help / 显示帮助信息");
    eprintln!();
    eprintln!("Examples / 示例:");
    eprintln!("  {} hello          # Output / 输出: ɦɛʟʟօ", env::args().next().unwrap());
    eprintln!("  {} -r ɦɛʟʟօ       # Output / 输出: hello", env::args().next().unwrap());
    eprintln!("  echo 'Hello World' | {}  # Output / 输出: Ɦɛʟʟօ Աօʀʟɖ", env::args().next().unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (to_wizard, to_normal) = create_mapping();
    
    // 没有参数，尝试从标准输入读取
    if args.len() == 1 {
        if atty::is(atty::Stream::Stdin) {
            // 交互模式
            eprintln!(r#"
    ╔══════════════════════════════════════════════╗
    ║   ꝡizardTextGenerator                        ║
    ║   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━     ║
    ║   Ŧʀǟռֆʄօʀʍ ȶɛӼȶ ɨռȶօ ʍǟɢɨƈǟʟ ƈɦǟʀǟƈȶɛʀֆ     ║
    ╚══════════════════════════════════════════════╝
"#);
            eprintln!("Type text to convert (auto-detects direction), Ctrl+D to exit");
            eprintln!();
            
            let stdin = io::stdin();
            
            loop {
                eprint!("> ");
                use std::io::Write;
                io::stderr().flush().unwrap();
                
                let mut line = String::new();
                match stdin.read_line(&mut line) {
                    Ok(0) => break, // EOF (Ctrl+D)
                    Ok(_) => {
                        let text = line.trim();
                        
                        if text.is_empty() {
                            continue;
                        }
                        
                        // 自动检测输入类型并转换
                        let result = if contains_wizard_chars(text, &to_normal) {
                            // 包含魔法字符，执行反向转换
                            convert_to_normal(text, &to_normal)
                        } else {
                            // 普通文本，执行正向转换
                            convert_to_wizard(text, &to_wizard)
                        };
                        println!("{}", result);
                    }
                    Err(_) => break,
                }
            }
            eprintln!("Goodbye!");
            return;
        } else {
            // 管道输入
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(text) = line {
                    println!("{}", convert_to_wizard(&text, &to_wizard));
                }
            }
            return;
        }
    }
    
    // 解析参数
    let mut reverse = false;
    let mut text_parts = Vec::new();
    
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return;
            }
            "-r" | "--reverse" => {
                reverse = true;
            }
            _ => {
                text_parts.push(arg.as_str());
            }
        }
    }
    
    if text_parts.is_empty() {
        eprintln!("错误: 请提供要转换的文本");
        eprintln!();
        print_usage();
        std::process::exit(1);
    }
    
    let text = text_parts.join(" ");
    
    let result = if reverse {
        convert_to_normal(&text, &to_normal)
    } else {
        convert_to_wizard(&text, &to_wizard)
    };
    
    println!("{}", result);
}
