# pragmatic_logger

## About

This is only intended as an experimental project, and not for production use. 

Logger that logs data in a buffer, 
but only writes buffer to file on error or crash.

Buffer and file output is handled in a separate thread 
minimizing timing impact on application.

std::sync::MPSC is used for communication from log senders to log message receiver.


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

## Tests

To run tests, the environment variable `TEST_DIRECTORY` must be set to a directory where the tests can write files.

This can be done by adding the following to `.cargo/config.toml`, 
and replacing `/media/ramdisk` with your desired location.

```toml
[env]
# Location that can be written to for test execution
TEST_DIRECTORY = "/media/ramdisk"
```

Once the above is complete, run the usual `cargo test`. 

## Example

```rust

fn my_program(log: pragmatic_logger::log_sender::LogSender){
    log.info_str("Running my program");
    // do amazing stuff...
}

fn main() -> Result::<(),_>{
    // Make logger instance

    const LOG_LOCATION : &'static str = "/media/ramdisk/my_program_log.txt";

    let log = pragmatic_logger::build_logger(LOG_LOCATION, 
        pragmatic_logger::Level::Trace, 
        pragmatic_logger::Level::Warn, 
        pragmatic_logger::BufferSize::Size128)?;

    log.info_str("Running");

    // Launch program
    let my_program_logger = log.clone();
    let handle = std::thread::spawn(move || {
        my_program( my_program_logger );
    });

    // Clean up
    if let Err(e) = handle.join(){
        let error_message = format!("my_program returned: {:?}", e);
        log.error_string(error_message);
    }

    log.shutdown();

}

```








