extern crate clap;

mod cmdget;

fn main() {
    let params = clap::App::new("git-credential-docker-store")
        .version("v0.1.0")
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .help("credentials file")
                .takes_value(true),
        )
        .subcommand(clap::SubCommand::with_name("get").about("get credentials"))
        .subcommand(clap::SubCommand::with_name("store").about("store credentials - silently dropped"))
        .get_matches();

    let file_name = params.value_of("file").unwrap_or("./.git-credentials");

	match params.subcommand_name() {
		Some("get") => {
			match cmdget::execute(file_name) {
				Ok(s) => println!("{}", s),
				Err(e) => println!("{}", e),
			};
		},
		_ => {
			// silently drop
		}
	}
    // git exptects empty line at the end
    println!("")
}
