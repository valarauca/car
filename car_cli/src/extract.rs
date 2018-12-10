
use super::{io, Read, Operation, App, Arg, ArgMatches, SubCommand, PathBuf, Path, Regex,
            RegexFault, Entry};

use std::io::Write;
use std::fs::File;

fn valid_path(x: String) -> Result<(), String> {
    let p = PathBuf::from(&x);
    match (p.exists(), p.is_file()) {
        (true, true) => Ok(()),
        (false, _) => Err(format!("Cannot process {} it does not exist", &x)),
        (true, false) => Err(format!("Cannot process {} it is a directory or link", &x)),
    }
}
fn valid_regex(x: String) -> Result<(), String> {
    match Regex::new(&x) {
        Ok(_) => Ok(()),
        Err(RegexFault::CompiledTooBig(val)) => Err(format!(
            "Input regex is too large. Set size limit {:?}",
            val
        )),
        Err(RegexFault::Syntax(s)) => Err(format!("Regex Syntax Error: {}", s)),
        Err(_) => Err(format!("Regex Syntax Error. Source undocumented :(")),
    }
}
fn valid_dir(x: String) -> Result<(), String> {
    let p = PathBuf::from(&x);
    match (p.exists(), p.is_dir()) {
        (true, true) => Ok(()),
        (false, _) => Err(format!("Cannot extract to {} it does not exist", &x)),
        (true, false) => Err(format!("Cannot extract to {} it is file or link", &x)),
    }
}

/// Build the `extract` subcommand
pub fn build<'a>() -> App<'static, 'a> {
    SubCommand::with_name("extract")
        .about("Extract contents of a tar")
        .arg(
            Arg::with_name("file")
                .index(1)
                .takes_value(true)
                .multiple(false)
                .value_name("INFILE")
                .required(true)
                .validator(valid_path)
                .next_line_help(true)
                .help("file to read"),
        )
        .arg(
            Arg::with_name("regex")
                .short("r")
                .long("regex")
                .takes_value(true)
                .multiple(false)
                .value_name("REGEX")
                .validator(valid_regex)
                .next_line_help(true)
                .help("Only files which match REGEX will be extracted"),
        )
        .arg(
            Arg::with_name("out")
                .index(2)
                .takes_value(true)
                .multiple(false)
                .value_name("OUTPATH")
                .validator(valid_dir)
                .next_line_help(true)
                .help("Where to extract too"),
        )
        .arg(
            Arg::with_name("xattrs")
                .long("xattrs")
                .takes_value(false)
                .next_line_help(true)
                .help("Indicate to preserve xattrs (Unix only)"),
        )
        .arg(
            Arg::with_name("perms")
                .long("perms")
                .takes_value(false)
                .next_line_help(true)
                .help("Indicate to perserve perms (Unix only)"),
        )
}

/// Extraction logic
pub fn exec<R: Read>(
    entry: &mut Entry<R>,
    regex: &Option<Regex>,
    to: &Option<PathBuf>,
    xattrs: bool,
    perms: bool,
) -> io::Result<()> {
    let flag = match regex {
        &Option::None => true,
        &Option::Some(ref regex) => {
            let path = entry.path()?;
            match path.file_name() {
                Option::None => false,
                Option::Some(f_name) => {
                    match f_name.to_str() {
                        Option::None => false,
                        Option::Some(f_name_str) => regex.is_match(f_name_str),
                    }
                }
            }
        }
    };
    if flag {
        #[cfg(unix)]
        {
            entry.set_unpack_xattrs(xattrs);
            entry.set_preserve_permissions(perms);
        }
        unpack(to, entry)?;
    }
    Ok(())
}
/// Get entry path
fn entry_path<R: Read>(entry: &Entry<R>) -> io::Result<String> {
    #[cfg(unix)]
    {
        Ok(entry.path()?.to_string_lossy().replace("\\", "/"))
    }
    #[cfg(windows)]
    {
        Ok(entry.path()?.to_string_lossy().replace("/", "\\"))
    }
}
fn build_path<P: AsRef<Path>>(path: P) -> io::Result<File> {
    use std::fs::{create_dir_all, File};
    use std::path::Component;
    use std::io::{Error, ErrorKind};
    let mut complete = PathBuf::new();
    let mut peek = path.as_ref().components().peekable();
    loop {
        let curr = peek.next();
        let next = peek.peek();
        match (&curr, &next) {
            (&Option::Some(Component::CurDir), _) => continue,
            (&Option::None, _) => {
                return Err(Error::new(ErrorKind::InvalidData, "Path has no length"))
            }
            (&Option::Some(Component::Prefix(_)), _) |
            (_, &Option::Some(&Component::Prefix(_))) |
            (&Option::Some(Component::RootDir), _) |
            (_, &Option::Some(&Component::RootDir)) => {
                return Err(Error::new(ErrorKind::InvalidData, "Must be relative path"))
            }
            (&Option::Some(Component::ParentDir), _) |
            (_, &Option::Some(&Component::ParentDir)) => {
                return Err(Error::new(ErrorKind::InvalidData, "URHAXSUX"))
            }
            (&Option::Some(Component::Normal(ref osstr)), &Option::Some(_)) => {
                complete.push(osstr);
                create_dir_all(&complete)?;
            }
            (&Option::Some(Component::Normal(ref osstr)), &Option::None) => {
                complete.push(osstr);
                return File::create(&complete);
            }
        }
    }
}

/// This handles unpacking
fn unpack<R: Read>(to: &Option<PathBuf>, entry: &mut Entry<R>) -> io::Result<()> {
    let path = entry_path(entry)?;
    match to {
        &Option::None => {
            let mut f = build_path(&path)?;
            let mut v = Vec::with_capacity(4096);
            entry.read_to_end(&mut v)?;
            f.write_all(v.as_slice())?;
            f.flush()?;
            Ok(())
        }
        &Option::Some(ref p) => {
            let mut pathlike = PathBuf::from(p);
            pathlike.push(&path);
            let mut f = build_path(&pathlike)?;
            let mut v = Vec::with_capacity(4096);
            entry.read_to_end(&mut v)?;
            f.write_all(v.as_slice())?;
            f.flush()?;
            Ok(())
        }
    }
}

#[cfg(unix)]
pub fn get(x: &ArgMatches) -> Operation {
    Operation::Extract(
        PathBuf::from(x.value_of("file").unwrap()),
        match x.value_of("regex") {
            Option::None => None,
            Option::Some(r) => Regex::new(&r).ok(),
        },
        match x.value_of("out") {
            Option::None => None,
            Option::Some(o) => Some(PathBuf::from(o)),
        },
        x.is_present("xattrs"),
        x.is_present("perms"),
    )
}

#[cfg(windows)]
pub fn get(x: &ArgMatches) -> Operation {
    Operation::Extract(
        PathBuf::from(x.value_of("file").unwrap()),
        match x.value_of("regex") {
            Option::None => None,
            Option::Some(r) => Regex::new(&r).ok(),
        },
        match x.value_of("out") {
            Option::None => None,
            Option::Some(o) => Some(PathBuf::from(o)),
        },
        false,
        false,
    )
}
