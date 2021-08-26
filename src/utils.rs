use colored::Colorize;

pub fn abort(msg: &str) {
    println!("{}", msg);
    std::process::exit(0);
}

pub fn to_color(mut num: String) -> String {
    if !num.starts_with("+") && !num.starts_with("-") { num = format!("+{}", num) }
    
    if num.contains("+") { return num.green().to_string() }
    else if num.contains("-") && num.len() > 1 { return num.red().to_string() }
    else { return num }
}