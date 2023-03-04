fn add(alu_input_left: u16, carry_in: u16, a: u16) -> (u16, bool, bool) {
    let in15 = alu_input_left & 0x8000;
    let a15 = a & 0x8000;

    let add_15bit = (alu_input_left & 0x7FFF) + (a & 0x7FFF) + carry_in;

    let c15 = add_15bit & 0x8000;

    let last_adder = (in15 as u32) + (a15 as u32) + (c15 as u32); // add in place

    let result = add_15bit & 0x7FFF | (last_adder as u16);

    let c16 = (last_adder >> 16) != 0;

    let overflow = carry ^ (c15 != 0);

    (result, carry, overflow)
}

fn main() {
    for a in 0_u16..=0xFFFF {
        dbg!(a);
        for b in 0_u16..=0xFFFF {
            assert!(add(a, 0, b).0 == a.wrapping_add(b));
            assert!(add(a, 1, b).0 == a.wrapping_add(b).wrapping_add(1));
        }
    }
}