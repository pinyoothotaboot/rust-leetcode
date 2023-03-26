/*
28. Find the Index of the First Occurrence in a String
Companies
Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.
 
Constraints:

1 <= haystack.length, needle.length <= 104
haystack and needle consist of only lowercase English characters.
*/

fn get_index_str(haystack: String, needle: String) -> Result<i32,&'static str> {
    /**
     *  ประกาศตัวแปรเพื่อเก็บความยาวของ haystack , needle
     */
    let len_of_needle : usize = needle.len();
    let len_of_haystack : usize = haystack.len();
    /**
     * ประกาศตัวแปรสำหรับทำ sliding window
     */
    let mut idx : usize = 0;
    let mut idy : usize = 0;

    /**
     * Loop แบบทำงานตลอด
     */
    loop {
        idy = 0;
        loop {
            /**
             * ตรวจเช็คว่า  idy == len_of_needle
             * หมายความว่าเมื่อความยาวของ idy ที่นับได้ มีความยาวเท่ากันกับความยาวของ String needle
             * ซึ่งหมายถึงข้อความของ needle นั้นมีใน haystack
             * ให้ทำการส่งค่าตำแหน่งเริ่มต้นของข้อความนั้นกลับ
             */
            if idy == len_of_needle {
                return Ok(idx as i32);
            }
            /**
             * ตรวจเช็คว่าเมื่อค้นหาข้อความ needle จนสิ้นสุดข้อความของ hastack แล้วยังไม่พบ
             * ให้ทำการส่งคืน -1 ออกไป
             */
            if idx + idy == len_of_haystack {
                return Ok(-1);
            }

            /**
             * ทำการดึง char จาก String needle,haystack และเก็บไว้ โดยที่
             * char_needle -> เก็บ Char ของ needle ที่ตำแหน่ง idy
             * char_haystack -> เก็บ Char ของ haystack ที่ตำแหน่ง idx+idy
             */
            let char_needle : char = needle.chars().nth(idy).expect("Error");
            let char_haystack : char = haystack.chars().nth(idx + idy).expect("Error");
            /**
             * ทำการเปรียบเทียบระหว่าง char ทั้งสองว่าตรงกันหรือไม่
             * ในกรณีที่ char ทั้งสองไม่ตรงกัน ก็ให้ทำการ break และออกจาก loop แล้วเริ่ม loop ใหม่
             * แต่ถ้ากรณี char ทั้งสองตรงกัน ก็ทำการ เพิ่มค่า idy ขึ้นทีละ 1 แล้ววน loop เพื่อตรวจเช็คและเปรียบเทียบอีกครั้ง
             */
            if char_needle != char_haystack {
                break;
            }
            idy+=1;
        }
        idx+=1;
    }
    Err("Could not get index of string!.")
}

fn main() {
    let haystack : String = String::from("leetcode");
    let needle : String   = String::from("leeto");

    let result = get_index_str(haystack,needle);
    match result {
        Ok(idx) => {
            println!("Index of needle is : {}",idx);
        },
        Err(e) => {
            println!("Error : {}",e);
        }
    }
    println!("Hello, world!");
}
