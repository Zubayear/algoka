pub struct Character<T: WeaponBehavior> {
    pub weapon: T,
}

impl<T: WeaponBehavior> Character<T> {
    pub fn new(weapon: T) -> Self {
        Self { weapon }
    }

    pub fn fight(&self) -> String {
        self.weapon.use_weapon()
    }
}

pub struct SwordBehavior;

pub struct BowAndArrowBehavior;

pub struct KnifeBehavior;

pub struct AxeBehavior;

pub trait WeaponBehavior {
    fn use_weapon(&self) -> String;
}

impl WeaponBehavior for BowAndArrowBehavior {
    fn use_weapon(&self) -> String {
        "use bow and arrow".to_string()
    }
}

impl WeaponBehavior for KnifeBehavior {
    fn use_weapon(&self) -> String {
        "use knife".to_string()
    }
}

impl WeaponBehavior for AxeBehavior {
    fn use_weapon(&self) -> String {
        "user axe".to_string()
    }
}

impl WeaponBehavior for SwordBehavior {
    fn use_weapon(&self) -> String {
        "use sword".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knife_behavior() {
        let character = Character::new(KnifeBehavior);
        assert_eq!(character.fight(), "use knife");
    }
}
