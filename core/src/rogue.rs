use crate::common::BarValue;


pub struct Rogue {
    pub energy: BarValue<f32>,
    energy_recover_per_second: f32,
    pub combat_points: BarValue<u32>,
    pub spells: Vec<Box<dyn RogueSpell>>
}

pub trait RogueSpell {
    fn take(&self, rouge: &mut Rogue) -> u32;
    fn check(&self, rouge: &mut Rogue) -> bool;
}

struct HuiShang {
    
}

impl RogueSpell for HuiShang {
    fn take(&self, rouge: &mut Rogue) -> u32 {

        if !self.check(rouge) {
            panic!("不能释放");
        }

        rouge.combat_points += 1;
        rouge.energy -= 30.0;

        return 300
    }
    
    fn check(&self, rouge: &mut Rogue) -> bool {
        rouge.energy.value() > 30.0
    }
}

struct DuShang {
    
}

impl RogueSpell for DuShang {
    fn take(&self, rouge: &mut Rogue) -> u32 {

        if !self.check(rouge) {
            panic!("不能释放");
        }

        let combat_points = rouge.combat_points.value(); 

        rouge.combat_points -= combat_points;

        return combat_points * 200
    }
    
    fn check(&self, rouge: &mut Rogue) -> bool {
        rouge.combat_points.value() > 0
    }
}


impl Rogue {
    pub fn new() -> Self {

        Self {
            energy: BarValue::new(0.0, 100.0, 0.0),
            energy_recover_per_second: 20.0,
            combat_points: BarValue::new(0, 5, 0),
            spells: vec![Box::new(HuiShang {}), Box::new(DuShang {})]
        }
    }

    pub fn auto_battle(&mut self, time_ms: u32) {
        self.energy += (time_ms as f32) / 1000.0 * self.energy_recover_per_second;
    }
}