use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn cli() -> Command {
	Command::new(get_cargo_bin("kittysay"))
}

#[test]
fn test_help() {
	assert_cmd_snapshot!(cli().arg("--help"), @"
	success: true
	exit_code: 0
	----- stdout -----
	cowsay, but with a cute kitty :3

	Usage: kittysay [OPTIONS] [MESSAGE]

	Arguments:
	  [MESSAGE]  [default: -]

	Options:
	  -w, --width <WIDTH>             Set width of speech/thought bubble
	  -t, --think                     Enable kittythink mode (thought bubble)
	  -c, --colors <COLORS> <COLORS>  Use custom colors. The first colors the message, the second colors the cat
	      --tab-size <TAB_SIZE>       Set the interpreted width/length of tabs [default: 4]
	  -h, --help                      Print help
	  -V, --version                   Print version

	----- stderr -----
	");
}

#[test]
fn test_blank() {
	assert_cmd_snapshot!(cli(), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  
	<  >
	  
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_basic() {
	assert_cmd_snapshot!(cli().arg(":3"), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  --
	< :3 >
	  --
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_fortune() {
	assert_cmd_snapshot!(cli().arg(r#"What is comedy?  Comedy is the art of making people laugh without making
them puke.
		-- Steve Martin"#), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  --------------------------------------------
	/ What is comedy?  Comedy is the art of making \
	| people laugh without making                  |
	| them puke.                                   |
	\         -- Steve Martin                      /
	  --------------------------------------------
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
	assert_cmd_snapshot!(cli().arg(r#"Grabel's Law:
	2 is not equal to 3 -- not even for large values of 2."#), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  ---------------------------------------------
	/ Grabel's Law:                                 \
	|     2 is not equal to 3 -- not even for large |
	\ values of 2.                                  /
	  ---------------------------------------------
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_stdin() {
	assert_cmd_snapshot!(cli().pass_stdin("Hello World!"), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  ------------
	< Hello World! >
	  ------------
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_think() {
	assert_cmd_snapshot!(cli().arg("--think").arg("Hello World!"), @"
	success: true
	exit_code: 0
	----- stdout -----

	  ⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜
	( Hello World! )
	  ⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝
	  ○
	    ○
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_stdin_think() {
	assert_cmd_snapshot!(cli().arg("--think").pass_stdin("Hello World!"), @"
	success: true
	exit_code: 0
	----- stdout -----

	  ⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜⏜
	( Hello World! )
	  ⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝⏝
	  ○
	    ○
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}

#[test]
fn test_width() {
	assert_cmd_snapshot!(cli().arg("Hello World!").arg("--width").arg("5"), @r"
	success: true
	exit_code: 0
	----- stdout -----

	  -----
	/ Hello \
	| World |
	\ !     /
	  -----
	  \
	    \
	      ／l、
	    （ﾟ､ ｡ ７
	      l  ~ヽ
	      じしf_,)ノ

	----- stderr -----
	");
}
