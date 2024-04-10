//从第b行第c个数向前读a位
pub fn read_hex(matrix: &[Vec<String>], a: i8, mut b: usize, mut c: usize) -> String {
    let mut s = String::new();
    for _ in 0..a {
        s.push_str(matrix[b][c].as_str());
        if c == 0 {
            c += 15;
            b -= 1;
        } else {
            c -= 1;
        }
    }
    s
}

//16进制加法
pub fn add_hex(a: &str, b: &str) -> String {
    let a_10 = u32::from_str_radix(a, 16).unwrap(); //字符串转10进制数
    let b_10 = u32::from_str_radix(b, 16).unwrap();
    let sum = a_10 + b_10;
    let mut sum_hex_string = format!("{:X}", sum);
    while sum_hex_string.len() < 8 {
        sum_hex_string = format!("0{}", sum_hex_string);
    }

    sum_hex_string
}

pub fn add_hex_cycle(a: &mut str, b: &str) -> String {
    let a_10 = u32::from_str_radix(a, 16).unwrap(); //字符串转10进制数
    let b_10 = u32::from_str_radix(b, 16).unwrap();
    let sum = a_10 + b_10;
    let mut sum_hex_string = format!("{:X}", sum); //输出16进制字符串
    while sum_hex_string.len() < 8 {
        sum_hex_string = format!("0{}", sum_hex_string);
    }

    sum_hex_string
}

//16进制减法
pub fn sub_hex(a: &str, b: &str) -> String {
    let a_10 = u32::from_str_radix(a, 16).unwrap(); //字符串转10进制数
    let b_10 = u32::from_str_radix(b, 16).unwrap();
    let sum = a_10 - b_10;
    let mut sub_hex_string = format!("{:X}", sum);
    while sub_hex_string.len() < 8 {
        sub_hex_string = format!("0{}", sub_hex_string);
    }

    sub_hex_string
}

//分隔16进制数
pub fn position1(a: &str) -> usize {
    let a_7 = a[0..7].to_string();
    let num = usize::from_str_radix(a_7.as_str(), 16).unwrap();
    num
}
pub fn position2(a: &str) -> usize {
    let a_7 = a[7..8].to_string();
    let num = usize::from_str_radix(a_7.as_str(), 16).unwrap();
    num
}

//16进制字符串比较
pub fn hex_compare1(a: &str, b: &str) -> bool {
    let a_10 = u32::from_str_radix(a, 16).unwrap(); //字符串转10进制数
    let b_10 = u32::from_str_radix(b, 16).unwrap();
    a_10 >= b_10
}

pub fn hex_compare2(a: &str, b: &str) -> bool {
    let a_10 = u32::from_str_radix(a, 16).unwrap(); //字符串转10进制数
    let b_10 = u32::from_str_radix(b, 16).unwrap();
    a_10 <= b_10
}
pub fn vec_to_string(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}
//字符串模糊查询
#[allow(dead_code)]
pub fn fuzzy_search(query: &str, target: &str) -> bool {
    // 转换查询字符串为小写，以进行不区分大小写的搜索
    query.to_lowercase().contains(&target.to_lowercase())
}
