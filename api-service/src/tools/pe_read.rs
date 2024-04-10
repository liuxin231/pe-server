use serde::{Deserialize, Serialize};
use std::vec;

use crate::tools::pe_tools;

#[derive(Debug, Deserialize, Serialize)]
pub struct PeStudy {
    pub pe_head: String,
    pub subsection_size: String,
    pub optional_head_size: String,
    pub subsection_information: Vec<Vec<String>>,
    pub import_real: String,
    pub iat_shifting: String,
    pub iat_real: String,
    pub byname_shifting: String,
    pub byname_real: String,
    pub byname_information: Vec<String>,
    pub pe_study: Vec<Vec<String>>,
    pub field_name: String,
    pub field_size: String,
}
impl PeStudy {
    pub fn generate_report(&self, result: String) -> String {
        let mut report = String::new();
        report.push_str("===== PE Study 报告 =====\n\n");
        report.push_str("一、文件信息\n");
        report.push_str(&format!("文件名称: {}\n", self.field_name));
        report.push_str(&format!("文件大小: {}\n", self.field_size));
        report.push_str(&format!("检测结果: {}\n\n", result));
        report.push_str("二、pe结构\n");
        report.push_str(&format!("PE 头: {}\n", self.pe_head));
        report.push_str(&format!("可选头大小: {}\n", self.optional_head_size));
        report.push_str(&format!("节表个数: {}\n", self.subsection_size));
        report.push_str("节表信息:\n");
        for (index, info) in self.subsection_information.iter().enumerate() {
            report.push_str(&format!("第{}个节表（偏移、真实地址）：\t", index + 1));
            for item in info {
                report.push_str(&format!("{}\t", item));
            }
            report.push('\n');
        }

        report.push_str(&format!("导入表真实地址: {}\n", self.import_real));
        report.push_str(&format!("IAT表偏移: {}\n", self.iat_shifting));
        report.push_str(&format!("IAT表真实地址: {}\n", self.iat_real));

        report.push_str(&format!("Byname表偏移: {}\n", self.byname_shifting));
        report.push_str(&format!("Byname表真实地址: {}\n\n", self.byname_real));

        report.push_str("三、调用的系统函数名称:\n");
        for item in &self.byname_information {
            report.push_str(&format!(
                "{}\n",
                String::from_utf8_lossy(&hex::decode(item).unwrap())
            ));
        }
        report.push_str("=============================\n");

        report
    }
}
#[allow(dead_code)]
impl PeStudy {
    pub fn new() -> Self {
        PeStudy {
            pe_head: "".to_string(),
            subsection_size: "".to_string(),
            optional_head_size: "".to_string(),
            subsection_information: Vec::new(),
            import_real: "".to_string(),
            iat_shifting: "".to_string(),
            iat_real: "".to_string(),
            byname_shifting: "".to_string(),
            byname_real: "".to_string(),
            byname_information: Vec::new(),
            pe_study: Vec::new(),
            field_name: "".to_string(),
            field_size: "0 byte".to_string(),
        }
    }
}
pub fn read_exe_file(
    hex_string: String,
    file_name: String,
    file_size: String,
) -> Result<PeStudy, std::io::Error> {
    let mut arr: Vec<String> = Vec::new();

    // 将字符串按照两个字符一组进行拆分，并存入数组
    for i in 0..hex_string.len() / 2 {
        let substr = &hex_string[i * 2..(i + 1) * 2];
        arr.push(substr.to_string());
    }

    let num_cols = 16; // 列数
    let num_rows = (arr.len() + num_cols - 1) / num_cols; // 行数

    let mut matrix: Vec<Vec<String>> = vec![vec!["".to_string(); num_cols]; num_rows]; // 初始化二维数组

    for (i, value) in arr.iter().cloned().enumerate() {
        let col = i % num_cols; // 计算当前值所在的列数
        let row = i / num_cols; // 计算当前值所在的行数
        matrix[row][col] = value.to_string();
    }
    //println!("{:?}", matrix[1]);
    let s1: String = pe_tools::read_hex(&matrix, 4, 3, 15);
    // println!("pe头：{}", s1);

    let s2 = pe_tools::add_hex(&s1, "6");
    // println!("{}", s2);
    let s3 = &matrix[pe_tools::position1(&s2)][pe_tools::position2(&s2)];
    // println!("节表的个数：{}", s3);

    let s4 = pe_tools::add_hex(&s1, "83");
    // println!("{}", s4);
    let s5 = pe_tools::read_hex(
        &matrix,
        4,
        pe_tools::position1(&s4),
        pe_tools::position2(&s4),
    );
    // println!("导入表偏移：{}", s5);

    let s6 = pe_tools::add_hex(&s1, "15");
    let s7 = pe_tools::read_hex(
        &matrix,
        2,
        pe_tools::position1(&s6),
        pe_tools::position2(&s6),
    );
    // println!("可选头大小：{}", s7);

    let s8 = pe_tools::add_hex(&s1, "18");
    let s9 = pe_tools::add_hex(&s7, &s8);
    // println!("第一个节开始位置：{}", s9);

    let mut s10 = pe_tools::add_hex(&s9, "0f");
    let s11 = pe_tools::read_hex(
        &matrix,
        4,
        pe_tools::position1(&s10),
        pe_tools::position2(&s10),
    );
    // println!("第一个节的偏移：{}", s11);

    let mut s12 = pe_tools::add_hex(&s10, "08");
    let s13 = pe_tools::read_hex(
        &matrix,
        4,
        pe_tools::position1(&s12),
        pe_tools::position2(&s12),
    );
    // println!("第一个节的真实地址：{}", s13);

    let mut table_arr: Vec<Vec<String>> = vec![vec!["".to_string(); 2]; s3.parse().unwrap_or(0)];
    // println!("{:?}{:?}",&table_arr,s3.parse().unwrap_or(0));
    table_arr[0][0] = s11;
    table_arr[0][1] = s13;

    for x in 0..s3.parse().unwrap_or(0) - 1 {
        s10 = pe_tools::add_hex_cycle(&mut s10, "28");
        let n1 = pe_tools::read_hex(
            &matrix,
            4,
            pe_tools::position1(&s10),
            pe_tools::position2(&s10),
        );
        s12 = pe_tools::add_hex_cycle(&mut s12, "28");
        let n2 = pe_tools::read_hex(
            &matrix,
            4,
            pe_tools::position1(&s12),
            pe_tools::position2(&s12),
        );

        table_arr[x + 1][0] = n1;
        table_arr[x + 1][1] = n2;
    }
    // println!("所有表信息：{:?}", &table_arr);

    //判断导入表位于哪个表
    let mut ii = 0;
    while pe_tools::hex_compare1(&s5, &table_arr[ii][0]) {
        ii += 1;
    }
    let s14 = pe_tools::add_hex(
        &pe_tools::sub_hex(&s5, &table_arr[ii - 1][0]),
        &table_arr[ii - 1][1],
    );
    // println!("导入表真实地址：{:?}", &s14);

    let s15 = pe_tools::add_hex(&s14, "03");
    let s16 = pe_tools::read_hex(
        &matrix,
        4,
        pe_tools::position1(&s15),
        pe_tools::position2(&s15),
    );
    // println!("IAT表偏移地址：{:?}", &s16);

    let s17 = pe_tools::add_hex(
        &pe_tools::sub_hex(&s16, &table_arr[ii - 1][0]),
        &table_arr[ii - 1][1],
    );
    // println!("IAT表真实地址：{:?}", &s17);

    let s18 = pe_tools::add_hex(&s17, "03");
    let s19 = pe_tools::read_hex(
        &matrix,
        4,
        pe_tools::position1(&s18),
        pe_tools::position2(&s18),
    );
    // println!("BYNAME表的偏移地址：{:?}", &s19);

    let s20 = pe_tools::add_hex(
        &pe_tools::sub_hex(&s19, &table_arr[ii - 1][0]),
        &table_arr[ii - 1][1],
    );
    // println!("BYNAME表的真实地址：{:?}", &s20);

    let x = pe_tools::position1(&s20);

    let mut table_byname_temporary: Vec<String> = Vec::new();
    let mut flag = false;
    let mut k = 0;
    for i in x..matrix.len() {
        for j in 0..16 {
            if i >= matrix.len() || j >= matrix[i].len() {
                break;
            }
            table_byname_temporary.push(matrix[i][j].clone());
            if k > 16
                && table_byname_temporary[k] == ("00")
                && table_byname_temporary[k - 1] == ("00")
                && table_byname_temporary[k - 2] == ("00")
                && table_byname_temporary[k - 3] == ("00")
            {
                flag = true;
                table_byname_temporary.remove(k);
                table_byname_temporary.remove(k - 1);
                table_byname_temporary.remove(k - 2);
                table_byname_temporary.remove(k - 3);
                break;
            }
            k += 1;
        }
        if flag {
            break;
        }
    }
    // println!("BYNAME临时表的内容：{:?}", &table_byname_temporary);
    let mut table_byname: Vec<String> = vec!["".to_string(); table_byname_temporary.len()];
    let mut num = 0;
    let mut i = 0;
    while i < table_byname_temporary.len() - 1 {
        if pe_tools::hex_compare1(&table_byname_temporary[i], "21")
            && pe_tools::hex_compare2(&table_byname_temporary[i], "7e")
        {
            if pe_tools::hex_compare1(&table_byname_temporary[i + 1], "21")
                && pe_tools::hex_compare2(&table_byname_temporary[i + 1], "7e")
            {
                table_byname[num] = format!("{}{}", table_byname[num], table_byname_temporary[i]);
            } else {
                table_byname[num] = format!("{}{}", table_byname[num], table_byname_temporary[i]);
                num += 1;
            }
        } else if pe_tools::hex_compare1(&table_byname_temporary[i + 1], "21")
            && pe_tools::hex_compare2(&table_byname_temporary[i + 1], "7e")
        {
            num += 1;
        } else {
            i += 1;
        }
        i += 1;
    }

    table_byname.retain(|s| !s.is_empty());
    let mut n = 0;
    while n < table_byname.len() {
        let current_element = &table_byname[n];
        if current_element.len() == 2 {
            table_byname.remove(n);
        } else {
            n += 1
        };
    }
    // println!("BYNAME表的内容：{:?}", &table_byname);
    Ok(PeStudy {
        pe_head: s1,
        subsection_size: s3.to_owned(),
        subsection_information: table_arr,
        import_real: s5,
        iat_shifting: s14,
        iat_real: s17,
        byname_shifting: s19,
        byname_real: s20,
        byname_information: table_byname,
        pe_study: matrix,
        optional_head_size: s7,
        field_name: file_name,
        field_size: file_size,
    })
}
