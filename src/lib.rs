/*!
A debug tool which writes logs into file. Timestamp & pid will be added to
the logs.

# Usage

```rust
use tlog::tlog;

tlog!("{} = {}", "5 x 7", 5 * 7);
```

Logs will be written to file `/tmp/t.log` unless changed via env `TMP_LOG_FILE`.

```
$ cat /tmp/t.log
[2022-09-05 11:10:31.763][15235] 5 x 7 = 35
```
*/

use std::fmt;
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq)]
pub struct DateTime {
    odt: OffsetDateTime,
}

impl DateTime {
    pub fn now() -> Self {
        let odt: OffsetDateTime;
        match OffsetDateTime::now_local() {
            Ok(dt) => {
                odt = dt;
            }
            Err(_) => {
                odt = OffsetDateTime::now_utc();
            }
        }
        DateTime { odt }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            self.odt.year(),
            self.odt.month() as u8,
            self.odt.day(),
            self.odt.hour(),
            self.odt.minute(),
            self.odt.second(),
            self.odt.millisecond(),
        )
    }
}

#[macro_export]
macro_rules! tlog {
    ($fmt:expr) => (
        use std::io::Write as _;

        let msg = $fmt;
        let default_log_file = String::from("/tmp/t.log");
        let log_file = if let Ok(x) = std::env::var("TMP_LOG_FILE") {
            if x.is_empty() { default_log_file } else { x.clone() }
        } else {
            default_log_file
        };

        let mut cfile;
        match std::fs::OpenOptions::new().append(true).create(true).open(&log_file) {
            Ok(x) => cfile = x,
            Err(e) => {
                println!("_tlog: open error: {}: {}", &log_file, e);
                return;
            }
        }
        let pid = unsafe { libc::getpid() };
        let now = tlog::DateTime::now();
        let msg = format!("[{}][{}] {}", now, pid, msg);
        let msg = if msg.ends_with('\n') { msg } else { format!("{}\n", msg) };
        match cfile.write_all(msg.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("_tlog: write_all failed: {}", e);
                return;
            }
        }
    );

    ($fmt:expr, $($arg:tt)*) => (
        let msg = format!($fmt, $($arg)*);
        tlog!(&msg);
    );
}

pub fn type_name<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
