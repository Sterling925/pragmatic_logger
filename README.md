# pragmatic_logger

## About

This is only intended as an experimental project, and not for production use. 

Logger that logs data in a buffer, 
but only writes buffer to file on error or crash.

Buffer and file output is handled in a separate thread 
minimizing timing impact on application.

std::sync::MPSC is used for communication from log senders to log message receiver.


## License

Licensed under the Apache License, Version 2.0 LICENSE-APACHE or
<https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
LICENSE-MIT or <https://opensource.org/licenses/MIT>, at your
option.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


## Build

Note: Building tested with rustc version 1.68.2

1. Clone repo
1. In cloned repo run `cargo build`

## Tests

To run tests, the environment variable `TEST_DIRECTORY` must be set to a directory where the tests can write files.

This can be done by adding the following to `.cargo/config.toml`, 
and replacing `/media/ramdisk` with your desired location.

```toml
[env]
# Location that can be written to for test execution
TEST_DIRECTORY = "/media/ramdisk"
```

Once the above is complete, run `cargo test`. 


## Add to a project

Clone PragmaticLogger to folder in or beside project that needs logging.

Update target projects Cargo.toml to include the following dependency.
Replace `<path to pragmatic_logger>` with location PragmaticLogger was cloned to.
```toml
[dependencies]
pragmatic_logger = { path = "<path to pragmatic_logger>", version = "0.5.0" }
```

## Use

1. Build logger producing a `LogSender` and spawning a background `LogReceiver` thread.
    * Specify minimum level of log message to buffer
    * Specify minimum log level to cause buffered messages to be written to log file
    * Specify location of log file
    * Size of buffer
1. Launch main application passing in a cloned `LogSender` for logging. 
1. Use the `LogSender` to send messages.
1. Clone the `LogSender` as needed to pass into additional threads or contexts.
1. When program is done and ready to exit call ``LogSender::shutdown()` to close the logger such that it knows the program did not panic.


## Example

```rust

use pragmatic_logger::{log_sender::LogSender, Level, BufferSize, build_logger};

fn my_program(log: LogSender){
    log.info_str("Running my program");
    // do amazing stuff...
}

fn main() -> Result::<(),_>{
    // Make logger instance

    const LOG_LOCATION : &'static str = "/media/ramdisk/my_program_log.txt";

    let log = build_logger(LOG_LOCATION, 
        Level::Trace, 
        Level::Warn, 
        BufferSize::Size128)?;

    log.info_str("Running");

    // Launch program
    let my_program_logger = log.clone();
    let handle = std::thread::spawn(move || {
        my_program( my_program_logger );
    });

    // Clean up
    if let Err(e) = handle.join(){
        let error_message = format!("my_program thread returned: {:?}", e);
        log.error_string(error_message);
    }

    log.shutdown();

}

```








