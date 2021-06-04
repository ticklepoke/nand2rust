use std::{
    fs::File,
    io::{LineWriter, Write},
};

use crate::utils::get_output_handle;

pub struct XMLWriter {
    line_writer: LineWriter<File>,
}

impl XMLWriter {
    pub fn new(filename: &str) -> Self {
        let mut writer = XMLWriter {
            line_writer: get_output_handle(filename),
        };

        write!(writer.line_writer, "<tokens>\n").expect("Unable to initialize file");
        writer
    }

    pub fn write_line(&mut self, tag: &str, value: &str) {
        write!(self.line_writer, "<{0}> {1} </{0}>\n", tag, value).expect("Unable to write line")
    }

    pub fn close(&mut self) {
        write!(self.line_writer, "</tokens>").expect("Unable to terminate file");
        self.line_writer.flush().expect("Unable to close file");
    }
}
