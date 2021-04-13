use core::arch::x86_64::_rdtsc;
use std::time;
use std::collections::VecDeque;

use brain_frick::BfInterpret;

#[test]
fn test_hello_world() -> (){
    let bf = BfInterpret::new(
        "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.?".to_string(),
    )
    .unwrap();
    let mut num = 0;
    let mut chars = VecDeque::from(vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);
    for code in bf{
        match code {
            brain_frick::bf_parse::Operation::Put(char) => {
                assert!(char == chars.pop_front().unwrap());
            }
            _ => {}
        };
        num += 1;
    }
    assert!(num == 10623);
}

fn main() {
    
    let bf = BfInterpret::new(
        "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.?".to_string(),
    )
    .unwrap();
    // for _ in 0..1000000 {
    //     //println!("{:?}", BF);
    //     BF.step();
    // }
    let now = unsafe { _rdtsc() }; //time::Instant::now();
                                   //let mut num = 0;
    for code in bf {
        //num += 1;
        match code {
            brain_frick::bf_parse::Operation::LoopStart() => {}
            brain_frick::bf_parse::Operation::LoopEnd() => {}
            brain_frick::bf_parse::Operation::Put(_) => {}
            _ => {} //print!("{}", code),
        };
    }
    let inst = unsafe { _rdtsc() } - now;
    let num = 10623;
    // println!(
    //     "\n{:?}, {}: {:?}",
    //     now.elapsed(),
    //     num,
    //     now.elapsed() * 1000 / num
    // );
    println!("inst: {}, inst/op: {}", inst, (inst as f64) / num as f64);
}
