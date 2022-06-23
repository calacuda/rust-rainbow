use md5;
use ntlm_hash::*;
use sha1;
use sha2::{Digest, Sha256, Sha512};

pub fn hash(func: &str, passwd: &Vec<u8>) -> String {
    return match func {
        "md5" => hash_md5(passwd),
        "sha1" => hash_sha1(passwd),
        "sha256" => hash_sha256(passwd),
        "sha512" => hash_sha512(passwd),
        "ntlm_v2" => hash_ntlm(passwd), // can only handle passwords shorter then 32 chars
        _ => panic!("how'd that get here, that hash funciton isn't implemented yet?"),
    };
}

fn hash_md5(passwd: &Vec<u8>) -> String {
    return format!("{:x}", md5::compute(passwd));
}

fn hash_sha1(passwd: &Vec<u8>) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(passwd);
    return format!("{:x}", hasher.finalize());
}

fn hash_sha256(passwd: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(passwd);
    return format!("{:x}", hasher.finalize());
}

fn hash_sha512(passwd: &Vec<u8>) -> String {
    let mut hasher = Sha512::new();
    hasher.update(passwd);
    return format!("{:x}", hasher.finalize());
}

fn hash_ntlm(passwd: &Vec<u8>) -> String {
    return if passwd.len() < 32 {
        ntlm_hash(&String::from_utf8(passwd.to_owned()).unwrap())
    } else {
        "N/A".to_string()
    };
}
