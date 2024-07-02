pub mod data_writer;

#[cfg(test)]
mod circular_buffer_tests;
#[cfg(test)]
mod log_reciever_tests;

pub use data_writer::TextDataWriter;

pub struct CircularStringsBuffer<T: TextDataWriter + Send > {
    buffer: std::vec::Vec<String>,
    max_size: usize,
    internal_index: usize,
    external_index: usize,
    writer: T,
}

impl<T: TextDataWriter + Send > CircularStringsBuffer<T>{
    pub fn new(input_buffer: std::vec::Vec<String>, writer: T) -> Self {
        debug_assert!(1 == input_buffer.len().count_ones());
        debug_assert!(0x03 < input_buffer.len());
        debug_assert!(0x80000000 > input_buffer.len());

        Self {
            max_size: input_buffer.len(),
            buffer: input_buffer,
            internal_index: 0,
            external_index: 0,
            writer,
        }
    }

    fn reset_fast(&mut self) {
        self.internal_index = 0;
        self.external_index = 0;
    }

    fn reset(&mut self) {
        self.reset_fast();
        for k in self.buffer.iter_mut() {
            (*k).clear();
        }
    }

    pub fn push(&mut self, new_value: String) {
        if self.internal_index >= self.max_size {
            self.internal_index = 0;
        }
        self.buffer[self.internal_index] = new_value;
        self.internal_index += 1;
        self.external_index += 1;
    }

    fn get(&self, external_target_index: usize) -> Option<String> {
        if self.does_index_exist(external_target_index) {
            let index = self.calc_ring_index(external_target_index);
            debug_assert!(index < self.max_size);
            Some(self.buffer[index].clone())
        } else {
            None
        }
    }

    pub fn write_to_file_and_clear(&mut self, f: &std::path::Path) -> Result<(), &'static str> {
        const DUMP_HEADER: &str = "\n"; // Just insert new line between data dumps
        let qty = self.get_qty_in_buffer();
        let mut ans: Result<(), &'static str> = Ok(());
        if qty > 0 {
            let start_index = self.get_min_external_index_in_buffer();
            let end_index = self.external_index;
            debug_assert!(end_index > start_index);
            let success = self.writer.open(f);

            if success.is_err() {
                ans = Err("File open failed in write_to_file_and_clear()");
            } else if self.writer.write(DUMP_HEADER).is_err() {
                ans = Err("write_to_file_and_clear() failed to write header");
            } else {
                for k in start_index..end_index {
                    if let Some(line) = self.get(k) {
                        if self.writer.write(&line).is_err() {
                            ans = Err("write_to_file_and_clear() failed to write data");
                            break;
                        }
                    } else {
                        debug_assert!(false);
                        self.reset_fast(); // May lose data, but will get buffer back to a working state if indexes are broken.
                        ans = Err("In write_to_file_and_clear(), index into buffer was not valid");
                        break;
                    }
                }
            }
            self.writer.close();
        }
        if qty < 10 {
            // Dont bother with a full reset if small batches are being written.
            self.reset_fast();
        } else {
            // Else do a full reset and clear of all strings in buffer.
            self.reset();
        }

        ans
    }

    fn calc_ring_index(&self, external_index: usize) -> usize // Internal index
    {
        external_index & (self.max_size - 1) // max_size must be 1 more than all 1s ex. 0x800
    }

    fn get_min_external_index_in_buffer(&self) -> usize // External index
    {
        if self.max_size > self.external_index {
            0
        } else {
            self.external_index - self.max_size
        }
    }

    fn get_qty_in_buffer(&self) -> usize {
        if self.max_size < self.external_index {
            self.max_size
        } else {
            self.external_index
        }
    }

    fn does_index_exist(&self, target_external_index: usize) -> bool // External Index
    {
        (target_external_index < self.external_index)
            && ((self.external_index - target_external_index) <= self.max_size)
    }

}
