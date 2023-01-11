 
use crate::file_utils::file_to_lines;
use std::path::Path;

#[derive(Debug, Clone, Default)]
enum Operand {
    #[default]
    VariableOld,
    Constant(u64)
}
impl Operand {
    fn parse(token: &str) -> Operand {
        match token {
            "old" => Operand::VariableOld,
            s => Operand::Constant(s.parse::<u64>().unwrap())
        }
    }
    fn get(&self, old: u64) -> u64 {
        match self {
            Operand::VariableOld => old,
            Operand::Constant(c) => *c
        }
    }
}

#[derive(Debug, Clone, Default)]
enum Operator {
    #[default]
    Addition,
    Multiplication
}
impl Operator {
    fn parse(token: &str) -> Operator {
        match token {
            "+" => Operator::Addition,
            "*" => Operator::Multiplication,
            s => panic!("Unknown operator {}", s)
        }
    }
}


#[derive(Debug, Clone, Default)]
struct Operation {
    op1: Operand,
    op: Operator,
    op2: Operand,
}
impl Operation {
    fn parse(line: &str) -> Operation {
        let tokens: Vec <&str> = line.split_ascii_whitespace().collect();
        Operation {
            op1: Operand::parse(tokens[3]),
            op: Operator::parse(tokens[4]),
            op2: Operand::parse(tokens[5])
        }
    }
    fn compute(&self, old: u64) -> u64 {
        match self.op {
            Operator::Addition => self.op1.get(old) + self.op2.get(old),
            Operator::Multiplication => self.op1.get(old) * self.op2.get(old)
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    id: u8,
    items: Vec<u64>,
    operation: Operation,
    divisor_test: u64,
    business: u64,
    monkey_success: u8,
    monkey_fail: u8
}
impl Monkey {
    fn reset(&mut self) {
        self.items.clear();
        self.business = 0;
    }
    fn turn(&mut self, m_suc: &mut Monkey, m_fail: &mut Monkey, part2: bool) {
        for item in self.items.iter() {
            let mut new_item = self.operation.compute(*item);
            if !part2 {
                new_item /= 3;
            } else {
                /* We can keep value mod (2*3*5*7*11*13*17*19) since they are all prime,
                 * it won't change the divisor test for any monkey */
                new_item = new_item % 9699690;
            }
            if new_item % self.divisor_test == 0 {
                m_suc.items.push(new_item)
            } else {
                m_fail.items.push(new_item)
            }
            self.business += 1;
        }
        self.items.clear();
    }
}

pub fn day11(filename: &Path, part2: bool)
{
    let lines = file_to_lines(filename);
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut m: Monkey = Default::default();

    for l in lines {
        let tokens: Vec<&str> = l.split_ascii_whitespace().collect();
        if tokens.len() < 2 {
            continue
        }
        match tokens[0] {
            "Monkey" => m.id = tokens[1].strip_suffix(":").unwrap().parse::<u8>().unwrap(),
            "Starting" => for t in &tokens[2..] {
                            match t.strip_suffix(",") {
                                Some(t2) => m.items.push(t2.parse::<u64>().unwrap()),
                                None =>     m.items.push(t.parse::<u64>().unwrap())
                            }
                          }
            "Operation:" => m.operation = Operation::parse(&l),
            "Test:" => m.divisor_test = tokens[3].parse::<u64>().unwrap(),
            "If" => if tokens[1] == "true:" {
                        m.monkey_success = tokens[5].parse::<u8>().unwrap()
                    } else {
                        m.monkey_fail = tokens[5].parse::<u8>().unwrap();
                        /* End of parsing */
                        monkeys.push(m.clone());
                        m.reset();
                    }
            _ => {}
        }
    }
    //println!("{:?}", monkeys);
    let n_rounds = match part2 {
                    false => 20,
                    true => 10000
                   };
    for _r in 0..n_rounds {
        for i in 0..monkeys.len() {
            let (suc_id, fail_id) = (monkeys[i].monkey_success as usize, monkeys[i].monkey_fail as usize);
            let [m, m_suc, m_fail] = monkeys.get_many_mut([i, suc_id, fail_id]).unwrap();
            m.turn(m_suc, m_fail, part2);
        }
        //println!("After round {}, state is : {:?}", r+1, monkeys);
    }
    let mut b_vec: Vec<u64> = monkeys.iter().map(|m| m.business).collect();
    let business: &mut[u64] = b_vec.as_mut_slice();
    business.sort();
    business.reverse();
    println!("Monkey business is {}", business[0] * business[1]);
}
