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
    // TODO: cast a skill and damage other champion
    fn cast_skill(&mut self, slot: SkillSlot, target: &mut Champion) -> Result<(), SkillError> {
        let skill = self
            .skills
            .iter()
            .find(|s| s.slot == slot)
            .expect("You're not playing Hwei 👀");
        if self.mana < skill.required_mana {
            Err(SkillError::OutOfMana(String::from("You have no mana")))
        } else {
            self.mana -= skill.required_mana;
            target.hp -= skill.damage;
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
    let mut lucian: Champion = Champion::new(
        String::from("Lucian"),
        2341,
        1051,
        18,
        102,
        0,
        [
            Skill {
                slot: SkillSlot::Q,
                name: String::from("Piercing Light"),
                damage: 220,
                required_mana: 80,
            },
            Skill {
                slot: SkillSlot::W,
                name: String::from("Ardent Blaze"),
                damage: 215,
                required_mana: 60,
            },
            Skill {
                slot: SkillSlot::E,
                name: String::from("Relentless Pursuit"),
                damage: 0,
                required_mana: 0,
            },
            Skill {
                slot: SkillSlot::R,
                name: String::from("The Culling"),
                damage: 1000,
                required_mana: 100,
            },
        ],
    );

    println!("{}", ezreal);
    println!("{}", lucian);
    println!("{:?}", ezreal.cast_skill(SkillSlot::Q, &mut lucian));
    println!("{}", lucian);
}
