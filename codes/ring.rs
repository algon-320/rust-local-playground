//# ring = "0.16.20"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use ring::aead;
use ring::error::Unspecified;
use std::convert::TryInto;

pub struct NonceSeq {
    next: u128,
}
impl NonceSeq {
    fn new() -> Self {
        Self { next: 0 }
    }
}
impl aead::NonceSequence for NonceSeq {
    fn advance(&mut self) -> Result<aead::Nonce, Unspecified> {
        let value = self.next;
        if value >= 0x0001_0000_0000_0000_0000_0000_0000 {
            Err(Unspecified)
        } else {
            self.next += 1;
            let value_bytes = value.to_ne_bytes();
            Ok(aead::Nonce::try_assume_unique_for_key(&value_bytes[..12]).expect("nonce length"))
        }
    }
}

fn main() {
    println!("Wellcome to the playground!");

    let sealing_key = {
        let ubk = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &[0; 32]).unwrap();
        aead::LessSafeKey::new(ubk)
    };

    let opening_key = {
        let ubk = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &[0; 32]).unwrap();
        aead::LessSafeKey::new(ubk)
    };

    let mut nonce_seq = NonceSeq::new();

    fn seal(
        sealing_key: &aead::LessSafeKey,
        nonce_seq: &mut NonceSeq,
        aad: &[u8],
        data: &[u8],
    ) -> Vec<u8> {
        use aead::NonceSequence;
        let nonce = nonce_seq.advance().expect("nonce wear out");

        // protect integrity of the nonce as well
        let aad = aead::Aad::from([aad.as_ref(), nonce.as_ref()].concat());
        let nonce_bytes: [u8; 12] = *nonce.as_ref();

        let mut bytes = data.to_vec();
        sealing_key
            .seal_in_place_append_tag(nonce, aad, &mut bytes)
            .unwrap();

        // append the nonce at the end of the ciphertext
        bytes.extend_from_slice(&nonce_bytes);

        bytes
    }

    fn unseal(opening_key: &aead::LessSafeKey, aad: &[u8], ciphertext: &mut [u8]) -> Vec<u8> {
        let (ciphertext, nonce_bytes) = ciphertext.split_at_mut(ciphertext.len() - aead::NONCE_LEN);
        let aad = aead::Aad::from([aad.as_ref(), nonce_bytes].concat());
        let nonce =
            aead::Nonce::assume_unique_for_key(nonce_bytes[..].try_into().expect("nonce len"));
        let plaintext = opening_key.open_in_place(nonce, aad, ciphertext).unwrap();
        plaintext.to_vec()
    }

    let mut msg = b"hello,world!".to_vec();
    let mut ciphertext = seal(&sealing_key, &mut nonce_seq, &[], &mut msg);

    // let msg = unseal(&opening_key, &[], &mut ciphertext);
    // assert_eq!(&msg, b"hello,world!");

    let mut msg = b"HELLO,WORLD!".to_vec();
    let mut ciphertext = seal(&sealing_key, &mut nonce_seq, &[], &mut msg);

    let msg = unseal(&opening_key, &[], &mut ciphertext);
    assert_eq!(&msg, b"HELLO,WORLD!");
}
