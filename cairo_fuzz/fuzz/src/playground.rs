/*use cairo_vm::{
    cairo_run::{cairo_run, CairoRunConfig},
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
};*/
use cairo_vm::serde::deserialize_program::{DebugInfo, Attribute, Identifier, HintParams, ReferenceManager};
use cairo_felt::Felt252;
//use honggfuzz::fuzz;
use serde::{Serialize, Deserialize, Serializer, ser::{SerializeSeq, SerializeMap}};
use arbitrary::{self, Unstructured, Arbitrary};
use std::collections::HashMap;

const BUILTIN_NAMES: [&str; 9] = [
    "output", 
    "range_check", 
    "pedersen", 
    "ecdsa", 
    "keccak", 
    "bitwise", 
    "ec_op", 
    "poseidon", 
    "segment_arena"
];

#[derive(Arbitrary, Serialize, Deserialize)]
struct ProgramJson {
    attributes: Vec::<Attribute>,
    #[arbitrary(with = arbitrary_builtins)]
    builtins: Vec::<String>,
    #[arbitrary(value = "0.11.0".to_string())]
    compiler_version: String,
    #[arbitrary(with = arbitrary_data)] 
    #[serde(serialize_with = "hex_notation")]
    data: Vec<Felt252>,
    debug_info: DebugInfo,
    #[arbitrary(with = prepend_main_identifier)] 
    identifiers: HashMap<String, Identifier>,
    hints: HashMap<usize, Vec<HintParams>>,
    #[arbitrary(value = "__main__".to_string())]
    main_scope: String,
    #[arbitrary(value = "0x800000000000011000000000000000000000000000000000000000000000001".to_string())]
    prime: String,
    reference_manager: ReferenceManager
}

fn arbitrary_builtins(u: &mut Unstructured) -> arbitrary::Result<Vec<String>> {
    let builtin_total = u.choose_index(BUILTIN_NAMES.len())?;
    let mut selected_builtins = Vec::new();

    for i in 0..=builtin_total {
        if u.ratio(2, 3)? {
            selected_builtins.push(BUILTIN_NAMES[i].to_string())
        }
    }

    Ok(selected_builtins)
}

fn arbitrary_data(u: &mut Unstructured) -> arbitrary::Result<Vec<Felt252>> {
    let data_size = u.arbitrary_len::<Felt252>()?;
    let mut data = Vec::with_capacity(data_size);

    for _ in 0..data_size{
        data.push(Felt252::arbitrary(u)?);
    }

    Ok(data)
}

fn hex_notation<S>(data: &Vec::<Felt252>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let mut seq = serializer.serialize_seq(Some(data.len()))?;
    for element in data {
        let mut number = String::from("0x");
        number.push_str(&element.to_str_radix(16));
        seq.serialize_element(&number)?;
    }
    seq.end()
}

fn prepend_main_identifier(u: &mut Unstructured) -> arbitrary::Result<HashMap<String, Identifier>> {
    let mut identifiers = HashMap::<String, Identifier>::arbitrary(u)?;
    identifiers.insert(
        String::from("__main__.main"),
        Identifier {
            pc: Some(0),
            type_: Some(String::from("function")),
            value: None,
            full_name: None, 
            members: None,
            cairo_type: None
        }
    );
    Ok(identifiers)
}

fn only_print_somes<S>(identifiers: &HashMap::<String, Identifier>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let mut map = serializer.serialize_map(Some(identifiers.len()))?;
    for (k, v) in identifiers {
        // Figure out how to avoid printing the nil fields of Identifiers
        map.serialize_entry(k, v.to_string());

    }
    map.end()
}

fn main() {/*
    loop {
        fuzz!(|data: (CairoRunConfig, &[u8])| {
            let (cairo_run_config, program_json) = data;
            let _ = cairo_run(
                program_json,
                &CairoRunConfig::default(),
                &mut BuiltinHintProcessor::new_empty(),
            );
            let _ = cairo_run(
                program_json,
                &cairo_run_config,
                &mut BuiltinHintProcessor::new_empty(),
            );
        });
    }*/
    
    //let mut u = Unstructured::new(&[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,5,6,7,8,9,0,11,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9]);
    let mut u = Unstructured::new(include_bytes!("../../cairo_programs/example_blake2s.cairo"));
    let program_json = ProgramJson::arbitrary(&mut u).unwrap();
    let serialized = serde_json::to_string_pretty(&program_json).unwrap();
    println!("{serialized}");
}

