#[macro_use]
extern crate honggfuzz;
use cairo_vm::{
    cairo_run::{CairoRunConfig, cairo_run, write_encoded_trace},
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    vm::errors::{cairo_run_errors::CairoRunError, trace_errors::TraceError}
};

use std::{fs::{read, remove_file, File}, io::{Write, BufWriter}};
use bincode::enc::write::Writer;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let trace_name_root = rand::random::<u64>().to_string();
            let py_trace_name = trace_name_root.clone() + ".py.trace";
            let rs_trace_name = trace_name_root + ".rs.trace";
            let program_name = rand::random::<u64>().to_string();

            match (execute_program(&rs_trace_name, &data), cairo_python(&program_name, &py_trace_name, &data)) {
                (Ok(_), Ok(_)) => {
                    match (read(&py_trace_name), read(&rs_trace_name)) {
                        (Ok(py_trace), Ok(rs_trace)) => assert_eq!(py_trace, rs_trace),
                        _ => {}
                    }
                }
                (Err(e), Ok(_)) if !e.to_string().contains("Unknown Hint")  => panic!("Err in cairo-rs"),
                _ => {}
            }

            let _ = remove_file(program_name);
            let _ = remove_file(rs_trace_name);
            let _ = remove_file(py_trace_name);
        });
    }
}

fn cairo_python(filename: &String, trace_path: &String, program_bytes: &[u8]) -> std::io::Result<()> {
    let mut program = File::create(&filename)?;
    program.write(program_bytes)?;

    std::process::Command::new("cairo-run")
        .args([
              "--program", &filename,
              "--layout", "all",
              "--trace_file", trace_path
        ])
        .output()?;
    Ok(())
}

fn execute_program(trace_path: &String, program_content: &[u8]) -> Result<(), CairoRunError> {
    let mut hint_executor = BuiltinHintProcessor::new_empty();
    let cairo_run_config = CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        layout: "all",
        proof_mode: false,
        secure_run: None,
    };

    let (cairo_runner, _) = cairo_run(&program_content, &cairo_run_config, &mut hint_executor)?;

    let relocated_trace = cairo_runner
        .relocated_trace
        .ok_or(CairoRunError::Trace(TraceError::TraceNotEnabled))?;

    let trace_file = File::create(trace_path).map_err(|_| CairoRunError::Trace(TraceError::TraceNotEnabled))?;
    let mut trace_writer = 
        FileWriter::new(BufWriter::with_capacity(3 * 1024 * 1024, trace_file));

    write_encoded_trace(&relocated_trace, &mut trace_writer).map_err(|_| CairoRunError::Trace(TraceError::TraceNotEnabled))?;
    trace_writer.flush().map_err(|_| CairoRunError::Trace(TraceError::TraceNotEnabled))?;

    Ok(())
}

struct FileWriter {
    buf_writer: BufWriter<File>,
    bytes_written: usize,
}

impl Writer for FileWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
        self.buf_writer
            .write_all(bytes)
            .map_err(|e| bincode::error::EncodeError::Io {
                inner: e,
                index: self.bytes_written,
            })?;

        self.bytes_written += bytes.len();

        Ok(())
    }
}

impl FileWriter {
    fn new(buf_writer: BufWriter<File>) -> Self {
        Self {
            buf_writer,
            bytes_written: 0,
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf_writer.flush()
    }
}
