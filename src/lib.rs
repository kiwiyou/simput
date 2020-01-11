#[doc(hidden)]
pub fn _read_stdin_until_delim() -> std::io::Result<Vec<u8>> {
    use std::io::BufRead;
    let mut buffer = Vec::new();
    let stdin_raw = std::io::stdin();
    let mut stdin = stdin_raw.lock();
    loop {
        let (done, used) = {
            let available = match stdin.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            let space = memchr::memchr(b' ', available);
            let newline = memchr::memchr(b'\n', available);
            let count = match (space, newline) {
                (Some(s), Some(n)) => {
                    let i = std::cmp::min(s, n);
                    Some(i)
                }
                _ => space.or(newline),
            };
            if let Some(count) = count {
                buffer.extend_from_slice(&available[..count]);
                (true, count + 1)
            } else {
                buffer.extend_from_slice(available);
                (false, available.len())
            }
        };
        stdin.consume(used);
        if done || used == 0 {
            return Ok(buffer);
        }
    }
}

/// Reads input from standard input, split by ascii space and newline character
///
/// # Example
///
/// ```
/// use simput::input;
/// let (line_of_string, string_piece, integer, decimal) = input!(Line, String, i32, f32);
#[macro_export]
macro_rules! input {
    (inner Line) => {{
        use std::io::BufRead;
        let mut buffer = Vec::new();
        std::io::stdin().lock().read_until(b'\n', &mut buffer).unwrap();
        std::str::from_utf8(&buffer).unwrap().trim_end().to_string()
    }};
    (inner $t:ty) => {{
        let buffer = simput::_read_stdin_until_delim().unwrap();
        std::str::from_utf8(&buffer).unwrap().parse::<$t>().unwrap()
    }};
    ($($t:tt),*) => (($(simput::input!(inner $t)),*));
}
