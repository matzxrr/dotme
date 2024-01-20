use std::panic::{set_hook, PanicInfo};

pub fn setup() {
    set_hook(Box::new(|info: &PanicInfo| {
        let output = handle_dump(info);
        eprintln!("{}", output);
    }));
}

fn handle_dump(panic_info: &PanicInfo) -> String {
    let mut out = String::new();
    let message = match (
        panic_info.payload().downcast_ref::<&str>(),
        panic_info.payload().downcast_ref::<String>(),
    ) {
        (Some(s), _) => Some(s.to_string()),
        (_, Some(s)) => Some(s.to_string()),
        (None, None) => None,
    };
    let cause = match message {
        Some(m) => m,
        None => "Unknown".into(),
    };
    eprintln!("{}", cause);
    match panic_info.location() {
        Some(location) => out.push_str(&format!(
            "Panic occurred in file '{}' at line {}\n",
            location.file(),
            location.line(),
        )),
        None => out.push_str("Panic location unknown.\n"),
    };
    out
}

pub enum PanicStyle {
    Debug,
    Human,
}

impl Default for PanicStyle {
    fn default() -> Self {
        match std::env::var("RUST_BACKTRACE") {
            Ok(_) => PanicStyle::Debug,
            Err(_) => PanicStyle::Human,
        }
    }
}
