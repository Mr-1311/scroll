use crate::commands::{build, new, serve, watch};
use clap::{App, Arg};

pub fn app() {
    let matches = App::new("Scroll")
        .version("0.1.0")
        .author("Metin Ur <metinur.1311@gmail.com>")
        .about("Magical Static Site Generator")
        .subcommand(
            App::new("new")
                .about("Create new site.")
                .arg(Arg::with_name("site_name").index(1).required(true)),
        )
        .subcommand(
            App::new("build").about("Build the site.").arg(
                Arg::with_name("then-serve")
                    .short("s")
                    .long("then-serve")
                    .help("Start serve and watch on local after build the site."),
            ),
        )
        .subcommand(
            App::new("serve")
                .about("Serve static site for local usage and test.")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .help("Specify port to serve.")
                        .takes_value(true)
                        .default_value("1919"),
                )
                .arg(
                    Arg::with_name("no-watch")
                        .short("n")
                        .long("no-watch")
                        .help("Don't watch file changes, only serve."),
                ),
        )
        .subcommand(App::new("watch").about("Only Watch and Rebuild files."))
        .get_matches();

    match matches.subcommand() {
        ("build", Some(matches)) => build(matches.is_present("then-serve")),
        ("new", Some(matches)) => {
            new(matches.value_of("site_name").unwrap());
        }
        ("serve", Some(matches)) => serve(
            matches.value_of("port").unwrap(),
            matches.is_present("no-watch"),
        ),
        ("watch", Some(_)) => watch(),
        ("", None) => {
            println!("No subcommand was used, 'scroll -h' or 'scroll --help' for more information.")
        }
        _ => println!("unimplemented subcommend!"),
    }
}
