use std::vec;

//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let  mut a = vec![1,2,3,4,5];
    let  mut i = 0;
    let  mut c = 0;
    loop {
        let (a,&c) = test(a);
        println!("{}",&c);
        i += 1;
        if i >=5 {break};
    }
}

pub fn test( a: &Vec<u8>, c: &i32) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}