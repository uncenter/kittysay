use color_eyre::eyre::Result;

use clap::Parser;
use clap_stdin::MaybeStdin;

use crossterm::terminal;

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
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();
	let (cols, _) = terminal::size()?;

	let width = args.width.unwrap_or(45).min(cols.saturating_sub(5));

	let format_opts = kittycore::FormatOptions {
		think: args.think,
		width,
	};

	println!("{}", kittycore::print(&args.message, &format_opts));

	Ok(())
}
