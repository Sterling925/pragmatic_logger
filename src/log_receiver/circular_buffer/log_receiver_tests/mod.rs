#![cfg(test)]

use super::super::log_common;
use super::super::super::log_receiver::BufferSize;

use super::super::circular_buffer::circular_buffer_tests::mock_writer;
use super::super::circular_buffer::circular_buffer_tests::mock_writer::MockTextFile;
use super::super::circular_buffer;
use super::super::LogReceiver;

fn spawn_mocked(
    log_dump_level: log_common::Level,
    receiver: std::sync::mpsc::Receiver<log_common::LogData>,
    log_file_path: std::path::PathBuf,
    buffer_size: BufferSize,
) -> Option<MockTextFile> {
    if log_common::Level::Off != log_dump_level {
        debug_assert!( buffer_size.is_valid() );

        let mut buffer = std::vec::Vec::<String>::with_capacity(buffer_size.value());
        for _k in 0..buffer_size.value() {
            buffer.push(String::new());
        }

        let (mock_file, text_data_writer) = mock_writer::get_mock_text_data_writer();
        let circle = circular_buffer::CircularStringsBuffer::new(buffer, text_data_writer);

        std::thread::spawn(move || {    
            let mut logger = LogReceiver::new(log_dump_level, receiver, log_file_path, circle);
            logger.execute();
        });
        Some(mock_file) // return mock file

    } else {
        None
    }
}


#[test]
fn test_init_off() {
    let fp = std::path::PathBuf::from("/media/ramdisk/test_init_off.txt");

    let (sender, receiver) = std::sync::mpsc::channel::<log_common::LogData>();

    let _mock_file = spawn_mocked(log_common::Level::Off, receiver, fp, BufferSize::Size128);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Off, ts, format!("Off line"));
    assert!(sender.send(d).is_err());
}


#[test]
fn test_init_error() {
    let fp = std::path::PathBuf::from("/media/ramdisk/test_init_error.txt");

    let (sender, receiver) = std::sync::mpsc::channel::<log_common::LogData>();

    let mut mock_file = spawn_mocked(log_common::Level::Error, receiver, fp, BufferSize::Size128).unwrap();

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 1"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 2"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 3"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Off, ts, format!("Off line"));
    assert!(sender.send(d).is_ok());

    mock_file.process_queued_messages();
    let lines = mock_file.get_mock_data();
    assert!(0 == lines.len());
}


#[test]
fn test_write_error() {
    let fp = std::path::PathBuf::from("/media/ramdisk/test_write_error.txt");

    let (sender, receiver) = std::sync::mpsc::channel::<log_common::LogData>();

    let mut mock_file = spawn_mocked(log_common::Level::Error, receiver, fp, BufferSize::Size16).unwrap();

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 1"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 2"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Info, ts, format!("test line 3"));
    assert!(sender.send(d).is_ok());

    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Error, ts, format!("test line 4"));
    assert!(sender.send(d).is_ok());

    std::thread::sleep(std::time::Duration::from_millis(500));
    let ts = log_common::get_time_now();
    let d = log_common::LogData::new(log_common::Level::Off, ts, format!("Off line"));
    assert!(sender.send(d).is_ok());

    for _k in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let ts = log_common::get_time_now();
        let d = log_common::LogData::new(log_common::Level::Off, ts, format!("Off line"));
        if sender.send(d).is_err() {
            break;
        }
    }

    mock_file.process_queued_messages();
    let lines = mock_file.get_mock_data();
    
    assert!(5 == lines.len()); // lines plus header
    assert!("\n" == lines[0]);
    assert!( lines[1].find("test line 1").is_some() );
    assert!( lines[2].find("test line 2").is_some() );
    assert!( lines[3].find("test line 3").is_some() );
    assert!( lines[4].find("test line 4").is_some() );
}