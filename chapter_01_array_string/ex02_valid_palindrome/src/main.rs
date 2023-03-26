/**

125. Valid Palindrome
Companies
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.
 

Constraints:

1 <= s.length <= 2 * 105
s consists only of printable ASCII characters.
 */

/**
 * Function : is_letter_or_digit
 * @sync
 * About : ตรวจเช็คว่าตัวอักษรที่เข้ามาเป็นอักขระ a-z,A-Z,0-9 หรือไม่
 * Param :
 *      - cha : ตัวอักษรที่ต้องการเช็ค
 * Return :
 *      - Boolean
 */
fn is_letter_or_digit(cha : char) -> bool {
    if cha.is_ascii_alphanumeric()  {
        return true;
    }
    return false;
}

/**
 * Function : to_lower_str
 * @sync
 * About : แปลงตัวอักษรใหญ่เป็นตัวอักษรเล็ก
 * Param :
 *      - cha : ตัวอักษรที่ต้องการแปลง
 * Return :
 *      - String อักษรที่แปลงเสร็จ
 */
fn to_lower_str(cha : char) -> String {
    return cha.to_lowercase().to_string();
}

/**
 * Function : is_palindrome
 * @sync
 * About : เช็ค String ที่เข้ามาว่าเป็น Palindrome หรือไม่
 * Param :
 *      - text : เป็นข้อความที่ป้อนเข้ามา
 * Return :
 *      -  Boolean 
 */
fn is_palindrome(text : String) -> bool {
    /**
     * ประกาศตัวแปรสำหรับการเลื่อนไปใน String โดยที่
     * slide_left -> เริ่มจาก index ที่ 0 
     * slide_right -> เริ่มจาก index ที่ความยาวของ String - 1
     */
    let mut slide_left : usize = 0;
    let mut slide_right : usize = text.len() - 1;
    /**
     * ประกาศตัวแปรเพื่อเก็บ Char ที่ได้จาก String ทีี่ตำแหน่ง slide_left ,slide_right โดยที่
     * cha_left -> เก็บ char ฝั่ง left slide
     * cha_right -> เก็บ char ฝั่ง right slide
     */
    let mut cha_left : char = text.chars().nth(slide_left).expect("Error");
    let mut cha_right : char  = text.chars().nth(slide_right).expect("Error");
    /**
     * สร้าง Loop โดยกำหนดเงื่อนไขว่า slide_left < slide_right 
     * ซึ่งจะออกจาก Loop ก็ต่อเมื่อ slide_left == slide_right
     * หรือ มีการ return
     */
    while slide_left < slide_right {
        /**
         * Loop เพื่อเช็คว่า left char นั้นเป็น
         * a-z , A-Z ,0-9 หรือไม่ 
         * ในกรณีที่ไม่เป็น ก็จะทำการเลื่อน slide_left เพิ่มขึ้น และตรวจเช็คอีกครั้งจนกว่า
         * จะได้ char ที่ต้องการ
         */
        while (slide_left < slide_right) && !is_letter_or_digit(cha_left) {
            slide_left+=1;
            cha_left = text.chars().nth(slide_left).expect("Error");
        };
        /**
         * Loop เพื่อเช็คว่า right char นั้นเป็น
         * a-z , A-Z ,0-9 หรือไม่ 
         * ในกรณีที่ไม่เป็น ก็จะทำการเลื่อน slide_right ลดลง และตรวจเช็คอีกครั้งจนกว่า
         * จะได้ char ที่ต้องการ
         */
        while (slide_left < slide_right) && !is_letter_or_digit(cha_right) {
            slide_right-=1;
            cha_right  = text.chars().nth(slide_right).expect("Error");
        }
        /**
         * ทำการเปรียบเทียบ char ทั้งสองฝั่งว่าตรงกันหรือไม่ โดยที่ไม่สนว่าจะเป็นตัวใหญ่หรือตัวเล็ก
         * ในกรณีที่ char ทั้งสองฝั่งไม่ตรงกันจะทำการ return false กลับไป
         */
        if to_lower_str(cha_left) != to_lower_str(cha_right){
            return false;
        }
        
        /**
         * เลื่อน slide_left ไปทางขวา โดยการเพิ่มค่าทีละ 1
         * เลื่อน slide_right มาทางซ้าย โดยการลดค่าทีละ 1
         */
        slide_left+=1;
        slide_right-=1;
    }
    return true;
}

fn main() {
    let text : String = String::from("A man, a plan, a canal: Panama");
    println!("Is palindrome : {:?}",is_palindrome(text));
}
