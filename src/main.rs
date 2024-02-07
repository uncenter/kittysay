use color_eyre::eyre::Result;
use std::mem;

use clap::Parser;
use clap_stdin::MaybeStdin;

use crossterm::terminal;

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

#[derive(Parser)]
struct Cli {
	message: MaybeStdin<String>,
	#[clap(long, short)]
	width: Option<u16>,
	#[clap(long, short)]
	think: Option<bool>,
}

fn word_wrap(paragraph: &str, line_length: usize) -> Vec<String> {
	let mut result = Vec::new();
	let mut current_line = String::new();

	for word in paragraph.split_whitespace() {
		if current_line.is_empty() {
			current_line.push_str(word);
		} else {
			let potential_line = format!("{current_line} {word}");
			if potential_line.len() <= line_length {
				current_line = potential_line;
			} else {
				result.push(mem::take(&mut current_line));
				current_line.push_str(word);
			}
		}
	}

	result.push(current_line);
	result
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();
	let (cols, _) = terminal::size()?;

	let width = args.width.unwrap_or(45).min(cols.saturating_sub(5));
	let chars = if args.think.unwrap_or(false) {
		&THINK_CHARS
	} else {
		&SAY_CHARS
	};

	let mut lines = word_wrap(&args.message, width as usize);
	let longest = lines.iter().map(String::len).max().unwrap();

	println!(
		"
  {}
{}
  {}
  {}
    {}
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ
    ",
		chars.top.repeat(longest),
		if lines.len() == 1 {
			format!("{} {} {}", chars.single_left, lines[0], chars.single_right)
		} else {
			let mut result = format!(
				"{} {}{}{}",
				chars.angled_up_right,
				lines[0],
				" ".repeat(longest - lines[0].len() + 1),
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
					" ".repeat(longest - line.len() + 1),
					chars.right,
				);
			}

			format!(
				"{}\n{} {}{}{}",
				result,
				chars.angled_down_right,
				last,
				" ".repeat(longest - last.len() + 1),
				chars.angled_down_left,
			)
		},
		chars.bottom.repeat(longest),
		chars.arrow,
		chars.arrow,
	);

	Ok(())
}
