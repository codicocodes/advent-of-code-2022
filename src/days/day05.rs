pub fn solve(input: &str){
    let (matrix_input, instruction_input) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(matrix_input);
    let instructions = parse_many_instructions(instruction_input);

    for instruction in instructions {
        for _ in 0..instruction.count {
            let stack_crate = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(stack_crate)
        }
    }
    debug_print_stacks(stacks.clone());
}

#[derive(Debug)]
struct Instruction {
    text: String,
    count: u32,
    from: usize,
    to: usize,
}

fn parse_instruction(instruction_input: &str) -> Instruction {
    let parsed = instruction_input.trim().split(" ").collect::<Vec<_>>();
    return Instruction{
        text: instruction_input.to_string(),
        count: parsed[1].parse().unwrap(),
        from: parsed[3].parse().unwrap(),
        to: parsed[5].parse().unwrap(),
    }
}

fn parse_many_instructions(instruction_input: &str) -> Vec<Instruction> {
    let parsed = instruction_input.trim().split("\n").collect::<Vec<_>>();
    let mut instructions: Vec<Instruction> = vec![];
    for instruction_input in parsed {
        instructions.push(parse_instruction(instruction_input))
    }
    return instructions;
}

fn debug_print_stacks(stacks: Vec<Vec<char>>) {
    println!("- - - - -");
    for stack in stacks.clone() {
        for c in stack {
            print!("{c}");
        }
        println!("");
    }
    println!("- - - - -");
}

fn parse_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    let mut lines = stacks_input.split("\n").collect::<Vec<_>>();
    lines.reverse();
    let mut stacks: Vec<Vec<char>> = vec![];
    for idx in 0..lines.clone()[0].len() {
        let mut stack: Vec<char> = vec![];
        for line in lines.clone() {
            let c: char = line.as_bytes()[idx] as char;
            if !c.is_numeric() && c != ' ' {
                stack.push(c)
            }
        }
        if stack.len() > 0 && stack[0].is_alphabetic() {
            stacks.push(stack)
        }
    }
    debug_print_stacks(stacks.clone());
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = "move 1 from 2 to 3";
        let instruction = parse_instruction(input);
        assert_eq!(instruction.count, 1);
        assert_eq!(instruction.from, 2);
        assert_eq!(instruction.to, 3);
    }

    #[test]
    fn test_parse_matrix() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";
          let _stacks = parse_stacks(input);
    }
}
