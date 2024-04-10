use color_eyre::eyre::Result;

use clap::Parser;
use clap_stdin::MaybeStdin;

use crossterm::terminal;

use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

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
#[command(version, about)]
struct Cli {
	message: MaybeStdin<String>,
	/// Set custom width of the message box
	#[clap(long, short)]
	width: Option<u16>,
	/// Enable kittythink mode
	#[clap(long, short)]
	think: bool,
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();
	let (cols, _) = terminal::size()?;

	let width = args.width.unwrap_or(45).min(cols.saturating_sub(5));
	let chars = if args.think { &THINK_CHARS } else { &SAY_CHARS };

	let mut lines = wrap(&args.message, width as usize);
	let longest = lines.iter().map(|line| line.width()).max().unwrap();

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
	);

	Ok(())
}
