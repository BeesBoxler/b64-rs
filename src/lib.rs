use ::std::str;

const ALPHABET: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2b, 0x2f,
];

const INDEX: [u8; 123] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3e, 0xff, 0xff, 0xff, 0x3f,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33,
];

pub fn encode(value: &str) -> String {
    // let mut result = vec![];
    let mut data = String::new();
    let mut bytes = value.bytes();

    let len = bytes.len() / 3;
    let rem = bytes.len() % 3;

    for _ in 0..len {
        let chunk: u32 = (bytes.next().unwrap() as u32) << 0x10 | (bytes.next().unwrap() as u32) << 0x08 | bytes.next().unwrap() as u32;

        data.push( ALPHABET[(chunk >> 0x12 as u8 & 0x3f) as usize] as char);
        data.push( ALPHABET[(chunk >> 0x0c as u8 & 0x3f) as usize] as char);
        data.push( ALPHABET[(chunk >> 0x06 as u8 & 0x3f) as usize] as char);
        data.push( ALPHABET[(chunk as u8 & 0x3f) as usize] as char);
    }

    if rem == 1 {
        let chunk: u32 = (bytes.next().unwrap() as u32) << 0x04;

        data.push(ALPHABET[(chunk >> 0x06 as u8 & 0x3f) as usize] as char);
        data.push(ALPHABET[(chunk as u8 & 0x3f) as usize] as char);
        data.push('=');
        data.push('=');


    } else if rem == 2 {

        let chunk = (bytes.next().unwrap() as u32) << 0x0a | (bytes.next().unwrap() as u32) << 0x02;
        
        data.push( ALPHABET[(chunk >> 0x0c as u8 & 0x3f) as usize] as char);
        data.push( ALPHABET[(chunk >> 0x06 as u8 & 0x3f) as usize] as char);
        data.push( ALPHABET[(chunk as u8 & 0x3f) as usize] as char);
        data.push('=');
    }

    data
}

pub fn decode(value: &str) -> String {
    let mut result = vec![];
    let mut data = String::new();
    let bytes = value.bytes();

    for b in bytes {
        if b == 61 {continue}
        data.push_str(&format!("{:0>6b}", INDEX[b as usize] << 2 >> 2));
    }

    while data.len() > 7 {
        result.push(u8::from_str_radix(data.drain(0..8).as_str(), 2).unwrap());
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
#[test]
fn lookup_tables() {
    for i in 0..ALPHABET.len() {
        assert_eq!(INDEX[ALPHABET[i] as usize], i as u8);
    }
}

#[cfg(test)]
mod test {
    use super::{decode, encode};

    struct B64Pair(pub &'static str, pub &'static str);

    fn test_data() -> Vec<B64Pair> {
        vec![
            B64Pair("Test String", "VGVzdCBTdHJpbmc="),
            B64Pair("bea is cool", "YmVhIGlzIGNvb2w="),
            B64Pair("🥺", "8J+lug=="),
            B64Pair("These sentences feel random but they're not, I promise. I am just making sure to test as many different silly things I can!", "VGhlc2Ugc2VudGVuY2VzIGZlZWwgcmFuZG9tIGJ1dCB0aGV5J3JlIG5vdCwgSSBwcm9taXNlLiBJIGFtIGp1c3QgbWFraW5nIHN1cmUgdG8gdGVzdCBhcyBtYW55IGRpZmZlcmVudCBzaWxseSB0aGluZ3MgSSBjYW4h"),
            B64Pair("おはいよう！私の名前はBea!元気ですか？", "44GK44Gv44GE44KI44GG77yB56eB44Gu5ZCN5YmN44GvQmVhIeWFg+awl+OBp+OBmeOBi++8nw=="),
            B64Pair("I need to test padding of each length, you see!!", "SSBuZWVkIHRvIHRlc3QgcGFkZGluZyBvZiBlYWNoIGxlbmd0aCwgeW91IHNlZSEh"),
        ]
    }

    #[test]
    fn encode_string() {
        test_data()
            .iter()
            .for_each(|B64Pair(i, o)| assert_eq!(encode(i), *o));
    }

    #[test]
    fn decode_string() {
        test_data()
            .iter()
            .for_each(|B64Pair(i, o)| assert_eq!(decode(o), *i));
    }
}
