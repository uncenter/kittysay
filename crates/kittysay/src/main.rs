use color_eyre::eyre::Result;

use clap::Parser;
use clap_stdin::MaybeStdin;

use anstream::{adapter::strip_str, println};
use crossterm::terminal;

use kittycore::{generate, FormatOptions, KITTY};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
	#[clap(default_value = "-")]
	message: MaybeStdin<String>,
	/// Set width of speech/thought bubble
	#[clap(long, short)]
	width: Option<u16>,
	/// Enable kittythink mode (thought bubble)
	#[clap(long, short)]
	think: bool,
	/// Use custom colors. The first colors the message, the second colors the cat
	#[arg(long, short, num_args(2))]
	colors: Option<Vec<u8>>,
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();
	let cols: u16 = 80;

	let width = args.width.unwrap_or(45).min(cols.saturating_sub(5));

	let format_opts = FormatOptions {
		think: args.think,
		width,
	};

	let msg = generate(&strip_str(&args.message).to_string(), &format_opts);

	let mut msg_color = console::Color::White;
	let mut cat_color = console::Color::White;
	if let Some(colors) = args.colors {
		msg_color = console::Color::Color256(colors[0]);
		cat_color = console::Color::Color256(colors[1]);
	}

	println!(
		"{}{}",
		console::style(msg).fg(msg_color),
		console::style(KITTY).fg(cat_color)
	);

	Ok(())
}
