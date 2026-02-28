extern crate textwrap;
extern crate unicode_width;
use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

pub static KITTY: &str = "
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ";

pub struct FormatOptions {
	pub think: bool,
	pub width: u16,
}

struct Chars {
	arrow: &'static str,
	top: &'static str,
	bottom: &'static str,
	left: &'static str,
	right: &'static str,
	single_left: &'static str,
	single_right: &'static str,
	angled_up_right: &'static str,
	angled_up_left: &'static str,
	angled_down_right: &'static str,
	angled_down_left: &'static str,
}

static SAY_CHARS: Chars = Chars {
	arrow: "\\",
	top: "-",
	bottom: "-",
	left: "|",
	right: "|",
	single_left: "<",
	single_right: ">",
	angled_up_right: "/",
	angled_up_left: "\\",
	angled_down_right: "\\",
	angled_down_left: "/",
};

static THINK_CHARS: Chars = Chars {
	arrow: "○",
	top: "⏜",
	bottom: "⏝",
	left: "(",
	right: ")",
	single_left: "(",
	single_right: ")",
	angled_up_right: "⎛",
	angled_up_left: "⎞",
	angled_down_right: "⎝",
	angled_down_left: "⎠",
};

#[must_use]
pub fn generate(message: &str, format_opts: &FormatOptions) -> String {
	let think = format_opts.think;
	let width = format_opts.width;

	let chars = if think { &THINK_CHARS } else { &SAY_CHARS };
	let mut lines = wrap(message, width as usize);
	let longest = lines.iter().map(|line| line.width()).max().unwrap();

	format!(
		"
  {}
{}
  {}
  {}
    {}",
		chars.top.repeat(longest),
		if lines.len() == 1 {
			format!("{} {} {}", chars.single_left, lines[0], chars.single_right)
		} else {
			let mut result = format!(
				"{} {}{}{}",
				chars.angled_up_right,
				lines[0],
				" ".repeat(longest - lines[0].width() + 1),
				chars.angled_up_left
			);
			lines.remove(0);
			let last = lines.pop().unwrap();

			for line in lines {
				result = format!(
					"{}\n{} {}{}{}",
					result,
					chars.left,
					line,
					" ".repeat(longest - line.width() + 1),
					chars.right,
				);
			}

			format!(
				"{}\n{} {}{}{}",
				result,
				chars.angled_down_right,
				last,
				" ".repeat(longest - last.width() + 1),
				chars.angled_down_left,
			)
		},
		chars.bottom.repeat(longest),
		chars.arrow,
		chars.arrow,
	)
}

pub fn print(message: &str, format_opts: &FormatOptions) -> String {
	format!("{}{}", generate(message, &format_opts), KITTY)
}
