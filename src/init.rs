const BANNER: &str = r#"
 _     ___   ____ _  ______ 
| |   / _ \ / ___| |/ /  _ \
| |  | | | | |   | ' /| |_) |
| |__| |_| | |___| . \|  _ < 
|_____\___/ \____|_|\_\_| \_\
"#;

pub fn display_banner() {
    println!("{}", BANNER);
}

pub fn init() {
    display_banner();
}