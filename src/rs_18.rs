use std::{
    error::Error,
    io::{self, BufRead, ErrorKind, Read, Write},
    path::PathBuf,
};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

// size 和 ?Sized: 在编译时是否已知大小, Vec<T> 是 Sized, 在编译时就知道在栈上的内存大小
// [T]就是不知道大小， 长度不知道, 只有在运行时才知道, 也就是在堆上分配内存
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: ?Sized + Read,
    W: ?Sized + Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

// reader.read_to_end(&mut buf)?;
// reader.read_to_string(&mut string)?;
// reader.read_exact(&mut buf)?;
// reader.bytes();
// reader.chain(reader1);
// reader.take(10);

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => return Err(From::from("Usage: grep PATTERN FILE...")),
    };

    let files = args.map(PathBuf::from).collect::<Vec<_>>();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let file = std::fs::File::open(file)?;
            let reader = io::BufReader::new(file);
            grep(&target, reader)?;
        }
    }

    Ok(())
}
