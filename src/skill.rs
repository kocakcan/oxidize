use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SkillSlot {
    Q,
    W,
    E,
    R,
}

#[derive(Debug)]
struct Skill {
    slot: SkillSlot,
    name: String,
    damage: u32,
    required_mana: u32,
}

#[derive(Debug)]
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
    fn cast_skill(&mut self, slot: SkillSlot) -> Result<(), SkillError> {
        let skill = self
            .skills
            .iter()
            .find(|s| s.slot == slot)
            .expect("You're not playing Hwei 👀");
        if self.mana < skill.required_mana {
            Err(SkillError::OutOfMana(String::from("You have no mana")))
        } else {
            self.mana -= skill.required_mana;
            Ok(())
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
            Skill {
                slot: SkillSlot::Q,
                name: String::from("Mystic Shot"),
                damage: 300,
                required_mana: 28,
            },
            Skill {
                slot: SkillSlot::W,
                name: String::from("Essence Flux"),
                damage: 200,
                required_mana: 50,
            },
            Skill {
                slot: SkillSlot::E,
                name: String::from("Arcane Shift"),
                damage: 140,
                required_mana: 70,
            },
            Skill {
                slot: SkillSlot::R,
                name: String::from("Trueshot Barrage"),
                damage: 600,
                required_mana: 100,
            },
        ],
    );
    println!("{}", ezreal);
    println!("{:?}", ezreal.cast_skill(SkillSlot::Q));
}
