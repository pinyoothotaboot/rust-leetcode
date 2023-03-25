/**
 * 
1. Two Sum
Companies
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
 

Constraints:

2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109
Only one valid answer exists.
 

Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
 */
use std::collections::HashMap;
use std::sync::RwLock;

const NONE_NUMBER : i32 = -999;

struct TwoSum {
    nums : Vec<i32>,
    maps : HashMap<i32,i32>,
    lock : RwLock<u32>
}

trait TwoSumInterface {
    fn new(nums : Vec<i32>) -> Self;

    fn add(&mut self,key : i32 , value : i32);

    fn get(&self,key : &i32) -> i32;

    fn find(&mut self,target : i32) -> Vec<i32>;
}

impl TwoSumInterface for TwoSum {
    /**
     * Function : new
     * @sync
     * About : Initial new class
     * Param :
     *      - nums : Array of number
     * Return :
     *      Object (TwoSum)
     */
    fn new(nums : Vec<i32>) -> TwoSum {
        TwoSum {nums : nums , maps : HashMap::new() , lock : RwLock::new(5)}
    }

    /**
     * Function : add
     * @sync
     * About :  Add data to maps
     * Param :
     *      - key : Number of array
     *      - value : Number of iterator array
     */
    fn add(&mut self, key : i32 , value : i32) {
        let mut write_lock = self.lock.write().unwrap();
        self.maps.insert(key,value);
        *write_lock+=1;
    }

    /**
     * Function : get
     * @sync
     * About : Get number of iterator from maps
     * Param :
     *      - key : Pointer of number to get data
     * Return :
     *      Number of iterator if successed else constant number 
     */
    fn get(&self, key : &i32) -> i32 {
        let read_lock = self.lock.read().unwrap();
        if self.maps.contains_key(key) {
            let value : i32 = match self.maps.get(key) {
                Some(value) => *value,
                None => panic!("Not found value!..")
            };
            return value;
        }
        return NONE_NUMBER;
    }

    /**
     * Function : find
     * @sync
     * About : Find number of iterator in array when two number in array sum equal at target
     * Param :
     *      - target : Number of target to find
     * Return :
     *      Array of iterator two sum number
     */
    fn find(&mut self, target : i32) -> Vec<i32> {
        for idx in 0..self.nums.len() {
            let num : i32 = self.nums[idx];
            let idy : i32 = self.get(&(target - num));
            if idy != NONE_NUMBER {
                return vec![idy,idx as i32];
            }
            let x : i32 = idx as i32;
            self.add(num,x);
        }

        return vec![];
    }
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    /**
     * สร้างตัวแปร maps เป็นชนิด key , value ไว้สำหรับเก็บ 
     * key -> เก็บตัวเลขที่ได้ใน Array
     * value -> เก็บตัวเลขตำแหนงของ Array
     */
    let mut maps : HashMap<i32,i32> = HashMap::new();
    // ทำการวนลูปตั้งแต่ 0 ไปจนถึง ขนาดของ Array num - 1
    for idx in 0..nums.len() {
        // เก็บค่าตัวเลขปัจจุบันที่ตำแหน่ง Array num[idx]
        let current_num : i32 = nums[idx];
        /**
         * ตรวจเช็คว่าข้อมูลเลขของ target - current_num นั้นมีอยู่ใน maps หรือไม่
         * เช่น ถ้า current_num = 2 และ target = 9 ก็จะได้ว่า 9-2 = 7 
         * แล้วนำไปเช็คว่ามี key = 7 อยู่ใน maps หรือไม่
         */
        if maps.contains_key(&(target - current_num)) {
            /**
             * กรณีพบว่ามีข้อมูลตัวเลขที่ต้องการอยู่ใน maps ก็ทำการดึงเอาข้อมูล value จาก maps
             * ซึ่งค่าที่ได้จะเป็นเลขตำแหน่งของ Array ที่ target - currrent_num
             */
            let idy : i32 = match maps.get(&(target  - current_num)) {
                Some(value) => *value,
                None => panic!("Not found value!..")
            };
            return vec![idy,idx as i32];
        }
        /**
         * ทำการเพิ่มข้อมูลไปใน maps ในกรณีที่ current_num ยังไม่มี
         * key -> current_num เก็บเป็นตัวเลขที่ได้จาก Array
         * value -> idx เก็บเป็นตำแหน่งของ Array
         */
        maps.insert(current_num,idx as i32);
    }
    return vec![];
}

fn main() {
    let nums : Vec<i32> = vec![2,7,11,15];
    let target : i32 = 9;
    let mut twosum : TwoSum = TwoSumInterface::new(nums);
    let result : Vec<i32> = twosum.find(target);
    println!("Result is : {:?}",result);
}
