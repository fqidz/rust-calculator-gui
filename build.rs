// cargo rustc --release --bin rust-calculator-gui -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
}
