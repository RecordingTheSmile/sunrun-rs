use aes::cipher::{BlockEncryptMut, KeyIvInit};
use aes::cipher::block_padding::Pkcs7;
use chrono::Timelike;
use rand::Rng;

pub(crate) struct SunrunCrypto;

const KEY: &[u8] = "osldaaasmkldospd".as_bytes();
const IV: &[u8] = "0392030003920392".as_bytes();

impl SunrunCrypto {
    pub fn get_encrypt_key() -> Vec<char> {
        let mut seed = "abcdefghijklmnopqrstuvwxyz".as_bytes().to_vec();
        let mut result: Vec<char> = vec![];

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let position = rng.gen_range(0..seed.len());
            let passwd: char = seed[position] as char;
            result.push(passwd);
            seed.remove(position);
        }

        result
    }

    pub fn encrypt_integer(key: &Vec<char>, num: i64) -> String {
        let mut result = vec![];

        let base_num = '0' as usize;

        for position in num.to_string().as_bytes().iter().map(|x| *x as usize) {
            result.push(key[position - base_num]);
        }
        result.iter().map(|x| x.to_string()).collect()
    }

    pub fn user_info_encrypt() -> String {
        let time_now = chrono::Local::now().with_timezone(&chrono_tz::PRC);
        let minute = time_now.minute();
        let second = time_now.second();

        let minute = if minute < 10 { format!("0{minute}") } else { minute.to_string() };
        let second = if second < 10 { format!("0{second}") } else { second.to_string() };

        let encrypt = cbc::Encryptor::<aes::Aes128>::new(KEY.into(), IV.into());

        let result = encrypt.encrypt_padded_vec_mut::<Pkcs7>(format!("{minute}{second}").as_bytes());

        let result = base64::encode(result);
        let mut ret = String::from("A");
        ret.push_str(&result);
        ret
    }
}