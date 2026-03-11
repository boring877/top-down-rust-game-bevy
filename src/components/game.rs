use bevy::prelude::*;

// ============================================================================
// ENTITIES
// ============================================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossLastPosition(pub Vec2);

#[derive(Component)]
pub struct Obstacle;

#[derive(Clone, Copy, PartialEq)]
pub enum ObstacleShape {
    Rock,
    Crystal,
    Pillar,
}

#[derive(Component)]
pub struct Floor;

// ============================================================================
// HEALTH
// ============================================================================

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.current = self.current.saturating_add(amount);
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

// ============================================================================
// STATS AND INVENTORY
// ============================================================================

#[derive(Component, Clone, Copy, Debug)]
pub struct CombatStats {
    pub intelligence: u32,
    pub strength: u32,
    pub agility: u32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub dodge_rate: f32, 
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            intelligence: 10,
            strength: 10,
            agility: 10,
            crit_rate: 0.05,
            crit_damage: 1.5,
            dodge_rate: 0.05,
        }
    }
}

#[derive(Component)]
pub struct PlayerStats {
    pub level: u32,
    pub xp: u32,
    pub max_xp: u32,
    pub gold: u32,
    pub materials: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            xp: 0,
            max_xp: 100, // XP needed for level 2
            gold: 0,
            materials: 0,
        }
    }
}

impl PlayerStats {
    pub fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        while self.xp >= self.max_xp {
            self.xp -= self.max_xp;
            self.level += 1;
            // Increase XP needed for next level (e.g. 50% more)
            self.max_xp = (self.max_xp as f32 * 1.5) as u32;
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    pub fn color(&self) -> Color {
        match self {
            ItemRarity::Common => Color::srgb(0.8, 0.8, 0.8),
            ItemRarity::Uncommon => Color::srgb(0.2, 0.8, 0.2),
            ItemRarity::Rare => Color::srgb(0.2, 0.5, 1.0),
            ItemRarity::Epic => Color::srgb(0.7, 0.2, 0.8),
            ItemRarity::Legendary => Color::srgb(1.0, 0.6, 0.0),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemSet {
    None,
    WarriorSet,
    MageSet,
    RogueSet,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub rarity: ItemRarity,
    pub set: ItemSet,
    pub pickup_type: PickupType,
    
    // Stats
    pub intelligence: u32,
    pub strength: u32,
    pub agility: u32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub dodge_rate: f32,

    pub granted_skill: Option<PlayerSkill>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerSkill {
    BasicAttack,
    SpinBlades,
    HolyBurst,
    Fireball,
    Dash,
    Earthquake,
}

impl PlayerSkill {
    pub fn description(&self) -> &'static str {
        match self {
            PlayerSkill::BasicAttack => "A simple melee attack.",
            PlayerSkill::SpinBlades => "Continuous magical blades that orbit you.",
            PlayerSkill::HolyBurst => "A continuous holy aura that burns nearby enemies.",
            PlayerSkill::Fireball => "Shoots a fireball towards the closest enemy.",
            PlayerSkill::Dash => "Quickly dashes in the direction of movement.",
            PlayerSkill::Earthquake => "Slams the ground creating shockwaves.",
        }
    }

    pub fn damage_info(&self) -> &'static str {
        match self {
            PlayerSkill::BasicAttack => "Damage: 10 + (100% STR)\nType: Physical\nCooldown: 0.2s",
            PlayerSkill::SpinBlades => "Damage: 5 + (100% INT)\nType: Magic\nCooldown: None (Always Active)",
            PlayerSkill::HolyBurst => "Damage: 10 + (150% INT)\nType: Holy/Magic\nCooldown: None (Continuous Aura)",
            PlayerSkill::Fireball => "Damage: 20 + (200% INT)\nType: Fire/Magic\nCooldown: None (Rapid Fire)",
            PlayerSkill::Dash => "Damage: None\nType: Utility\nCooldown: 2.0s",
            PlayerSkill::Earthquake => "Damage: 30 + (150% STR)\nType: Physical\nCooldown: 1.0s",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PlayerClass {
    #[default]
    Warrior,
    Mage,
    Archer,
    Druid,
}

#[derive(Resource, Default)]
pub struct PlayerEquipment {
    pub player_class: PlayerClass,

    pub weapon: Option<Item>,
    pub helmet: Option<Item>,
    pub armor: Option<Item>,
    pub pants: Option<Item>,
    pub shoes: Option<Item>,
    pub ring: Option<Item>,
    pub earring: Option<Item>,
    pub necklace: Option<Item>,
    pub gemstone: Option<Item>,
    
    // Skill slots
    pub skill_1: Option<Item>,
    pub skill_2: Option<Item>,
    pub skill_3: Option<Item>,
    pub skill_4: Option<Item>,

    pub inventory: Vec<Item>,
}

impl PlayerEquipment {
    pub fn get_total_stats(&self, base: &CombatStats) -> CombatStats {
        let mut total = base.clone();
        for item_opt in [
            &self.weapon, &self.helmet, &self.armor, &self.pants,
            &self.shoes, &self.ring, &self.earring, &self.necklace, &self.gemstone,
            &self.skill_1, &self.skill_2, &self.skill_3, &self.skill_4
        ] {
            if let Some(item) = item_opt {
                total.intelligence += item.intelligence;
                total.strength += item.strength;
                total.agility += item.agility;
                total.crit_rate += item.crit_rate;
                total.crit_damage += item.crit_damage;
                total.dodge_rate += item.dodge_rate;
            }
        }
        total
    }

    pub fn has_skill(&self, skill: PlayerSkill) -> bool {
        for slot in [&self.skill_1, &self.skill_2, &self.skill_3, &self.skill_4] {
            if let Some(item) = slot {
                if let Some(granted) = item.granted_skill {
                    if granted == skill {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PickupType {
    Xp,
    Gold,
    Material,
    Weapon,
    Helmet,
    Armor,
    Pants,
    Shoes,
    Ring,
    Earring,
    Necklace,
    Gemstone,
    SkillGem,
}

#[derive(Component)]
pub struct Pickup {
    pub pickup_type: PickupType,
    pub amount: u32,
}
