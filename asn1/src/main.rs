use honggfuzz::fuzz;
//use simple_asn1::*;

fn main() {
    loop {
        fuzz!(|seed: &[u8]| {
            let data: Vec<u8> = if seed.len() == 1 || seed.len() > 16 {
                vec![ 0x30, 0x06, 0x02, 0x01, seed[0], 0x02, 0x01, seed[0] ]
            } else {
                let mut sequence_decl = vec![ 0x30, (seed.len() + 4) as u8, 0x02, (seed.len() / 2) as u8 ];
                sequence_decl.extend(&seed[..seed.len() / 2]);
                sequence_decl.extend(vec![0x02, (seed.len() - seed.len() / 2) as u8 ]);
                sequence_decl.extend(&seed[seed.len() / 2..]);
                sequence_decl   
            };

            let result_asn1: asn1::ParseResult<_> = asn1::parse(&data, |d| {
                return d.read_element::<asn1::Sequence>()?.parse(|d| {
                    let r = d.read_element::<u64>()?;
                    let s = d.read_element::<u64>()?;
                    return Ok((r, s));
                })
            });
            result_asn1.unwrap();
            /*match result_asn1 {
                Ok(sk) => { 
                    // ... use sk ...
                    let (ui1, ui2) = sk;
                    let vec: Vec<u8> = Vec::new();
                    let asn1block = ASN1Block::BitString(ui1 as usize, ui2 as usize, vec);
                    let _result_simple_asn1 = simple_asn1::to_der(&asn1block);
                },
                Err(e) => {
                    // ... sk is not available, and e explains why ...
                    panic!(
                        "{}", e
                    )
                },
            }*/
        });
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn try_crash() {
        let seed: [u8; 1] = [
            0xd7
        ];
        let data: Vec<u8> = if seed.len() == 1 || seed.len() > 16 {
            vec![ 0x30, 0x06, 0x02, 0x01, seed[0], 0x02, 0x01, seed[0] ]
        } else {
            let mut sequence_decl = vec![ 0x30, (seed.len() + 4) as u8, 0x02, (seed.len() / 2) as u8 ];
            sequence_decl.extend(&seed[..seed.len() / 2]);
            sequence_decl.extend(vec![0x02, (seed.len() - seed.len() / 2) as u8 ]);
            sequence_decl.extend(&seed[seed.len() / 2..]);
            sequence_decl   
        };

        for n in data.iter() {
            print!("{n:x} ");
        }
        println!("");
        let result_asn1: asn1::ParseResult<_> = asn1::parse(&data, |d| {
            return d.read_element::<asn1::Sequence>()?.parse(|d| {
                let r = d.read_element::<u64>()?;
                let s = d.read_element::<u64>()?;
                return Ok((r, s));
            })
        });
        assert!(matches!(result_asn1, Ok(_)));
    }
}
