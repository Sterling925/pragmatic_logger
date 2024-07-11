#![cfg(test)]

use super::*;

pub mod mock_writer;


#[test]
fn test_init() {
    const BUFFER_SIZE: usize = 0x10;

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }
    // input_buffer : std::vec::Vec::<String>, writer: data_writer::DataWriter
    let d = data_writer::DataWriter::new();

    let mut cb = CircularStringsBuffer::new(b, d);
    cb.push(String::from("1"));
    assert!(cb.get_qty_in_buffer() == 1);
    cb.push(String::from("2"));
    assert!(cb.get_qty_in_buffer() == 2);

    assert!(cb.get(0) == Some(1.to_string()));
    assert!(cb.get(1) == Some(2.to_string()));
    assert!(cb.get(2) == None);
}

#[should_panic]
#[test]
fn test_invalid() {
    const BUFFER_SIZE: usize = 10; // 0x0A is not a valid buffer size

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }

    // Will panic if Debug build
    let d = data_writer::DataWriter::new();

    let _cb = CircularStringsBuffer::new(b, d);
}

#[test]
fn test_wrap() {
    const BUFFER_SIZE: usize = 0x10;

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }
    let d = data_writer::DataWriter::new();

    let mut cb = CircularStringsBuffer::new(b, d);

    for k in 0..BUFFER_SIZE + 4 {
        cb.push(format!("{}", k));
    }

    assert!(cb.get_qty_in_buffer() == BUFFER_SIZE);

    for k in 4..BUFFER_SIZE + 4 {
        //cb.push(format!("{}",k));
        assert!(cb.get(k) == Some(format!("{}", k)));
    }

    assert!(cb.get_min_external_index_in_buffer() == 4);
}

#[test]
fn test_reset() {
    const BUFFER_SIZE: usize = 0x10;

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }

    let d = data_writer::DataWriter::new();

    let mut cb = CircularStringsBuffer::new(b, d);

    for k in 0..BUFFER_SIZE + 5 {
        cb.push(format!("{}", k));
    }

    assert!(cb.get_qty_in_buffer() == BUFFER_SIZE);

    for k in 5..BUFFER_SIZE + 5 {
        assert!(cb.get(k) == Some(format!("{}", k)));
    }

    assert!(cb.get_min_external_index_in_buffer() == 5);

    cb.reset();
    assert!(cb.get_min_external_index_in_buffer() == 0);
    assert!(cb.get_qty_in_buffer() == 0);
}

#[test]
fn test_reset_fast() {
    const BUFFER_SIZE: usize = 0x10;

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }

    let (mut mock_file, mock_writer) = mock_writer::get_mock_text_data_writer();

    let mut cb = CircularStringsBuffer::new(b, mock_writer);

    for k in 0..BUFFER_SIZE + 8 {
        cb.push(format!("{}", k));
    }

    assert!(cb.get_qty_in_buffer() == BUFFER_SIZE);

    for k in 8..BUFFER_SIZE + 8 {
        assert!(cb.get(k) == Some(format!("{}", k)));
    }

    assert!(cb.get_min_external_index_in_buffer() == 8);

    cb.reset_fast();
    assert!(cb.get_min_external_index_in_buffer() == 0);
    assert!(cb.get_qty_in_buffer() == 0);

    mock_file.process_queued_messages();
    let f = mock_file.get_mock_data();
    assert!(f.len() == 0);
}

#[test]
fn test_more_overflow() {
    const BUFFER_SIZE: usize = 0x10;

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }

    let d = data_writer::DataWriter::new();

    let mut cb = CircularStringsBuffer::new(b, d);

    for k in 0..BUFFER_SIZE * 1000 {
        cb.push(format!("{}", k));
    }

    assert!(cb.get_qty_in_buffer() == BUFFER_SIZE);

    for k in ((BUFFER_SIZE * 1000) - BUFFER_SIZE)..1000 * BUFFER_SIZE {
        assert!(cb.get(k) == Some(format!("{}", k)));
    }
    assert!(cb.get(((BUFFER_SIZE * 1000) - BUFFER_SIZE) - 1) == None);
    assert!(cb.get(BUFFER_SIZE * 1000) == None);

    assert!(cb.does_index_exist(((BUFFER_SIZE * 1000) - BUFFER_SIZE) - 1) == false);
    assert!(cb.does_index_exist(BUFFER_SIZE * 1000) == false);

    assert!(cb.does_index_exist((BUFFER_SIZE * 1000) - BUFFER_SIZE) == true);
    assert!(cb.does_index_exist((BUFFER_SIZE * 1000) - 1) == true);

    assert!(cb.get_min_external_index_in_buffer() == ((BUFFER_SIZE * 1000) - BUFFER_SIZE));

    cb.reset();
    assert!(cb.get_min_external_index_in_buffer() == 0);
    assert!(cb.get_qty_in_buffer() == 0);
}


#[test]
fn test_wrap_and_write() {
    const BUFFER_SIZE: usize = 0x10;
    // "TEST_DIRECTORY" assumed to be defined in [env] of `.cargo/config.toml`
    let rw_dir = std::env::var("TEST_DIRECTORY").unwrap();
    let out_path = std::path::PathBuf::from(rw_dir);

    let mut b: std::vec::Vec<String> = std::vec::Vec::<String>::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        b.push(String::new());
    }

    let (mut mock_file, mock_writer) = mock_writer::get_mock_text_data_writer();

    let mut cb = CircularStringsBuffer::new(b, mock_writer);

    for k in 0..BUFFER_SIZE + 4 { // Wrap arround is +4 past size of buffer
        cb.push(format!("{}", k));
    }

    assert!(cb.get_qty_in_buffer() == BUFFER_SIZE);

    // First 4 values were over written by wrapped data. Total size is still BUFFER_SIZE
    for k in 4..BUFFER_SIZE + 4 {
        assert!(cb.get(k) == Some(format!("{}", k)));
    }

    assert!(cb.get_min_external_index_in_buffer() == 4);

    assert!( cb.write_to_file_and_clear(&out_path).is_ok());

    mock_file.process_queued_messages();

    let mock_file_data = mock_file.get_mock_data();

    const HEADER_SIZE : usize = 1; // Header written at start of each data dump
    for k in 4..BUFFER_SIZE + 4 {
        let v  = format!("{}", k);
        assert!(*mock_file_data[k - 4 + HEADER_SIZE] == v);
    }

    assert!(BUFFER_SIZE + HEADER_SIZE == mock_file_data.len());
}