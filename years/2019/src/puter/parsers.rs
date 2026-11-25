use crate::puter::program::Executor;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::usize as parse_usize,
    multi::separated_list1,
};

pub fn parse_executor(input: &str) -> IResult<&str, Executor> {
    separated_list1(tag(","), parse_usize)
        .map(|memory| Executor { memory })
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const FULL_PROG_STR: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn test_parse_executor() {
        let res: IResult<&str, Vec<usize>> =
            separated_list1(tag(","), parse_usize).parse(FULL_PROG_STR);
        let (_, memvec) = res.expect("nope");
        let (_, exec) = parse_executor(FULL_PROG_STR).expect("can't parse program");
        assert_eq!(exec.memory, memvec);
        eprintln!()
    }
}
