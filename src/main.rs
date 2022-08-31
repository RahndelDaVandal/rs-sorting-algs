#![allow(dead_code)]
use rand::{thread_rng, Rng};

fn main() {
    let rand_vec = gen_vec_of_f32(0.0, 10.0, 10);
    println!("unsorted: {}", fmt_vec_f32(&rand_vec));
    let sorted = quick_sort(rand_vec);
    println!("sorted: {}", fmt_vec_f32(&sorted));
}

fn quick_sort(vector: Vec<f32>) -> Vec<f32>{
    if vector.len() <= 1{
        return vector;
    }
    let mut lt_pivot: Vec<f32> = Vec::new();
    let mut eq_pivot: Vec<f32> = Vec::new();
    let mut gt_pivot: Vec<f32> = Vec::new();
    let pivot = vector[vector.len()/2];
    for v in vector {
        if v < pivot {
            lt_pivot.push(v);
        } else if v > pivot {
            gt_pivot.push(v);
        } else if v == pivot {
            eq_pivot.push(v);
        } else {
            panic!("Error comparing v to pivot");
        }
    }
    [
        quick_sort(lt_pivot),
        quick_sort(eq_pivot),
        quick_sort(gt_pivot),
    ].concat()
}

fn gen_vec_of_f32(min: f32, max: f32, len: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    let mut output: Vec<f32> = Vec::new();
    for _ in 0..len {
        output.push(rng.gen_range(min..max));
    }
    output
}

fn bubble_sort(mut list: Vec<f32>) -> Vec<f32>{
    let mut swap_count = 0;
    loop {
        for i in 0..list.len() - 1 {
            let x = list[i];
            let y = list[i+1];
            if x > y {
                list[i] = y;
                list[i+1] = x;
                swap_count += 1;
            }
        }
        if swap_count == 0 { break };
        swap_count = 0;
    }
    list
}

fn fmt_vec_f32(nums: &Vec<f32>) -> String {
    let mut output:String = String::from("[ ");
    let num_len = nums.len();
    for (i, n) in nums.iter().enumerate() {
        if i == num_len - 1 {
            let tmp = format!("{:.02}", n);
            output.push_str(&tmp);
        } else {
            let tmp = format!("{:.02}, ", n);
            output.push_str(&tmp);
        }
    }
    output.push_str(" ]");
    output
}

