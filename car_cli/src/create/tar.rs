
use super::{App, SubCommand, ArgMatches, Arg, Operation, Comp, Write, File, PathBuf, valid_item,
            item_exists};

use std::io::BufWriter;

pub fn build<'a>() -> App<'static, 'a> {
    SubCommand::with_name("tar")
        .about("Create a tar file with no compression")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .multiple(true)
                .value_name("FILE/DIR")
                .required(true)
                .validator(valid_item)
                .next_line_help(true)
                .global(true)
                .help("what to tar"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .takes_value(true)
                .multiple(false)
                .value_name("OUTFILE")
                .required(true)
                .validator(item_exists)
                .global(true)
                .help("tarball output"),
        )
}

pub fn get(x: &ArgMatches) -> Operation {
    Operation::Create(
        Comp::Tar({
            let path = x.value_of("output").unwrap();
            match File::create(&path) {
                Ok(x) => BufWriter::with_capacity(131072, x),
                Err(e) => {
                    println!("Could not create output {}", &path);
                    println!("Error {:?}", e);
                    ::std::process::exit(1)
                }
            }
        }),
        x.values_of("file").unwrap().map(PathBuf::from).collect(),
    )
}
