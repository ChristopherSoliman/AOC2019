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
}

impl Operation {
    pub fn from_int(opcode: u32) -> Self {
        match opcode {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Input,
            4 => Operation::Output,
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
            Operation::Add | Operation::Multiply => 3,
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

pub fn part1(path: &str) -> i32 {
    let mut intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<i32>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let input = 1;
    let mut out: Vec<i32> = vec![];
    let mut ipointer = 0;

    while intcode[ipointer] != 99 {
        let opcode = Opcode::from_int(intcode[ipointer] as u32);
        match opcode.opcode {
            Operation::Add => {
                let params = opcode.get_params(&intcode, &ipointer);
                let idx = intcode[ipointer + 3] as usize;
                intcode[idx] = params[0] + params[1];
            }
            Operation::Multiply => {
                let params = opcode.get_params(&intcode, &ipointer);
                let idx = intcode[ipointer + 3] as usize;
                intcode[idx] = params[0] * params[1];
            }
            Operation::Input => {
                let idx = intcode[ipointer + 1] as usize;
                intcode[idx] = input;
            }
            Operation::Output => {
                let params = opcode.get_params(&intcode, &ipointer);
                out.push(params[0]);
            }
        }

        ipointer += opcode.variables as usize + 1;
    }

    *out.last().unwrap()
}
