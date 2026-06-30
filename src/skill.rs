use std::fmt;

#[derive(Debug)]
enum Skill {
    Q {
        name: String,
        damage: u32,
        required_mana: u32,
    },
    W {
        name: String,
        damage: u32,
        required_mana: u32,
    },
    E {
        name: String,
        damage: u32,
        required_mana: u32,
    },
    R {
        name: String,
        damage: u32,
        required_mana: u32,
    },
}

enum SkillError {
    OutOfMana(String),
    // SkillNotLearned(String),
}

struct Champion {
    name: String,
    hp: u32,
    mana: u32,
    level: u8,
    ad: u32,
    ap: u32,
    skills: [Skill; 4],
}

impl fmt::Display for Champion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {} | HP: {} | Mana: {} | Level: {} | AD: {} | AP: {} | Skills: {:?}",
            self.name, self.hp, self.mana, self.level, self.ad, self.ap, self.skills
        )
    }
}

impl Champion {
    fn new(
        name: String,
        hp: u32,
        mana: u32,
        level: u8,
        ad: u32,
        ap: u32,
        skills: [Skill; 4],
    ) -> Self {
        Self {
            name,
            hp,
            mana,
            level,
            ad,
            ap,
            skills,
        }
    }
    fn cast_skill(&mut self, skill: Skill) -> Result<(), SkillError> {
        match skill {
            Skill::Q {
                required_mana: mana,
                ..
            }
            | Skill::W {
                required_mana: mana,
                ..
            }
            | Skill::E {
                required_mana: mana,
                ..
            }
            | Skill::R {
                required_mana: mana,
                ..
            } => {
                if self.mana < mana {
                    Err(SkillError::OutOfMana(String::from("You have no mana")))
                } else {
                    self.mana -= mana;
                    Ok(())
                }
            }
        }
    }
}

fn main() {
    let mut ezreal: Champion = Champion::new(
        String::from("Ezreal"),
        2334,
        1565,
        18,
        124,
        0,
        [
            Skill::Q {
                name: String::from("Mystic Shot"),
                damage: 300,
                required_mana: 28,
            },
            Skill::W {
                name: String::from("Essence Flux"),
                damage: 200,
                required_mana: 50,
            },
            Skill::E {
                name: String::from("Arcane Shift"),
                damage: 140,
                required_mana: 70,
            },
            Skill::R {
                name: String::from("Trueshot Barrage"),
                damage: 600,
                required_mana: 100,
            },
        ],
    );
    println!("{}", ezreal);
    ezreal.mana = 25;
    let result = ezreal.cast_skill(Skill::Q {
        name: String::from("Mystic Shot"),
        damage: 300,
        required_mana: 28,
    });
    match result {
        Ok(()) => println!("{} deals some damage", ezreal.name),
        Err(SkillError::OutOfMana(e)) => println!("Error: {e}"),
    }
}
