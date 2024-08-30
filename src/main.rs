use wasm_bindgen::prelude::*;
use std::cmp::Ordering;
use std::iter::Sum;
use std::num::FpCategory::Nan;
use std::ops::Add;
use std::ops::Drop;
use std::str::FromStr;

//基础版本的数组操作
trait MyJsApiArray<T>{
    fn new(kw: Vec<T>) -> Vec<T>; //新建一个数组
    fn pop(&mut self)->Option<T>;//删除最后一个元素
    fn append(&mut self, value: T)->Option<T>; //添加一个元素到数组末尾
    fn index(&self,value: T)->String; //获取数组中指定位置的元素
    // fn inert(&mut self, index: usize, value: T); //在指定位置插入一个元素
    fn len(&self) ->usize; //获取数组的长度
    // fn remove(&mut self, index: usize)->Self; //删除指定位置的元素
    // fn clear(&mut self)->Self<T>; //清空数组
    // fn cmp(&self, kw1: Self<T>,kw2: Self<T>)->bool; //比较两个数组是否相等
    // fn min(&self,kw: Self<T>)->T; //获取数组中的最小值
    // fn max(&self,kw: Self<T>)->T; //获取数组中的最大值
    // fn count(&self,kw: Self<T>)->usize; //获取数组中某个元素出现的次数
    fn sum(&self)->T; //求数组元素的和
    // fn reverse(&mut self,kw: Self<T>)->Self<T>; //反转数组
     //fn  reduce(&self,fn: fn(T,T)->T)->T; //对数组中的元素进行累加
    // fn filter(&self,fn: fn(T)->bool)->Vec<T>; //过滤数组中的元素
    // fn map(&self,fn: fn(T)->T)->Vec<T>; //对数组中的元素进行操作

}
impl<T: std::clone::Clone + std::cmp::PartialEq + std::fmt::Debug +std::fmt::Display>  MyJsApiArray<T> for Vec<T>
 where T:Add<Output=T>+Sum<T>+Copy+Clone
{
    fn new(kw: Vec<T>) -> Vec<T> {
        //如果数组为空，则返回一个空数组
        let result=match kw.len() {
            0 =>kw.clone().to_vec(),
            _ => {
                let mut data = vec![];
                let len = kw.len();
                for i in 0..len {
                    let temp=&kw[i];
                    data.push(temp.clone());
                }
                Vec::from(data)
            }
        };
        result
    }
    fn pop(&mut self)->Option<T> {
        if self.is_empty(){
            None
        }else{
            if self.len()==0{
                None
            }
            else if self.len()==1{
                Some(self.remove(0))
            }else{
                Some(self.remove(self.len()-1))
            }
        }
    }
    fn append(&mut self, value: T)->Option<T> {
        &self.push(value);
        None
    }
    fn len(&self) ->usize {
        self.len()
    }
    fn sum(&self)->T where
        T:Add<Output=T>+Sum<T>+Copy,
    {
        self.iter().copied().sum()
    }
    fn index(&self,value: T)->String {
        let mut index: i32 = 0;
        let mut result = String::new();
        if self.len() == 0 {
            result="数组为空".to_string();

        }
        for i in 0..self.len() {
            if self[i] == value {
                index = i as i32;
                result =index.to_string();
                break;
            }else{
                result = "没有找到该元素".to_string();
            }
        }
        result
    }
}
fn baseArray(){
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut arr2 = <Vec<i32> as MyJsApiArray<i32>>::new(arr);
    println!("{:?}", arr2);
    println!("{:?}", arr2.len());
    println!("{:?}", arr2.sum());
    arr2.pop();
    println!("{:?}", arr2);
    arr2.append(&mut vec![10,11, 12, 13, 14, 15]);
    println!("{:?}", arr2);
    println!("{:?}", arr2.len());
    println!("{:?}", arr2.sum());
    let result = arr2.index(16);
    println!("result{:?}", result);
}
fn main() {
    baseArray();
    println!("Hello, world!");
}
