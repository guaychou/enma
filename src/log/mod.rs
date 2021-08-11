use {env, figlet_rs::FIGfont, log::info};

pub fn log_init(path: &str) {
    log4rs::init_file(path, Default::default()).unwrap();
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("Enma");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    info!("Starting Enma version: {}", env!("CARGO_PKG_VERSION"))
}
