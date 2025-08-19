use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

pub struct Aes128Cfb8 {
    cipher: Aes128,
    shift_register: [u8; 16],
}

impl Aes128Cfb8 {
    pub fn new(key: &[u8; 16], iv: &[u8; 16]) -> Self {
        Self {
            cipher: Aes128::new(GenericArray::from_slice(key)),
            shift_register: *iv,
        }
    }

    pub fn encrypt_byte(&mut self, byte: u8) -> u8 {
        let mut block = GenericArray::clone_from_slice(&self.shift_register);
        self.cipher.encrypt_block(&mut block);

        let cipher_byte = byte ^ block[0];

        self.shift_register.rotate_left(1);
        self.shift_register[15] = cipher_byte;

        cipher_byte
    }

    pub fn decrypt_byte(&mut self, byte: u8) -> u8 {
        let mut block = GenericArray::clone_from_slice(&self.shift_register);
        self.cipher.encrypt_block(&mut block);

        let plain_byte = byte ^ block[0];

        self.shift_register.rotate_left(1);
        self.shift_register[15] = byte;

        plain_byte
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = self.encrypt_byte(*byte);
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = self.decrypt_byte(*byte);
        }
    }
}