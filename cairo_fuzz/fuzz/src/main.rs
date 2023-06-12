#[macro_use]
extern crate honggfuzz;
use cairo_vm::cairo_run::CairoRunConfig;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::hint_processor::hint_processor_definition::HintProcessor;
use cairo_vm::types::program::Program;
use cairo_vm::vm::runners::cairo_runner::RunResources;
use cairo_vm::vm::vm_core::VirtualMachine;
use cairo_vm::vm::{
    errors::{cairo_run_errors::CairoRunError, vm_exception::VmException},
    runners::cairo_runner::CairoRunner,
    security::verify_secure_runner,
};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut hint_processor = BuiltinHintProcessor::new_empty();
            let cairo_run_config = CairoRunConfig::default();
            let _ = cairo_fuzz_run(data, &cairo_run_config, &mut hint_processor);
        });
    }
}

pub fn cairo_fuzz_run(
    data: &[u8],
    cairo_run_config: &CairoRunConfig,
    hint_executor: &mut dyn HintProcessor,
) -> Result<CairoRunner, CairoRunError> {
    let program = match Program::from_bytes(data, Some(cairo_run_config.entrypoint)) {
        Ok(program) => program,
        Err(error) => return Err(CairoRunError::Program(error)),
    };

    let secure_run = cairo_run_config
        .secure_run
        .unwrap_or(!cairo_run_config.proof_mode);

    let mut cairo_runner = CairoRunner::new(
        &program,
        cairo_run_config.layout,
        cairo_run_config.proof_mode,
    )?;
    let mut vm = VirtualMachine::new(cairo_run_config.trace_enabled);
    let end = cairo_runner.initialize(&mut vm)?;

    let run_resources = RunResources::default();

    cairo_runner
        .run_until_pc(end, &mut Some(run_resources), &mut vm, hint_executor)
        .map_err(|err| VmException::from_vm_error(&cairo_runner, &vm, err))?;
    cairo_runner.end_run(false, false, &mut vm, hint_executor)?;

    vm.verify_auto_deductions()?;
    cairo_runner.read_return_values(&mut vm)?;
    if cairo_run_config.proof_mode {
        cairo_runner.finalize_segments(&mut vm)?;
    }
    if secure_run {
        verify_secure_runner(&cairo_runner, true, None, &mut vm)?;
    }
    cairo_runner.relocate(&mut vm, true)?;

    Ok(cairo_runner)
}
