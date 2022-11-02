// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            h if h == 0 => Some(Player {
                health: 100,
                mana: match self.mana {
                    Some(x) => Some(100),
                    None => None,
                },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) => match m {
                m if m > mana_cost => {
                    self.mana = Some(m - mana_cost);
                    return mana_cost * 2;
                }
                _ => 0,
            },
            None => {
                match self.health {
                    h if h > mana_cost => {
                        self.health = self.health - mana_cost;
                    }
                    _ => {
                        self.health = 0;
                    }
                };
                return 0;
            }
        }
    }
}
