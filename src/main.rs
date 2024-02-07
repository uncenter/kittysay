use color_eyre::eyre::Result;
use std::mem;

use clap::Parser;
use clap_stdin::MaybeStdin;

use crossterm::terminal;

#[derive(Parser)]
struct Cli {
	message: MaybeStdin<String>,
	#[clap(long, short)]
	width: Option<u16>,
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

	let mut lines = word_wrap(&args.message, width as usize);
	let longest = lines.iter().map(String::len).max().unwrap();

	println!(
		"
  {}
{}
  {}
  \\
    \\
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ
    ",
		"-".repeat(longest),
		if lines.len() == 1 {
			format!("< {} >", lines[0])
		} else {
			let mut result = format!(
				"/ {}{}\\",
				lines[0],
				" ".repeat(longest - lines[0].len() + 1)
			);
			lines.remove(0);
			let last = lines.pop().unwrap();

			for line in lines {
				result = format!(
					"{}\n| {}{}|",
					result,
					line,
					" ".repeat(longest - line.len() + 1)
				);
			}

			format!(
				"{}\n\\ {}{}/",
				result,
				last,
				" ".repeat(longest - last.len() + 1)
			)
		},
		"-".repeat(longest)
	);

	Ok(())
}
