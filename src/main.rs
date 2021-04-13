use brain_frick::BfInterpret;
use std::time::Instant;

extern crate perfcnt;
extern crate x86;

use perfcnt::linux::PerfCounterBuilderLinux as Builder;
use perfcnt::linux::SoftwareEventType as Software;
use perfcnt::{AbstractPerfCounter, PerfCounter};
use std::process;

#[test]
fn test_hello_world() -> () {
    use std::collections::VecDeque;
    let bf = BfInterpret::new(
        "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.?".to_string(),
    )
    .unwrap();
    let mut num = 0;
    let mut chars = VecDeque::from(vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);
    for code in bf {
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

fn test_default() {
    use brainfuck;
    let mut pc: PerfCounter = Builder::from_software_event(Software::TaskClock)
        .on_all_cpus()
        .for_pid(process::id() as i32)
        .finish()
        .expect("Could not create counter");

    let mut inst: u64 = 0;
    let now = Instant::now();
    for _ in 0..1_000 {
        pc.start().expect("Can not start the counter");
        //brainfuck::eval_string("+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.").unwrap();
        brainfuck::eval_string("+[-[<<[+[--->]-[<<<]]]>>>-]>---->><<<<-<+>>>>>><<<-").unwrap();
        inst += pc.read().expect("Can not read counter");
        pc.reset().expect("Can not reset the counter");
    }
    let num = 10623 * 1_000; //3570766770-3894325034
    println!(
        "\nbrainfuck: {:?}, {}: {:.3}",
        now.elapsed(),
        inst,
        inst as f64 / num as f64
    );
}

fn test_brain_frick() {
    let mut pc: PerfCounter = Builder::from_software_event(Software::TaskClock)
        .on_all_cpus()
        .for_pid(process::id() as i32)
        .finish()
        .expect("Could not create counter");

    let mut inst: u64 = 0;
    let now = Instant::now();
    for _ in 0..1_000 {
        let bf = BfInterpret::new(
            "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.?".to_string(),
        )
        .unwrap();
        // for code in bf {
        //     println!("{}", code);
        // }
        // return;

        //let mut num = 0;

        pc.start().expect("Can not start the counter");
        for code in bf {
            //num += 1;
            match code {
                brain_frick::bf_parse::Operation::LoopStart() => {}
                brain_frick::bf_parse::Operation::LoopEnd() => {}
                brain_frick::bf_parse::Operation::Put(_) => {}
                _ => {} //print!("{}", code),
            };
        }
        pc.stop().expect("Can not stop the counter");
        //let inst = unsafe { _rdtsc() } - now;
        inst += pc.read().expect("Can not read counter");
        pc.reset().expect("Can not reset the counter");
        //println!("inst: {}, inst/op: {}", inst, (inst as f64) / num as f64);
    }
    let num = 10623 * 1_000; //3570766770-3894325034
    println!(
        "\nbrain_frick: {:?}, {}: {:.3}",
        now.elapsed(),
        inst,
        inst as f64 / num as f64
    );
}

fn main() {
    test_default();
    test_brain_frick();
}
