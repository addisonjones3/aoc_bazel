use crate::puter::program::{Instruction, Operation, Program};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{u8 as parse_u8, usize as parse_usize},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map_res(parse_u8, Operation::try_from).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    (parse_operation, count(preceded(tag(","), parse_usize), 3))
        .map(|(operation, ints)| Instruction {
            operation,
            pos_1: ints[0],
            pos_2: ints[1],
            target_pos: ints[2],
        })
        .parse(input)
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(separated_list1(tag(","), parse_instruction), tag(",99")).parse(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    separated_pair(
        parse_instructions,
        tag(","),
        separated_list1(tag(","), parse_usize),
    )
    .map(|(instructions, slots)| Program {
        instructions,
        slots,
    })
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_INSTRUCTION_STR: &str = "1,0,0,3";
    const MULTI_INSTRUCTION_STR: &str = "1,9,10,3,2,3,11,0,99";
    const BASIC_INSTRUCTION: Instruction = Instruction {
        operation: Operation::Sum,
        pos_1: 0,
        pos_2: 0,
        target_pos: 3,
    };
    const BASIC_PROG_STR: &str = "1,0,0,3,99";
    const FULL_PROG_STR: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn test_parse_operation() {
        let (_, add_op) = parse_operation("1").expect("couldn't parse operation");
        assert_eq!(add_op, Operation::Sum);

        let (_, mult_op) = parse_operation("2").expect("couldn't parse operation");
        assert_eq!(mult_op, Operation::Mult);
    }

    #[test]
    fn test_parse_instruction() {
        let (rest, got_instr) =
            parse_instruction(BASIC_INSTRUCTION_STR).expect("couldn't parse instruction");
        eprintln!("rest: {rest}");
        assert_eq!(got_instr, BASIC_INSTRUCTION);
    }

    #[test]
    fn test_parse_instructions() {
        let (rest, got_instrs) =
            parse_instructions(MULTI_INSTRUCTION_STR).expect("couldn't parse instruction");
        eprintln!("rest: {rest}");
        eprintln!("got_instrs: {got_instrs:?}");
        // assert_eq!(got_instr, BASIC_INSTRUCTION);
    }

    #[test]
    fn test_parse_program() {
        let (rest, got_program) =
            parse_instructions(FULL_PROG_STR).expect("couldn't parse instruction");
        eprintln!("rest: {rest}");
        eprintln!("got_instrs: {got_program:?}");
        // assert_eq!(got_instr, BASIC_INSTRUCTION);
    }
}
