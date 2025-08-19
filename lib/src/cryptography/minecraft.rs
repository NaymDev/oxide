use super::cfb8::Aes128Cfb8;

pub struct McCrypto {
    pub encrypt: Aes128Cfb8,
    pub decrypt: Aes128Cfb8,
}

impl McCrypto {
    pub fn new(shared_secret: &[u8; 16]) -> Self {
        let encrypt = Aes128Cfb8::new(shared_secret, shared_secret);
        let decrypt = Aes128Cfb8::new(shared_secret, shared_secret);
        Self { encrypt, decrypt }
    }

    pub fn encrypt_packet(mut self, data: &mut [u8]) {
        self.encrypt.encrypt(data);
    }

    pub fn decrypt_packet(&mut self, data: &mut [u8]) {
        self.decrypt.decrypt(data);
    }
}
