#[derive(Clone, Copy, Debug)]
enum ParameterType {
    Position,
    Immediate,
    Relative,
}

impl ParameterType {
    pub fn from_int(instruction: u32) -> [ParameterType; 3] {
        let mut val = instruction.clone();
        let mut i = 2;
        let mut types: [ParameterType; 3] = [ParameterType::Position; 3];
        while i >= 0 && instruction > 0 {
            let temp = val % 10;
            match temp {
                0 => types[(2 - i) as usize] = ParameterType::Position,
                1 => types[(2 - i) as usize] = ParameterType::Immediate,
                2 => types[(2 - i) as usize] = ParameterType::Relative,
                _ => panic!("invalid parameter mode"),
            }
            val /= 10;
            i -= 1;
        }
        types
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
}

impl Operation {
    pub fn from_int(opcode: u32) -> Self {
        match opcode {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThan,
            8 => Operation::Equals,
            9 => Operation::AdjustRelativeBase,
            _ => panic!("invalid opcode"),
        }
    }
}

#[derive(Debug)]
struct Opcode {
    parameter_modes: [ParameterType; 3],
    opcode: Operation,
    variables: u32,
}

impl Opcode {
    pub fn from_int(instruction: u32) -> Self {
        let opcode = Operation::from_int((instruction % 100) as u32);
        let variables = match opcode {
            Operation::Add | Operation::Multiply | Operation::LessThan | Operation::Equals => 3,
            Operation::JumpIfTrue | Operation::JumpIfFalse => 2,
            Operation::Input | Operation::Output | Operation::AdjustRelativeBase => 1,
        };
        let parameter_modes = ParameterType::from_int(instruction / 100);
        Self {
            parameter_modes,
            opcode,
            variables,
        }
    }

    pub fn get_params(
        &self,
        intcode: &Vec<i64>,
        ipointer: &usize,
        base: &i64,
    ) -> Vec<(usize, i64)> {
        let mut vars: Vec<(usize, i64)> = vec![];
        for i in 0..self.variables as usize {
            match self.parameter_modes[i] {
                ParameterType::Position => {
                    let idx = intcode[ipointer + i + 1];
                    if idx < 0 {
                        panic!("attempted to access negative index");
                    }
                    if idx >= intcode.len() as i64 {
                        vars.push((idx as usize, 0));
                    } else {
                        vars.push((idx as usize, intcode[idx as usize]));
                    }
                }
                ParameterType::Immediate => {
                    vars.push((ipointer + i + 1, intcode[ipointer + i + 1]))
                }
                ParameterType::Relative => {
                    let idx = intcode[ipointer + i + 1] + base;
                    if idx < 0 {
                        panic!("attempted to access negative index");
                    }
                    if idx >= intcode.len() as i64 {
                        vars.push((idx as usize, 0));
                    } else {
                        vars.push((idx as usize, intcode[idx as usize]));
                    }
                }
            };
        }
        vars
    }
}

struct Program {
    intcode: Vec<i64>,
    ipointer: usize,
    inputs: Vec<i32>,
    relative_base: i64,
}

impl Program {
    pub fn new(intcode: Vec<i64>, inputs: Vec<i32>) -> Self {
        Self {
            intcode,
            ipointer: 0,
            relative_base: 0,
            inputs,
        }
    }

    pub fn get_inputs(&mut self) -> &mut Vec<i32> {
        &mut self.inputs
    }

    fn fill_empty(&mut self, idx: &usize) {
        for _ in self.intcode.len()..=*idx {
            self.intcode.push(0);
        }
    }

    pub fn execute(&mut self) -> Vec<i64> {
        let mut out: Vec<i64> = vec![];
        while self.intcode[self.ipointer] != 99 {
            let opcode = Opcode::from_int(self.intcode[self.ipointer] as u32);
            match opcode.opcode {
                Operation::Add => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.fill_empty(&params[2].0);
                    self.intcode[params[2].0] = params[0].1 + params[1].1
                }
                Operation::Multiply => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.fill_empty(&params[2].0);
                    self.intcode[params[2].0] = params[0].1 * params[1].1;
                }
                Operation::Input => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.intcode[params[0].0] = self.inputs.remove(0) as i64;
                }
                Operation::Output => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    out.push(params[0].1);
                }
                Operation::JumpIfTrue => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    if params[0].1 != 0 {
                        self.ipointer = params[1].1 as usize;
                        continue;
                    }
                }
                Operation::JumpIfFalse => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    if params[0].1 == 0 {
                        self.ipointer = params[1].1 as usize;
                        continue;
                    }
                }
                Operation::LessThan => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    let val = if params[0].1 < params[1].1 { 1 } else { 0 };
                    self.fill_empty(&params[2].0);
                    self.intcode[params[2].0] = val;
                }
                Operation::Equals => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    let val = if params[0].1 == params[1].1 { 1 } else { 0 };
                    self.fill_empty(&params[2].0);
                    self.intcode[params[2].0] = val;
                }
                Operation::AdjustRelativeBase => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.relative_base += params[0].1;
                }
            }

            self.ipointer += opcode.variables as usize + 1;
        }
        out
    }
}

pub fn part2(path: &str) -> i64 {
    let intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut program = Program::new(intcode, vec![2]);
    let out = program.execute();
    *out.last().unwrap()
}
