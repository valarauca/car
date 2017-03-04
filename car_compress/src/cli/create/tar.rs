
use super::{
  App,
  SubCommand,
  ArgMatches,
  Arg,
  Operation,
  Comp,
  Write,
  File,
  PathBuf,
};

pub fn build<'a>() -> App<'static,'a> {
  SubCommand::with_name("tar")
    .about("Create a tar file with no compression")
}

pub fn get(x: &ArgMatches) -> Operation {
  Operation::Create(
    Comp::Tar(
      {
        let path = x.value_of("output").unwrap();
        match File::create(&path) {
          Ok(x) => x,
          Err(e) => {
            println!("Could not create output {}",&path);
            println!("Error {:?}",e);
            ::std::process::exit(1)
          }
        }
      }
    ),
    x.values_of("file")
    .unwrap()
    .map(PathBuf::from)
    .collect()
  )
}


