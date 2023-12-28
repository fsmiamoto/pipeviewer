use std::io::{self, BufWriter, Result, ErrorKind, Write};
use std::fs::File;


pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer : Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))

    };

    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            // This means, gracefully exit
            return Ok(false);
        }
        return Err(e);
    }

    Ok(true)
}
