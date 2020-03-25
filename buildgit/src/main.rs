pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!(
        "This is version {}{}, built for {} by {}.",
        built_info::PKG_VERSION,
        built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
        built_info::TARGET,
        built_info::RUSTC_VERSION
    );

    match built_info::CI_PLATFORM {
        None => print!("It seems I've not been built on a continuous integration platform,"),
        Some(ci) => print!("I've been built on CI-platform {},", ci),
    }

    if built::util::detect_ci().is_some() {
        println!(" but I'm currently executing on one!");
    } else {
        println!(" and I'm currently not executing on one!");
    }
}

