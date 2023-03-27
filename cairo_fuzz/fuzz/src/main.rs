#[macro_use]
extern crate honggfuzz;
use cairo_vm::cairo_run;
use cairo_vm::{
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    vm::errors::{
        cairo_run_errors::CairoRunError, runner_errors::RunnerError, trace_errors::TraceError,
    },
};
use std::{
    fs::{remove_file, File},
    io::Write,
    path::PathBuf,
};

struct Args {
    filename: u32,
    trace_file: Option<PathBuf>,
    print_output: bool,
    entrypoint: String,
    #[allow(dead_code)]
    trace: Option<PathBuf>,
    memory_file: Option<PathBuf>,
    layout: String,
    proof_mode: bool,
    secure_run: Option<bool>,
}

fn main() {
    loop {
        fuzz!(|program_bytes: &[u8]| {
            let args = Args {
                filename: rand::random::<u32>(),
                trace_file: None,
                print_output: true,
                entrypoint: "main".to_string(),
                trace: None,
                memory_file: None,
                layout: "all".to_string(),
                proof_mode: false,
                secure_run: None,
            };
            let filename = PathBuf::from(args.filename.to_string());
            let file = File::create(&filename);
            if matches!(file, Err(_)) {
                return;
            }
            if matches!(file.unwrap().write_all(program_bytes), Err(_)) {
                return;
            }
            match cairo_main(&args) {
                Ok(_) => {}
                Err(e) => println!("{e:?}"),
            }
            let _ = remove_file(filename);
        });
    }
}

fn cairo_main(args: &Args) -> Result<(), CairoRunError> {
    let trace_enabled = args.trace_file.is_some();
    let mut hint_executor = BuiltinHintProcessor::new_empty();
    let cairo_run_config = cairo_run::CairoRunConfig {
        entrypoint: &args.entrypoint,
        trace_enabled,
        print_output: args.print_output,
        layout: &args.layout,
        proof_mode: args.proof_mode,
        secure_run: args.secure_run,
    };
    let cairo_runner = match cairo_run::cairo_run(
        &PathBuf::from(args.filename.to_string()),
        &cairo_run_config,
        &mut hint_executor,
    ) {
        Ok(runner) => runner,
        Err(error) => {
            println!("{error}");
            return Err(error);
        }
    };

    if let Some(trace_path) = &args.trace_file {
        let relocated_trace = cairo_runner
            .relocated_trace
            .as_ref()
            .ok_or(CairoRunError::Trace(TraceError::TraceNotEnabled))?;
        match cairo_run::write_binary_trace(relocated_trace, &trace_path) {
            Ok(()) => (),
            Err(_e) => return Err(CairoRunError::Runner(RunnerError::WriteFail)),
        }
    }

    if let Some(memory_path) = &args.memory_file {
        match cairo_run::write_binary_memory(&cairo_runner.relocated_memory, &memory_path) {
            Ok(()) => (),
            Err(_e) => return Err(CairoRunError::Runner(RunnerError::WriteFail)),
        }
    }

    Ok(())
}
