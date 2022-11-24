use honggfuzz::fuzz;
use simple_asn1::*;
use asn1::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let result_asn1: asn1::ParseResult<_> = asn1::parse(data, |d| {
                return d.read_element::<asn1::Sequence>()?.parse(|d| {
                    let r = d.read_element::<u64>()?;
                    let s = d.read_element::<u64>()?;
                    return Ok((r, s));
                })
            });
            match result_asn1 {
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
            }
        });
    }
}