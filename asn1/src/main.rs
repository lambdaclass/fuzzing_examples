use honggfuzz::fuzz;

// This fuzzer tests the following DER strcuture:
// SEQUENCE(
//     INTEGER (signed, 64 bits)
//     INTEGER (signed, 64 bits)
// )
// In DER this is encoded as such:
// 0x30 | 0xSS | 0x02 | 0xI1 | ... | 0x02 | 0xI2 | ... 
// 0x30 is the Sequence tag, this means the following bytes represent a sequence.
// The following byte 0xSS tells us how many bytes we should read: for example if we had a 0x03,
// that means the next 3 bytes belong to the Sequence.
//
// 0x02 is the Integer tag, also followed by the size in bytes of the integer 0xI1 and 0xI2. The
// bytes are written in big endian format.
//
// Something else to keep in mind is that -1 and 0 can only be represented as a single 0xFF and 0x00 respectively.
//
// To recap with an example:
// SEQUENCE (4, 1233)
// encoding: 0x30 0x07 | 0x02 0x01 0x04 | 0x02 0x02 0x04 0xD1

fn main() {
    loop {
        fuzz!(|seed: &[u8]| {
            let data: Vec<u8> = if seed.len() == 1 || seed.len() > 16 {
                vec![ 0x30, 0x06, 0x02, 0x01, seed[0], 0x02, 0x01, seed[0] ]
            } else {
                let mut first_int: Vec<u8> = Vec::from(&seed[..seed.len() / 2]);
                first_int = first_int.into_iter().skip_while(|&n| n == 0 || n == 0xff).collect();
                if first_int.is_empty() {
                    first_int = vec![0];
                }

                let mut second_int = Vec::from(&seed[seed.len() / 2..]);
                second_int = second_int.into_iter().skip_while(|&n| n == 0 || n == 0xff).collect();
                if second_int.is_empty() {
                    second_int = vec![0];
                }

                let mut sequence_decl = vec![ 0x30, (first_int.len() + second_int.len() + 4) as u8, 0x02, first_int.len() as u8 ];
                sequence_decl.extend(&first_int);
                sequence_decl.extend(vec![0x02, second_int.len() as u8 ]);
                sequence_decl.extend(&second_int);
                sequence_decl   
            };

            let result_asn1: asn1::ParseResult<_> = asn1::parse(&data, |d| {
                return d.read_element::<asn1::Sequence>()?.parse(|d| {
                    let r = d.read_element::<i64>()?;
                    let s = d.read_element::<i64>()?;
                    return Ok((r, s));
                })
            });
            result_asn1.unwrap();
        });
    }
}
