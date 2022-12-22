use rlp::{Encodable, RlpStream};
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: u128,
    pub gas_price: u128,
    pub gas: u128,
    pub to: Option<[u8; 20]>,
    pub value: u64,
    pub data: Vec<u8>,
    pub chain_id: u64,
}

impl Transaction {
    pub fn sign(&self, private_key: &[u8]) -> Vec<u8> {
        let hashed_tx = self.hash();

        let sign_only = Secp256k1::signing_only();
        let message = Message::from_slice(&hashed_tx).unwrap();
        let secret_key = SecretKey::from_slice(&private_key).expect("Wrong Private Key");
        let (v, signature) = sign_only
            .sign_ecdsa_recoverable(&message, &secret_key)
            .serialize_compact();

        let v = v.to_i32() as u64 + (self.chain_id * 2 + 35);
        let r = signature[0..32].to_vec();
        let s = signature[32..64].to_vec();

        let mut stream = RlpStream::new();
        self.rlp_append(&mut stream);
        stream.append(&v);
        stream.append(&r);
        stream.append(&s);
        stream.finalize_unbounded_list();

        stream.out().to_vec()
    }

    pub fn hash(&self) -> Vec<u8> {
        // Rlp encode transaction
        let mut stream = RlpStream::new();
        self.rlp_append(&mut stream);

        // Add params for legacy transaction
        stream.append(&self.chain_id);
        stream.append_raw(&[0x80], 1);
        stream.append_raw(&[0x80], 1);
        stream.finalize_unbounded_list();
        let rlp_bytes = stream.out().to_vec();

        // Hash rlp_bytes
        let mut hasher = Keccak256::new();
        hasher.update(rlp_bytes);
        // Return hashed transaction to be signed
        hasher.finalize().to_vec()
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_unbounded_list();
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let None = self.to {
            s.append(&Vec::new());
        } else {
            s.append(&self.to.unwrap().to_vec());
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            nonce: 0,
            gas_price: 250,
            gas: 21000,
            to: None,
            value: 0,
            data: Vec::new(),
            chain_id: 1,
        }
    }
}
