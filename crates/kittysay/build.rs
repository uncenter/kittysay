use std::env;

fn main() {
	println!("cargo::rustc-check-cfg=cfg(test_no_tty)");
	// Set test_no_tty cfg attribute based on TEST_NO_TTY variable, for the purposes of testing without a TTY.
	if env::var("TEST_NO_TTY").unwrap_or_default() == "1" {
		println!("cargo::rustc-cfg=test_no_tty");
	}
}
