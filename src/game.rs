use image::{GenericImageView};
use rand::Rng;
use crate::cell::Cell;

pub struct Game {
  pub width: f64,
  pub height: f64,
  pub scale: f64,
  pub size: usize,
  pub grid: Vec<Cell>,
  pub paused: bool
}

impl Game {
  pub fn new(width: f64, height: f64, scale: f64) -> Game {
    let w = (width / scale).floor();
    let h = (height / scale).floor();
    let size = (w * h) as usize;
    Game {
      grid: vec![Cell::new(); size],
      width: w,
      height: h,
      scale,
      size,
      paused: true
    }
  }

  pub fn init(&mut self) {
    for c in 0..self.size {
        self.grid[c].init(c, self.width, self.height);
    }
  }

  pub fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  pub fn randomise(&mut self) {
    let mut rng = rand::thread_rng();
    for c in 0..self.size {
      let n: usize = rng.gen();
      self.grid[c].alive = n % 2 == 0;
    }
  }

  pub fn count_neighbours(&self, n: usize) -> u8 {
    let mut count = 0;
    let cell = self.grid[n];
    for i in cell.neighbours.iter() {
      let neighbour = self.grid[*i as usize];
      if neighbour.alive {
        count += 1;
      }
    }
    count
  }

  pub fn update(&mut self) {
    // make new grid
    let mut next = vec![Cell::new(); self.size];
    // update cell state
    for (i, cell) in self.grid.iter().enumerate() {
      let alive = cell.alive;
      let neighbours = self.count_neighbours(i);
      // change state based on living neighbours
      if !alive && neighbours == 3 {
          next[i].alive = true;
      } else if (neighbours < 2) || (neighbours > 3) {
          next[i].alive = false;
      } else {
          next[i] = *cell;
      }
      next[i].coords = cell.coords;
      next[i].neighbours = cell.neighbours;
    }
    // recycle grid
    self.grid = next;
  }

  pub fn get_cell(&self, coords: [f64; 2]) -> usize {
    let x = (coords[0] / self.scale).floor();
    let y = (coords[1] / self.scale).floor();
    let cell = (y * self.width) + x;
    cell as usize
  }

  pub fn paint(&mut self, coords: [f64; 2]) {
    let index = self.get_cell(coords);
    self.grid[index].toggle_life();
  }

  pub fn image_to_grid(&mut self, filename: &str) {
    let im = image::open(filename).unwrap();

    // TODO better error checking
    let (a, b) = im.dimensions();
    let u_scale = self.scale as u32;
    let u_width = self.width as u32;
    let u_height = self.height as u32;

    assert!(u_width * u_scale == a);
    assert!(u_height * u_scale == b);
    
    let white: [u8; 4] = [255; 4]; 
    for c in 0..self.size {
      let cell = self.grid[c];
      let coords = cell.get_coords(c, self.width);
      let pixel = im.get_pixel(coords[0] as u32 * u_scale, coords[1] as u32 * u_scale);
      if pixel.data == white {
        self.grid[c].alive = false;
      } else {
        self.grid[c].alive = true;
      }
    }
  }
}