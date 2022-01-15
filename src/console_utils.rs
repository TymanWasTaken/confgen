pub mod console {
	macro_rules! err {
		($msg:expr) => {
			{
				eprintln!(
					"{} {}",
					Color::Red.bold().paint("::"),
					Color::Red.paint($msg)
				);
				panic!();
			}
		};
	}

	macro_rules! prompt {
		($val:expr, $opt_name:expr, $default:expr) => {
			{
				match $default {
					Some(default) => print!(
						"{} {} {}: ",
						Color::Yellow.bold().paint("::"),
						Color::Blue.bold().paint($val),
						format!(
							"{}{}{}",
							Color::Green.bold().paint("["),
							Color::Yellow.bold().paint(default),
							Color::Green.bold().paint("]")
						)
					),
					None => print!(
						"{} {}: ",
						Color::Yellow.bold().paint("::"),
						Color::Blue.bold().paint($val)
					)
				}
				io::stdout().flush().unwrap();
				let mut value = String::new();
				io::stdin().read_line(&mut value).unwrap();
				value = value.replace("\n", "");
				let input = match value.is_empty() {
					true => match $default {
						Some(default) => default.to_owned(),
						None => console::err!(format!("Option {} does not have a default, you must enter a value for it.", $opt_name))
					},
					false => value.to_owned()
				};
				input
			}
		};
	}

	macro_rules! val {
		($name:expr, $value:expr) => {
			{
				println!(
					"{} {}: {}",
					Color::Yellow.bold().paint("::"),
					Color::Blue.bold().paint($name),
					Color::Cyan.paint(format!("{}", $value))
				);
			}
		};
	}

	macro_rules! info {
		($msg:expr) => {
			{
				println!(
					"{} {}",
					Color::Cyan.bold().paint("::"),
					Color::Green.bold().paint($msg)
				);
			}
		};
	}
	
	pub(crate) use {err, prompt, info, val};
}