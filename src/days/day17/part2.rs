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

    pub fn execute(&mut self) -> Option<i64> {
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
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.intcode[params[0].index] = self.inputs.remove(0) as i64;
                }
                Operation::Output => {
                    let params =
                        opcode.get_params(&self.intcode, &self.ipointer, &self.relative_base);
                    self.ipointer += opcode.variables as usize + 1;
                    return Some(params[0].value);
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
        None
    }
}

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part2(path: &str) -> u32 {
    let intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (grid, start) = create_grid(intcode.clone());
    let path = get_path(&grid, &start);
    println!("{:?}", path);

    //Movement A,B,A,C,B,A,B,A,A,B
    // A => L,11,L,11,R,5
    // B => R,11,R,7,R,5,R,5
    // C => R,7,L,11,L,11

    let routine = "A,B,A,C,B,A,B,C,C,B\n";
    let a = "L,12,L,12,R,4\n";
    let b = "R,10,R,6,R,4,R,4\n";
    let c = "R,6,L,12,L,12\n";

    let mut intcode = intcode.clone();
    intcode[0] = 2;
    let inputs = vec![routine, a, b, c, "n\n"];
    let mut program = Program::new(intcode, vec![]);
    let mut i = 0;
    let mut out = 0;
    loop {
        if program.get_inputs().is_empty() && i < 5 {
            *program.get_inputs() = inputs[i]
                .chars()
                .map(|c| c.to_ascii_uppercase() as i32)
                .collect::<Vec<_>>();
            i += 1;
        }

        if let Some(c) = program.execute() {
            out = c;
        } else {
            break;
        }
    }
    out as u32
}

fn create_grid(intcode: Vec<i64>) -> (Vec<Vec<bool>>, (u32, u32)) {
    let mut program = Program::new(intcode, vec![]);
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut i = 0;
    let mut j = 0;
    let mut start: (u32, u32) = (0, 0);
    grid.push(vec![]);
    loop {
        if let Some(c) = program.execute() {
            match c {
                10 => {
                    i += 1;
                    j = 0;
                    grid.push(vec![]);
                }
                35 => grid[i].push(true),
                94 => {
                    grid[i].push(true);
                    start = (i as u32, j);
                }
                46 => grid[i].push(false),
                _ => unreachable!("invalid code"),
            }
            j += 1;
        } else {
            break;
        }
    }
    grid.pop();
    grid.pop();

    (grid, start)
}

fn get_path(grid: &Vec<Vec<bool>>, start: &(u32, u32)) -> Vec<(&'static str, u32)> {
    let mut current = *start;
    let mut path: Vec<(&str, u32)> = vec![];
    let mut cd = 0;

    'main_loop: loop {
        let (r, c) = current;
        let (nr, nc) = (r as i32 + DIRS[cd].0 as i32, c as i32 + DIRS[cd].1 as i32);
        if valid(&(nr, nc), &grid) {
            let last = path.pop().unwrap();
            path.push((last.0, last.1 + 1));
            current = (nr as u32, nc as u32);
            continue;
        }

        for i in 0..3 {
            if i == 1 {
                continue;
            }
            let new_dir = (cd as i32 + (i as i32 - 1)).rem_euclid(4) as usize;

            let (nr, nc) = (
                r as i32 + DIRS[new_dir].0 as i32,
                c as i32 + DIRS[new_dir].1 as i32,
            );

            if valid(&(nr, nc), &grid) {
                let mut str_dir = "L";
                if i == 2 {
                    str_dir = "R";
                }
                cd = new_dir;
                path.push((str_dir, 0));
                continue 'main_loop;
            }
        }
        break;
    }

    path
}

fn valid(current: &(i32, i32), grid: &Vec<Vec<bool>>) -> bool {
    let (nr, nc) = *current;
    return 0 <= nr
        && nr < grid.len() as i32
        && 0 <= nc
        && nc < grid[0].len() as i32
        && grid[nr as usize][nc as usize];
}
