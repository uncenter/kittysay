use clap::Parser;

#[derive(Parser)]
struct Cli {
    message: String,
}

fn word_wrap(paragraph: &str, line_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();

    for word in paragraph.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else {
            let potential_line = format!("{} {}", current_line, word);
            if potential_line.len() <= line_length {
                current_line = potential_line;
            } else {
                result.push(current_line.clone());
                current_line.clear();
                current_line.push_str(word);
            }
        }
    }

    result.push(current_line);

    return result;
}

fn find_longest_string(strings: Vec<String>) -> Option<String> {
    let mut longest_string: Option<String> = None;

    for s in strings {
        match longest_string {
            Some(ref longest) if s.len() > longest.len() => {
                longest_string = Some(s);
            }
            None => {
                longest_string = Some(s);
            }
            _ => {}
        }
    }

    longest_string
}

fn main() {
    let args = Cli::parse();
    let mut lines = word_wrap(&args.message, 45);
    let longest = find_longest_string(lines.clone()).unwrap();

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
        "-".repeat(longest.len()),
        match lines.clone().len() == 1 {
            true => format!("< {} >", lines.clone()[0]),
            false => {
                let mut result = format!(
                    "/ {}{}\\",
                    lines[0],
                    " ".repeat(longest.len() - lines[0].len() + 1)
                );
                lines.remove(0);
                let last = lines.pop().unwrap();

                for line in lines {
                    result = format!(
                        "{}\n| {}{}|",
                        result,
                        line,
                        " ".repeat(longest.len() - line.len() + 1)
                    );
                }

                format!(
                    "{}\n\\ {}{}/",
                    result,
                    last,
                    " ".repeat(longest.len() - last.len() + 1)
                )
            }
        },
        "-".repeat(longest.len())
    );
}
