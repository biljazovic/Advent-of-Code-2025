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

    pub fn transpose(self, empty: char) -> CharMatrix {
        let v = &self.0;
        let res = (0..v.iter().map(|l| l.len()).max().unwrap_or(0))
            .map(|i| v.iter().map(|vi| vi.get(i).unwrap_or(&empty).clone()).collect())
            .collect();
        CharMatrix(res)
    }

    pub fn unwrap(self) -> Vec<Vec<char>> {
        self.0
    }

    pub fn map_view(&self) -> impl Iterator<Item = (V2, char)> {
        self.0.iter().enumerate()
            .flat_map(|(i, vi)| {
                vi.iter().enumerate()
                    .map(move |(j, c)| ((i as i32, j as i32), *c))
        })
    }
}
