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
  get_comp_level,
  valid_item,
  item_exists
};

pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("brotli")
    .about("Create a tar file with brotli compression")
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
    .arg(Arg::with_name("slow")
      .long("slow")
      .takes_value(false)
      .global(true)
      .conflicts_with("fast")
      .help("sets the slow compression mode"))
    .arg(Arg::with_name("fast")
      .long("fast")
      .takes_value(false)
      .global(true)
      .conflicts_with("slow")
      .help("sets the fast compression mode"))
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
          Format::Brotli(get_comp_level(x)),
          w ) {
          Ok(x) => x,
          Err(e) => {
            println!("Building brotli compressor failed");
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

