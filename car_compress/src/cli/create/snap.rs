

use super::{
  Format,
  Quality,
  App,
  SubCommand,
  ArgMatches,
  Arg,
  Operation,
  Comp,
  File,
  PathBuf,
  valid_item,
  item_exists
};

pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("snappy")
    .about("Create a tar file with snappy compression")
    .arg(Arg::with_name("file")
      .short("f")
      .long("file")
      .takes_value(true)
      .multiple(true)
      .value_name("FILE/DIR")
      .required(true)
      .validator(valid_item)
      .next_line_help(true)
      .global(true)
      .help("what to tar"))
    .arg(Arg::with_name("output")
      .short("o")
      .long("out")
      .takes_value(true)
      .multiple(false)
      .value_name("OUTFILE")
      .required(true)
      .validator(item_exists)
      .global(true)
      .help("tarball output"))
}

pub fn get(x: &ArgMatches) -> Operation {
  Operation::Create(
      {
        let path = x.value_of("output").unwrap();
        let w = match File::create(&path) {
          Ok(x) => x,
          Err(e) => {
            println!("Could not create output {}",&path);
            println!("Error {:?}",e);
            ::std::process::exit(1)
          }
        };
        match Comp::from_format(
          Format::Snappy(Quality::Default),
          w ) {
          Ok(x) => x,
          Err(e) => {
            println!("Building snappy compressor failed");
            println!("{:?}",e);
            ::std::process::exit(1);
          }
        }
      },
    x.values_of("file")
    .unwrap()
    .map(PathBuf::from)
    .collect()
  )
}

