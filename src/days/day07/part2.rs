#[derive(Clone, Copy, Debug)]
enum ParameterType {
    Position,
    Immediate,
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
            Operation::Input | Operation::Output => 1,
        };
        let parameter_modes = ParameterType::from_int(instruction / 100);
        Self {
            parameter_modes,
            opcode,
            variables,
        }
    }

    pub fn get_params(&self, intcode: &Vec<i32>, ipointer: &usize) -> Vec<i32> {
        let mut vars: Vec<i32> = vec![];
        for i in 0..self.variables as usize {
            match self.parameter_modes[i] {
                ParameterType::Position => vars.push(intcode[intcode[ipointer + i + 1] as usize]),
                ParameterType::Immediate => vars.push(intcode[ipointer + i + 1]),
            };
        }
        vars
    }
}

struct Program {
    intcode: Vec<i32>,
    ipointer: usize,
    inputs: Vec<i32>,
    prev_output: Option<i32>,
    last_output: Option<i32>,
}

impl Program {
    pub fn new(intcode: Vec<i32>, inputs: Vec<i32>) -> Self {
        Self {
            intcode,
            ipointer: 0,
            inputs,
            prev_output: None,
            last_output: None,
        }
    }

    pub fn get_inputs(&mut self) -> &mut Vec<i32> {
        &mut self.inputs
    }

    pub fn execute(&mut self) -> i32 {
        while self.intcode[self.ipointer] != 99 {
            let opcode = Opcode::from_int(self.intcode[self.ipointer] as u32);
            match opcode.opcode {
                Operation::Add => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    let idx = self.intcode[self.ipointer + 3] as usize;
                    self.intcode[idx] = params[0] + params[1];
                }
                Operation::Multiply => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    let idx = self.intcode[self.ipointer + 3] as usize;
                    self.intcode[idx] = params[0] * params[1];
                }
                Operation::Input => {
                    let idx = self.intcode[self.ipointer + 1] as usize;
                    self.intcode[idx] = self.inputs.remove(0);
                }
                Operation::Output => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    self.prev_output = Some(params[0]);
                    self.ipointer += opcode.variables as usize + 1;
                    return params[0];
                }
                Operation::JumpIfTrue => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    if params[0] != 0 {
                        self.ipointer = params[1] as usize;
                        continue;
                    }
                }
                Operation::JumpIfFalse => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    if params[0] == 0 {
                        self.ipointer = params[1] as usize;
                        continue;
                    }
                }
                Operation::LessThan => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    let val = if params[0] < params[1] { 1 } else { 0 };
                    let idx = self.intcode[self.ipointer + 3] as usize;
                    self.intcode[idx] = val;
                }
                Operation::Equals => {
                    let params = opcode.get_params(&self.intcode, &self.ipointer);
                    let val = if params[0] == params[1] { 1 } else { 0 };
                    let idx = self.intcode[self.ipointer + 3] as usize;
                    self.intcode[idx] = val;
                }
            }

            self.ipointer += opcode.variables as usize + 1;
        }
        self.last_output = self.prev_output;
        self.last_output.unwrap()
    }
}

pub fn part2(path: &str) -> i32 {
    let intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<i32>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let permutations = permutations(&vec![5, 6, 7, 8, 9], &0);

    let mut max = 0;
    for permutation in permutations {
        let mut programs: Vec<Program> = vec![];
        for i in 0..5 {
            let program = Program::new(intcode.clone(), vec![permutation[i]]);
            programs.push(program);
        }

        let mut i = 0;
        programs[0].get_inputs().push(0);
        while let None = programs[4].last_output {
            let temp = programs[i % 5].execute();
            programs[(i + 1) % 5].get_inputs().push(temp);
            i += 1;
        }
        let out = programs[4].last_output.unwrap();
        if out > max {
            max = out;
        }
    }
    max
}

pub fn permutations(obj: &Vec<i32>, idx: &usize) -> Vec<Vec<i32>> {
    if *idx == obj.len() - 1 {
        return vec![obj.to_vec()];
    }
    let mut obj = obj.clone();
    let mut results: Vec<Vec<i32>> = vec![];
    let mut i = idx.clone();
    while i < obj.len() {
        obj.swap(*idx, i);
        results.append(&mut permutations(&obj, &(*idx + 1)));
        obj.swap(*idx, i);
        i += 1;
    }
    results
}
