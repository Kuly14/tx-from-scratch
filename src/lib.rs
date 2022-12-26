use rlp::{Encodable, RlpStream};
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    /// Nonce of your next transaction
    pub nonce: u128,

    /// Gas price
    pub gas_price: u128,

    /// Gas or Gas_limit. So amount of gas you are willing to spend
    pub gas: u128,

    /// Address you want to transact with. If you want to deploy a contract, `to` should be None.
    ///
    /// To convert your address from string to [u8; 20] you will have to use ethereum_types crate.
    /// ```no_run
    /// use ethereum_types::H160;
    /// use std::str::FromStr;
    ///
    /// let address: [u8; 20] = H160::from_str(&"/* your address */").unwrap().to_fixed_bytes();
    /// ```
    pub to: Option<[u8; 20]>,

    /// Amount of ether you want to send
    pub value: u128,

    /// If you want to interact or deploy smart contract add the bytecode here
    pub data: Vec<u8>,

    /// Chain id for the target chain. Mainnet = 1
    pub chain_id: u64,
}

impl Transaction {
    /// To use sign method you have to input your private key so it can sign the transaction.
    /// It takes `&[u8]` as parameter. If you want to convert your private_key from string here is
    /// how you can do that
    /// ```no_run
    /// use ethereum_types::H256;
    /// use std::str::FromStr;
    ///
    /// let private_key = H256::from_str("/*your private key*/").unwrap();
    ///
    /// let tx = Transaction::default();
    ///
    /// let signed_tx = tx.sign(private_key.as_bytes());
    /// ```
    /// This will convert your private key to &[u8] from string
    pub fn sign(&self, private_key: &[u8]) -> Vec<u8> {
        let hashed_tx = self.hash();

        let sign_only = Secp256k1::signing_only();
        let message = Message::from_slice(&hashed_tx).unwrap();
        let secret_key = SecretKey::from_slice(private_key).expect("Wrong Private Key");
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

    /// This method first RLP encodes the transaction and then hashes it with keccak256.
    /// It will return a hashed transaction that has to be signed
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
    /// Implement Encodable trait for simpler Rlp Encoding
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_unbounded_list();
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if self.to.is_none() {
            s.append(&Vec::new());
        } else {
            s.append(&self.to.unwrap().to_vec());
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

impl Default for Transaction {
    /// Implement Default trait so users can specify what what they need and rest will be added
    /// automatically.
    /// ```no_run
    /// use ethereum_types::H160;
    /// use std::str::FromStr;
    ///
    /// let address: [u8; 20] = H160::from_str(&"/* your address */").unwrap().to_fixed_bytes();
    ///
    /// let tx = tx_from_scratch::Transaction {
    ///     nonce: 10,
    ///     to,
    ///     value: 10,
    ///     ..Default::default(),
    /// }
    /// ```
    /// If you don't specify `to` the default is None so it will try to deploy a contract
    ///
    /// Default is:
    /// ```no_run
    /// Transaction {
    ///     nonce: 0,
    ///     gas_price: 250,
    ///     gas: 21000,
    ///     to: None,
    ///     value: 0,
    ///     data: Vec::new(),
    ///     chain_id: 1,
    /// }
    /// ```
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
