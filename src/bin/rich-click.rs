use rich_click_rs::list_themes;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--themes" || a == "themes") {
        for name in list_themes() {
            println!("{name}");
        }
        return;
    }
    eprintln!("rich-click-rs: use --themes to list available themes");
}
