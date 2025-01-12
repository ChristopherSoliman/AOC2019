#[derive(Clone, Copy, Debug)]
struct Parameter {
    index: usize,
    value: i64,
}

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

    pub fn get_params(&self, intcode: &Vec<i64>, ipointer: &usize, base: &i64) -> Vec<Parameter> {
        let mut vars: Vec<Parameter> = vec![];
        for i in 0..self.variables as usize {
            match self.parameter_modes[i] {
                ParameterType::Position => {
                    let idx = intcode[ipointer + i + 1];
                    if idx < 0 {
                        panic!("attempted to access negative index");
                    }
                    if idx >= intcode.len() as i64 {
                        vars.push(Parameter {
                            index: idx as usize,
                            value: 0,
                        });
                    } else {
                        vars.push(Parameter {
                            index: idx as usize,
                            value: intcode[idx as usize],
                        });
                    }
                }
                ParameterType::Immediate => {
                    vars.push(Parameter {
                        index: ipointer + i + 1,
                        value: intcode[ipointer + i + 1],
                    });
                }
                ParameterType::Relative => {
                    let idx = intcode[ipointer + i + 1] + base;
                    if idx < 0 {
                        panic!("attempted to access negative index");
                    }
                    if idx >= intcode.len() as i64 {
                        vars.push(Parameter {
                            index: idx as usize,
                            value: 0,
                        });
                    } else {
                        vars.push(Parameter {
                            index: idx as usize,
                            value: intcode[idx as usize],
                        });
                    }
                }
            };
        }
        vars
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Idle,
    Running,
    WaitingForInput,
    Halted,
}

struct Program {
    intcode: Vec<i64>,
    status: Status,
    ipointer: usize,
    pending_inputs: Vec<i64>,
    pending_outputs: Vec<i64>,
    relative_base: i64,
}

impl Program {
    pub fn new(intcode: Vec<i64>, pending_inputs: Vec<i64>) -> Self {
        Self {
            intcode,
            ipointer: 0,
            relative_base: 0,
            status: Status::Idle,
            pending_inputs,
            pending_outputs: vec![],
        }
    }

    pub fn get_pending_inputs(&mut self) -> &mut Vec<i64> {
        &mut self.pending_inputs
    }

    pub fn read_all_outputs(&mut self) -> Vec<i64> {
        let outputs = self.pending_outputs.clone();
        self.pending_outputs.clear();
        outputs
    }

    pub fn read_outputs(&mut self, max: usize) -> Vec<i64> {
        let mut outputs = vec![];
        let mut i = 0;
        while i < max && i < outputs.len() {
            outputs.push(self.pending_outputs.remove(0));
        }
        outputs
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    fn fill_empty(&mut self, idx: &usize) {
        for _ in self.intcode.len()..=*idx {
            self.intcode.push(0);
        }
    }

    pub fn run(&mut self) {
        self.status = Status::Running;
        while self.intcode[self.ipointer] != 99 {
            let opcode = Opcode::from_int(self.intcode[self.ipointer] as u32);
            match opcode.opcode {
                Operation::Add => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.fill_empty(&params[2].index);
                    self.intcode[params[2].index] = params[0].value + params[1].value
                }
                Operation::Multiply => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.fill_empty(&params[2].index);
                    self.intcode[params[2].index] = params[0].value * params[1].value;
                }
                Operation::Input => {
                    if self.pending_inputs.len() == 0 {
                        self.status = Status::WaitingForInput;
                        return;
                    }
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.fill_empty(&params[0].index);
                    self.intcode[params[0].index] = self.pending_inputs.remove(0) as i64;
                }
                Operation::Output => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    println!("output: {}", params[0].value);
                    self.pending_outputs.push(params[0].value);
                }
                Operation::JumpIfTrue => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    if params[0].value != 0 {
                        self.ipointer = params[1].value as usize;
                        continue;
                    }
                }
                Operation::JumpIfFalse => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    if params[0].value == 0 {
                        self.ipointer = params[1].value as usize;
                        continue;
                    }
                }
                Operation::LessThan => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    let val = if params[0].value < params[1].value {
                        1
                    } else {
                        0
                    };
                    self.fill_empty(&params[2].index);
                    self.intcode[params[2].index] = val;
                }
                Operation::Equals => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    let val = if params[0].value == params[1].value {
                        1
                    } else {
                        0
                    };
                    self.fill_empty(&params[2].index);
                    self.intcode[params[2].index] = val;
                }
                Operation::AdjustRelativeBase => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.relative_base += params[0].value;
                }
            }
            self.ipointer += opcode.variables as usize + 1;
        }
        self.status = Status::Halted;
    }
}

const N: usize = 50;

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

    let mut nics = vec![];
    for i in 0..N {
        nics.push(Program::new(intcode.clone(), vec![i as i64]));
    }

    loop {
        for i in 0..N {
            let nic = &mut nics[i];
            if *nic.status() == Status::Halted {
                continue;
            }
            if *nic.status() == Status::WaitingForInput {
                nic.get_pending_inputs().push(-1);
            }
            //println!("{i} => in: {:?}", nic.get_pending_inputs());
            nic.run();
            let outputs = nic.read_all_outputs();
            //println!("{i} => out: {:?}", outputs);
            //println!("{i} => status: {:?}", nic.status());
            let mut k = 0;
            while k + 3 < outputs.len() {
                if outputs[k] == 255 {
                    return outputs[k + 2];
                }
                nics[outputs[k] as usize]
                    .get_pending_inputs()
                    .push(outputs[k + 1]);
                nics[outputs[k] as usize]
                    .get_pending_inputs()
                    .push(outputs[k + 2]);
                k += 3;
            }
        }
    }
}
