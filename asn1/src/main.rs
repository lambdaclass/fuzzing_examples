use honggfuzz::fuzz;
//use simple_asn1::*;

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
