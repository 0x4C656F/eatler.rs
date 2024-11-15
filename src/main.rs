use dir::ScanParams;

mod choice;
mod dir;
mod reatler;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let include = choice::get_types();
    let mut ignore = choice::get_ignore();
    let gitignore = reatler::parse_gitignore();
    ignore.extend(gitignore);
    let params = ScanParams { include, ignore };

    reatler::run(&args, params);
}
