use wasm_bindgen::prelude::*;

const STATES: [[u8; 3]; 8] = [
    [1, 1, 1],
    [1, 1, 0],
    [1, 0, 1],
    [1, 0, 0],
    [0, 1, 1],
    [0, 1, 0],
    [0, 0, 1],
    [0, 0, 0],
];

#[wasm_bindgen]
pub struct Automata {
    rule_num: u8,
    width: usize,
    height: usize,
    rule: [u8; 8],
}

#[wasm_bindgen]
impl Automata {
    #[wasm_bindgen(constructor)]
    pub fn new(rule_num: u8, width: usize, height: usize) -> Automata {
        let rule = Self::_generate_rule(rule_num);
        Automata {
            rule_num,
            width,
            height,
            rule,
        }
    }

    pub fn generate_automata(&self) -> Vec<u8> {
        let result = self._generate_automata();
        result.into_iter().flatten().collect()
    }
}

impl Automata {
    fn _initial_condition(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; self.width as usize];
        let midpoint = data.len() / 2;
        data[midpoint] = 1;
        data
    }

    fn _generate_rule(rule_num: u8) -> [u8; 8] {
        let mut rule = [0; 8];

        for i in 0..8 {
            rule[7 - i] = (rule_num >> i) & 1;
        }

        rule
    }

    fn _generate_automata(&self) -> Vec<Vec<u8>> {
        let mut game_array = Vec::with_capacity(self.height);

        let mut current = self._initial_condition();
        let mut next = vec![0; self.width];

        for _ in 0..self.height - 1 {
            for i in 0..self.width {
                let triple = [
                    current[(i + self.width - 1) % self.width],
                    current[i],
                    current[(i + 1) % self.width],
                ];
                let id = STATES.iter().position(|s| s == &triple).unwrap();
                next[i] = self.rule[id];
            }
            game_array.push(current.clone());
            std::mem::swap(&mut current, &mut next);
        }
        game_array
    }
}
