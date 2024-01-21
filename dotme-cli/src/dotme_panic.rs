use std::panic::{set_hook, PanicInfo};

pub fn setup() {
    set_hook(Box::new(|info: &PanicInfo| {
        let output = handle_dump(info);
        eprintln!("{}", output);
    }));
}

#[derive(Debug)]
struct DumpData {
    origin: String,
    cause: String,
}

impl std::fmt::Display for DumpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\norigin: {}\ncause: {}\n", self.origin, self.cause)
    }
}

fn handle_dump(panic_info: &PanicInfo) -> DumpData {
    let mut origin = String::new();
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
    match panic_info.location() {
        Some(location) => origin.push_str(&format!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        )),
        None => origin.push_str("Panic location unknown."),
    };
    DumpData { origin, cause }
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
