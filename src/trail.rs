use rand::Rng;

pub struct TrailLayer {
    data: Vec<f32>,
    pub num_columns: usize,
    pub num_rows: usize,
}

#[inline]
fn index(row: usize, num_columns: usize, column: usize) -> usize {
    row + num_columns * column
}

impl TrailLayer {
    pub fn new(num_columns: usize, height: usize) -> TrailLayer {
        let len = num_columns * height;
        let zero_vec = vec![0.; len];
        TrailLayer {
            num_columns,
            num_rows: len / num_columns,
            data: zero_vec,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.data = self.data.iter().map(|_| rng.gen_range(0., 1.)).collect()
    }

    #[inline]
    pub fn get(&self, row: f32, column: f32) -> Option<&f32> {
        println!("{}, {}", row, column);
        if column as usize >= self.num_columns || (column as i32) < 0 {
            None
        } else if row as usize >= self.num_rows || (row as i32) < 0 {
            None
        } else {
            self.data
                .get(index(row as usize, column as usize, self.num_columns))
        }
    }

    #[inline]
    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        if let Some(elem) = self.data.get_mut(index(row, column, self.num_columns)) {
            *elem = value;
        }
    }

    pub fn as_rgba8(&self) -> Vec<u8> {
        let size = self.num_columns * self.num_rows * 4; // 4 = r g b a
        let mut values = vec![0; size];
        let mut index = 0;
        for elem in self.data.iter() {
            values[index] = 100;
            // values[index] = (elem * 255.0) as u8;
            // values[index + 1] = (elem * 255.0) as u8;
            // values[index + 2] = (elem * 255.0) as u8;
            index += 4;
        }
        values
    }

    // pub fn blur(&self, radius: f32) -> TrailLayer {

    // }
}
