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
        let seed: [u8; 4] = [
            0xff, 0xff, 0xff, 0xff
        ];
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

        for n in data.iter() {
            print!("{n:x} ");
        }
        println!("");
        let result_asn1: asn1::ParseResult<_> = asn1::parse(&data, |d| {
            return d.read_element::<asn1::Sequence>()?.parse(|d| {
                let r = d.read_element::<i64>()?;
                let s = d.read_element::<i64>()?;
                return Ok((r, s));
            })
        });
        dbg!(&result_asn1);
        assert!(matches!(result_asn1, Ok(_)));
    }
}
