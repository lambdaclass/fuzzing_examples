#[macro_use]
extern crate honggfuzz;
use cairo_rs::cairo_run;
use std::io::{BufReader, Read};
use cairo_rs::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::{
    BuiltinHintProcessor, HintFunc,
};
use cairo_rs::hint_processor::hint_processor_definition::HintProcessor;
use cairo_rs::vm::{runners::cairo_runner::CairoRunner, errors::cairo_run_errors::CairoRunError};
use cairo_rs::types::program::Program;
use cairo_rs::vm::vm_core::VirtualMachine;
use cairo_rs::cairo_run::write_output;



fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let reader = BufReader::new(data);
            let mut hint_processor = BuiltinHintProcessor::new_empty();
            cairo_fuzz_run(
                reader,
                "main",
                false,
                "plain",
                false,
                &hint_processor,
            );
        });
    }
}

pub fn cairo_fuzz_run(
    reader: impl Read,
    entrypoint: &str,
    trace_enabled: bool,
    layout: &str,
    proof_mode: bool,
    hint_executor: &dyn HintProcessor,
) -> Result<CairoRunner, CairoRunError> {
    
    let program = match Program::from_reader(reader, Some(entrypoint)) {
        Ok(program) => program,
        Err(error) => return Err(CairoRunError::Program(error)),
    };

    let mut cairo_runner = CairoRunner::new(&program, layout, proof_mode)?;
    let mut vm = VirtualMachine::new(
        program.prime,
        trace_enabled,
        program.error_message_attributes,
    );
    let end = cairo_runner.initialize(&mut vm)?;

    cairo_runner.run_until_pc(end, &mut vm, hint_executor)?;
    cairo_runner.end_run(false, false, &mut vm, hint_executor)?;

    vm.verify_auto_deductions()?;
    if proof_mode {
        cairo_runner.read_return_values(&vm)?;
        cairo_runner.finalize_segments(&mut vm)?;
    }
    cairo_runner.relocate(&mut vm)?;

    Ok(cairo_runner)
}
