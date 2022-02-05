extern crate execute;

use text_io::read;

use rand::prelude::*;
use rand_derive2::RandGen;
#[derive(RandGen)]

enum Instruction {
    WORD,
    ADD,
    SUB,
    MULT,
    MULTU,
    DIV,
    DIVU,
    MFHI,
    MFLO,
    LIS,
    LW,
    SW,
    SLT,
    SLTU,
    BEQ,
    BNE,
    JR,
    JALR,
}

impl Instruction {
    fn get_next() -> Instruction {
        Instruction::generate_random()
    }
    fn get_string(&self, labels: &Vec<String>, symTable: &Vec<String>) -> String {
        let mut ret: String = "".to_string();
        for label in labels {
            ret.push_str(format!("{}: ", label).as_str());
        }
        enum Intermedian {
            S(String),
            I(i32),
        }
        let intermedian = if symTable.len() <= 1 {
            Intermedian::I(thread_rng().gen_range(-32768..32767))
        } else if thread_rng().gen_range(0..3) == 0 {
            Intermedian::I(thread_rng().gen_range(-32768..32767))
        } else {
            Intermedian::S(
                symTable
                    .get(thread_rng().gen_range(0..symTable.len()))
                    .unwrap()
                    .clone(),
            )
        };
        match self {
            Instruction::WORD => {
                format!(
                    "{}.word {}\n",
                    ret,
                    thread_rng().gen_range(-2147483648..2147483647)
                )
            }
            Instruction::ADD => format!(
                "{}add ${}, ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31)
            ),
            Instruction::SUB => format!(
                "{}sub ${}, ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31)
            ),
            Instruction::MULT => format!(
                "{}mult ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
            ),
            Instruction::MULTU => format!(
                "{}multu ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
            ),
            Instruction::DIV => format!(
                "{}div ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
            ),
            Instruction::DIVU => format!(
                "{}divu ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
            ),
            Instruction::MFHI => format!("{}mfhi ${}\n", ret, thread_rng().gen_range(0..31),),
            Instruction::MFLO => format!("{}mflo ${}\n", ret, thread_rng().gen_range(0..31),),
            Instruction::LIS => format!("{}lis ${}\n", ret, thread_rng().gen_range(0..31),),
            Instruction::LW => format!(
                "{}lw ${}, {}(${})\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(-32768..32767),
                thread_rng().gen_range(0..31),
            ),
            Instruction::SW => format!(
                "{}sw ${}, {}(${})\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(-32768..32767),
                thread_rng().gen_range(0..31),
            ),
            Instruction::SLT => format!(
                "{}slt ${}, ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31)
            ),
            Instruction::SLTU => format!(
                "{}sltu ${}, ${}, ${}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31)
            ),
            Instruction::BEQ => format!(
                "{}beq ${}, ${}, {}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                match intermedian {
                    Intermedian::S(s) => s,
                    Intermedian::I(i) => i.to_string(),
                },
            ),
            Instruction::BNE => format!(
                "{}bne ${}, ${}, {}\n",
                ret,
                thread_rng().gen_range(0..31),
                thread_rng().gen_range(0..31),
                match intermedian {
                    Intermedian::S(s) => s,
                    Intermedian::I(i) => i.to_string(),
                },
            ),
            Instruction::JR => format!("{}jr ${}\n", ret, thread_rng().gen_range(0..31),),
            Instruction::JALR => format!("{}jalr ${}\n", ret, thread_rng().gen_range(0..31),),
        }
    }
}

fn getRandomLabel() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
    const PASSWORD_LEN: usize = 5;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return password;
}
fn main() {
    let num_lines: i32 = read!();
    let mut lables: Vec<String> = vec![];
    let mut sym_table: Vec<String> = vec![];

    // for _ in 0..100 {
    //     sym_table.push(getRandomLabel());
    // }

    for _ in 0..num_lines {
        lables = vec![];
        // if a line should have label
        if thread_rng().gen_range(0..5) == 0 {
            let num_labels = thread_rng().gen_range(1..10);
            let mut s = "".to_string();
            for _ in 0..num_labels {
                s = getRandomLabel();
                lables.push(s.clone());
            }
            // if the line is null
            if thread_rng().gen_range(0..3) == 0 {
                let mut ret: String = "".to_string();
                for label in &lables {
                    ret.push_str(format!("{}: ", label.clone()).as_str());
                }
                println!("{}", ret);
                continue;
            }
            sym_table.push(s.clone());
        }
        print!(
            "{}",
            Instruction::get_next().get_string(&lables, &sym_table)
        );
    }
}
