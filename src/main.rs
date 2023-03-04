use std::io::{self, Write, Read};
use std::time::SystemTime;
use num_format::{Locale, ToFormattedString};

const SAMPLE_RATE: f64 = 10_000_000f64;

fn add_with_flags(alu_input_left: u16, carry_in: u16, alu_input_right: u16) -> (u16, bool, bool) {

    let add_15bit = (alu_input_left & 0x7FFF) + (alu_input_right & 0x7FFF) + carry_in; // add 15 bits to get the carry

    // implement a full-adder for the 16th bit so we can get the output
    let c15 = add_15bit & 0x8000; // this will help us later
    let in15 = alu_input_left & 0x8000;
    let a15 = alu_input_right & 0x8000;
    let last_adder = (in15 as u32) + (a15 as u32) + (c15 as u32); // add in place
    // 17th bit is the c16 carry out

    // add last sum bit back to the 15 bit addition
    let result = add_15bit & 0x7FFF | (last_adder as u16);

    // generate carry and overflow flags
    let carry = (last_adder >> 16) != 0;
    let overflow = carry ^ (c15 != 0);

    (result, carry, overflow)

} 

struct Memory {
    ram: [u16; 2_usize.pow(16)]
}

impl Memory {
    fn write(&mut self, addr: u16, value: u16, mem2ser: bool) {
        match mem2ser {
            false => {self.ram[addr as usize] = value},
            true  => {io::stdout().write_all(&value.to_be_bytes()).unwrap()}
        }
    }

    fn read(&self, addr: u16, mem2ser: bool) -> u16 {
        match mem2ser {
            false => {self.ram[addr as usize]},
            true  => {
                let mut buff: [u8; 2] = [0; 2];
                io::stdin().read(&mut buff).expect("Read from stdin");
                return ((buff[0] as u16) << 8) | (buff[1] as u16);
            }
        }
    }
}

