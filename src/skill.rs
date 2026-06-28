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
    fn cast_skill(&self, skill: Skill) -> Result<(), SkillError> {
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
                    Ok(println!("{} deals some damage!", self.name))
                }
            }
        }
    }
}

fn main() {
    let ezreal: Champion = Champion {
        name: String::from("Ezreal"),
        hp: 2334,
        mana: 1565,
        level: 18,
        ad: 124,
        ap: 0,
        skills: [
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
    };
    println!("{}", ezreal);
    ezreal.cast_skill(Skill::Q {
        name: String::from("Mystic Shot"),
        damage: 300,
        required_mana: 28,
    });
}
