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

    pub fn read_ascii_outputs(&mut self) -> String {
        let outputs = self.read_all_outputs();
        outputs
            .iter()
            .map(|v| (*v as u8 as char).to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn read_all_outputs(&mut self) -> Vec<i64> {
        let outputs = self.pending_outputs.clone();
        self.pending_outputs.clear();
        outputs
    }

    pub fn read_outputs(&mut self, max: usize) -> Vec<i64> {
        let max = std::cmp::min(max, self.pending_outputs.len());
        let outputs = self.pending_outputs[0..max].to_vec();
        self.pending_outputs = self.pending_outputs[max..].to_vec();
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

const ITEMS: [&str; 8] = [
    "prime number",
    "spool of cat6",
    "mug",
    "asterisk",
    "monolith",
    "sand",
    "tambourine",
    "festive hat",
];

fn drop_all() -> Vec<i64> {
    ITEMS
        .iter()
        .flat_map(|s| {
            let drop = "drop ".to_string() + s + "\n";
            drop.chars()
                .map(|c| c.to_ascii_lowercase() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn take(idx: usize) -> Vec<i64> {
    let t = "take ".to_string() + ITEMS[idx] + "\n";
    t.chars()
        .map(|c| c.to_ascii_lowercase() as i64)
        .collect::<Vec<_>>()
}

fn tests() -> Vec<i64> {
    let mut test_commands = vec![];
    for i in 0..0b11111111 {
        for j in 0..8 {
            if i & (1 << j) != 0 {
                test_commands.append(&mut take(j));
            }
        }
        test_commands.append(&mut vec![119, 101, 115, 116, 10]);
        test_commands.append(&mut drop_all());
    }
    test_commands
}

pub fn part1(path: &str) -> String {
    let intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let commands = [
        "east\n",
        "north\n",
        "north\n",
        "take spool of cat6\n",
        "south\n",
        "east\n",
        "take mug\n",
        "north\n",
        "north\n",
        "west\n",
        "take asterisk\n",
        "south\n",
        "take monolith\n",
        "north\n",
        "east\n",
        "south\n",
        "east\n",
        "take sand\n",
        "south\n",
        "west\n",
        "take prime number\n",
        "east\n",
        "north\n",
        "east\n",
        "south\n",
        "take tambourine\n",
        "west\n",
        "take festive hat\n",
        "north\n",
    ];

    // prime number, asterisk, sand, tambourine

    let mut commands = commands
        .iter()
        .flat_map(|s| {
            s.chars()
                .map(|c| c.to_ascii_lowercase() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    commands.append(&mut drop_all());
    commands.append(&mut tests());
    let mut robot = Program::new(intcode.clone(), commands);
    loop {
        robot.run();
        if *robot.status() == Status::Halted {
            return robot
                .read_ascii_outputs()
                .split("\n\n")
                .last()
                .unwrap()
                .trim()
                .to_string();
        }
        if *robot.status() == Status::WaitingForInput {
            println!("{}", robot.read_ascii_outputs());
            println!("Enter command: ");
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let mut ascii = input
                        .chars()
                        .filter_map(|c| {
                            if c != '\r' {
                                return Some(c.to_ascii_lowercase() as i64);
                            }
                            None
                        })
                        .collect::<Vec<_>>();
                    robot.get_pending_inputs().append(&mut ascii);
                }
                _ => continue,
            }
        }
    }
}