fn run(mut mem: Memory) {

    let mut ip: u16 = 0;
    let mut sp: u16 = 0xFFFF;
    let mut a:  u16 = 0;

    let mut i = 0;
    let mut start = SystemTime::now();


    loop {

        if sp < 0x8000 || ip > 0x8000 {
            //dbg!(mem.ram);
            break;
        } 

        if a == 28657 {
            break;
        }
        // debug
        //dbg!(ip, sp, a);
        //dbg!(mem.ram[0xFFFF], mem.ram[0xFFFE]);
        i += 1;
        // CLOCK RISING EDGE
        //dbg!(i);

        // "FETCH" -----------------------------------------------------------------------------
        let instr = mem.read(ip, false);
        //println!("0b{:016b}", instr);

        //dbg!(instr);


        // "DECODE" -----------------------------------------------------------------------------
        let alu_left_reg   =  instr & 0b1110000000000000; // could shift them to all start from 0, but that would take an extra operation. 
        let alu_right_reg  =  instr & 0b0001100000000000;
        let carry_in       = (instr & 0b0000010000000000) >> 10; // rawdogging these guys
        let alu_op         =  instr & 0b0000001000000000;
        let alu_output_reg =  instr & 0b0000000110000000;
        let decr_sp        = (instr & 0b0000000001000000) >> 6; //0 or 1
        let incr_sp        = (instr & 0b0000000000100000) >> 5; //0 or 1
        let cond_incr_ip   =  instr & 0b0000000000011000;
        let incr_ip        = (instr & 0b0000000000000100) >> 2; //0 or 1
        let mem2ser       = (instr & 0b0000000000000010) != 0; //0 or 1

        //dbg!(incr_ip);

        let alu_input_left: u16 = match alu_left_reg { // Could be implemented with a MUX
            0x0000 => {0}, // Zeros
            0x2000 => {mem.read(sp, mem2ser)}, // Memory (perform memory read if possible)
            0x4000 => {panic!("This register not implemented yet.")}, // resevered
            0x6000 => {panic!("This register not implemented yet.")}, // TODO figure out Magic Constant
            0x8000 => {a}, // A
            0xA000 => {!a}, // NOT A
            0xC000 => {sp}, // SP (Stack Pointer)
            0xE000 => {ip}, // IP (Instruction Pointer)
            _ => unreachable!()
        };

        let alu_input_right = match alu_right_reg { // 0b0001_1000_0000_0000
            0x0000 => {0}, // 0
            0x0800 => {a}, // a
            0x1000 => {0xFFFF}, // NOT 0
            0x1800 => {!a}, // NOT a
            _ => unreachable!()
        };

        // EXECUTE -----------------------------------------------------------------------------

        debug_assert!(alu_op == 0 || alu_op == 0b0000001000000000);
        debug_assert!(carry_in == 0 || carry_in == 1);     

        // pattern = (result, carry, overflow)
        let alu_op_out = if alu_op != 0 {
            (!(alu_input_left & alu_input_right), false, false)

        } else if cond_incr_ip == 0 { // avoid expensive add_with_flags calculation if not nessecary
            ( alu_input_left.wrapping_add(alu_input_right).wrapping_add(carry_in), false, false)

        } else {
            add_with_flags(alu_input_left, carry_in, alu_input_right)
        }; 

        // CLOCK FALLING EDGE

        // "WRITE" -----------------------------------------------------------------------------

        sp -= decr_sp; // decrement SP

        match alu_output_reg { //0b0000_0001_1000_0000
            0x0000 => {a = alu_op_out.0}, // A
            0x0080 => {mem.write(sp, alu_op_out.0, mem2ser)}, // Mem
            0x0100 => {sp = alu_op_out.0}, // SP
            0x0180 => {ip = alu_op_out.0}, // IP
            _ => unreachable!()
        };

        sp += incr_sp; // increment SP
        ip += incr_ip; // increment IP


        match cond_incr_ip { // mask: 0b0000_0000_0001_1000
            0x0000 => {sp += 0}, // Don't bother
            0x0008 => {sp += ((alu_op_out.0 == 0) as u16)}, // ZF
            0x0010 => {sp += (alu_op_out.1 as u16)}, // CF
            0x0018 => {sp += (alu_op_out.2 as u16)}, // OF
            _ => unreachable!()
        }

        // if i % (SAMPLE_RATE as u64) == 0 {
        //     let time = start.elapsed().unwrap().as_secs_f64();

        //     println!("{:.3}MHz", (1.0 / (time / SAMPLE_RATE)) / 1_000_000.0);
        //     start = SystemTime::now();
        // }
    }

    let time = start.elapsed().unwrap().as_secs_f64();

    println!("{}s", time);
    dbg!(&mem.ram[0xFFE0..0xFFFF]);


}

fn main() {
    let mut mem = Memory {ram: [0; 2_usize.pow(16)]};

    mem.ram[0] = 0b000_00_1_0_00_0_0_00_1_0_0; // set A = 1
    mem.ram[1] = 0b000_01_0_0_01_1_1_00_1_0_0; // 0 + A -> s[1], decr sp then incr sp
    mem.ram[2] = 0b001_01_0_0_00_1_0_10_1_0_0; // s[0] + A -> A, decr sp
    mem.ram[3] = 0b000_01_0_0_01_1_1_00_1_0_0; // 0 + A -> s[2], decr sp then incr sp
    mem.ram[4] = 0b001_01_0_0_00_1_0_10_1_0_0; // s[1] + A -> A, decr sp
    mem.ram[5] = 0b000_00_0_0_11_0_0_00_1_0_0; // jump to beginning. normally jumps dont incr sp but they can if it's advantageous
    mem.ram[6] = 0; // effectively a NOP (clears a as well)
    //mem.ram[2] = 0b;

    mem.ram[0xFFFE] = 1;
    mem.ram[0xFFFF] = 1;


    run(mem);


}
