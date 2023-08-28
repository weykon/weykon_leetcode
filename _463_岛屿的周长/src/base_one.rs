
pub fn here_my_first_version_for_this_problem(grid: Vec<Vec<i32>>) -> i32 {
    // simple check
    fn check_arroud(pos: Pos, grid: &Vec<Vec<i32>>) -> i32 {
      println!("静静等待w:{0},h:{1},的计算", pos.w_id, pos.h_id);
      fn l(pos: &Pos, grid: &Vec<Vec<i32>>) -> Option<i32> {
          if pos.w_id == 0 {
              return None;
          }
          if let Some(row) = grid.get(pos.h_id) {
              match row.get(pos.w_id - 1) {
                  Some(v) => {
                      if *v == 1 {
                          return Some(1);
                      } else {
                          return None;
                      }
                  }
                  None => None,
              }
          } else {
              None
          }
      }
      fn r(pos: &Pos, grid: &Vec<Vec<i32>>) -> Option<i32> {
          if let Some(row) = grid.get(pos.h_id) {
              match row.get(pos.w_id + 1) {
                  Some(v) => {
                      if *v == 1 {
                          return Some(1);
                      } else {
                          return None;
                      }
                  }
                  None => None,
              }
          } else {
              None
          }
      }
      fn t(pos: &Pos, grid: &Vec<Vec<i32>>) -> Option<i32> {
          if pos.h_id == 0 {
              return None;
          }
          if let Some(row) = grid.get(pos.h_id - 1) {
              match row.get(pos.w_id) {
                  Some(v) => {
                      if *v == 1 {
                          return Some(1);
                      } else {
                          return None;
                      }
                  }
                  None => None,
              }
          } else {
              None
          }
      }
      fn b(pos: &Pos, grid: &Vec<Vec<i32>>) -> Option<i32> {
          if let Some(row) = grid.get(pos.h_id + 1) {
              match row.get(pos.w_id) {
                  Some(v) => {
                      if *v == 1 {
                          return Some(1);
                      } else {
                          return None;
                      }
                  }
                  None => None,
              }
          } else {
              None
          }
      }
      let mut edges = 4;
      if l(&pos, grid) != None {
          edges -= 1;
      }
      if r(&pos, grid) != None {
          edges -= 1;
      }
      if t(&pos, grid) != None {
          edges -= 1;
      }
      if b(&pos, grid) != None {
          edges -= 1;
      }
      edges
  }

  let flatten_iter: Vec<_> = grid
      .iter()
      .enumerate()
      .flat_map(|(h_id, row)| row.iter().enumerate().map(move |(w_id, _)| (w_id, h_id)))
      .collect();

  let mut nums = 0;
  for (w_id, h_id) in flatten_iter {
      println!("ONCE::{}", grid[h_id][w_id]);
      if grid[h_id][w_id] == 1 {
          nums += check_arroud(Pos { w_id, h_id }, &grid)
      }
  }
  nums
}
