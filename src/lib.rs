pub mod template;

// Use this file to add helper functions and additional modules.

pub type V2 = (i32, i32);

#[derive(Debug)]
pub struct CharMatrix(Vec<Vec<char>>);

impl CharMatrix {
	pub fn bounds(&self) -> V2 {
		(self.0.len() as i32, self.0[0].len() as i32)
	}

    pub fn at(&self, p: V2) -> Option<char> {
        self.0.get(p.0 as usize)?.get(p.1 as usize).cloned()
    }

    pub fn from_str(input: &str) -> CharMatrix {
        let v = input.lines().filter(|l| { !l.is_empty() })
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        CharMatrix(v)
    }

    pub fn susedi8(&self, orig: V2) -> impl Iterator<Item = char> {
        (-1..=1).flat_map(move |dx|
            (-1..=1).filter_map(move |dy|
                if dx != 0 || dy != 0 {
                    self.at((orig.0 + dx, orig.1 + dy))
                }
                else {
                    None
                }
            )
        )
    }

    pub fn susedi4(&self, orig: V2) -> impl Iterator<Item = char> {
        vec![(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(dx, dy)|
                if dx != 0 || dy != 0 {
                    self.at((orig.0 + dx, orig.1 + dy))
                }
                else {
                    None
                }
            )
    }

    pub fn set(&mut self, p: V2, c: char) {
        self.0.get_mut(p.0 as usize).map(|row| {
            row.get_mut(p.1 as usize).map(|cc: &mut char| *cc = c);
        });
    }
}
