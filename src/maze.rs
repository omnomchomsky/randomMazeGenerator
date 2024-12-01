use rand::Rng;
use crate::maze::unionfind::UnionFind;

mod unionfind;

#[derive(Clone, Copy)]
struct Cell{
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Cell{
    fn new() ->Self{
        Cell {
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }
}
pub struct Maze {
    rows: usize,
    columns: usize,
    maze_set: UnionFind,
    maze_visual: Vec<Vec<Cell>>,
}

impl Maze {


    pub fn new(rows: usize, cols: usize) -> Maze {
        Maze { rows: rows,
            columns: cols,
            maze_set: UnionFind::new((rows * cols) as isize),
            maze_visual: vec![vec![Cell::new(); cols]; rows]
        }
    }

    fn break_visual_wall(&mut self, random_neighbor: usize, current_cell: usize, direction: &str){
        let (x, y) = self.get_coordinates(current_cell);
        let (n_x, n_y) = self.get_coordinates(random_neighbor);
        match direction {
            "North" =>{
                //self.maze_visual[x][y].north = false;
                self.maze_visual[n_x][n_y].south = false;
            }
            "South" =>{
                self.maze_visual[x][y].south = false;
                //self.maze_visual[n_x][n_y].north = false;
            }
            "East" => {
                //self.maze_visual[x][y].east = false;
                self.maze_visual[x][y].west = false;
            }
            "West" => {
                self.maze_visual[n_x][n_y].west = false;
                //self.maze_visual[n_x][n_y].east = false;
            }
            _ => {}
        }
    }

    pub fn render_maze(&self){
        print!(" ");
        for _ in 0..self.columns{
            print!("_ ")
        }
        println!();
        for row in &self.maze_visual {
            print!("|");
            for cell in row {
                if cell.south {
                    print!("_");
                } else {
                    print!(" ");
                }
                if cell.west {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
    fn random_break_walls(&mut self){
        let mut rng = rand::thread_rng();
        let random_row = rng.gen_range(0..self.rows);
        let random_column = rng.gen_range(0..self.columns);
        let current_cell = self.get_cell_index(random_row, random_column);
        let random_neighbor_num = rng.gen_range(0..3);
        let mut random_neighbor:usize = current_cell;
        let mut direction = "None";
        match random_neighbor_num {
            0 => {//North
                if random_row as isize - 1 >= 0 {
                    random_neighbor = self.get_cell_index(random_row - 1, random_column);
                    direction = "North";
                }
            }
            1 => { //South
                if random_row + 1 < self.rows {
                    random_neighbor = self.get_cell_index(random_row + 1, random_column);
                    direction =  "South";
                }
            }
            2 => { //West
                if random_column as isize -1 >= 0 {
                    random_neighbor = self.get_cell_index(random_row, random_column - 1);
                    direction = "West";
                }
            }
            3 => { //East
                if random_column + 1 < self.columns {
                    random_neighbor = self.get_cell_index(random_row, random_column + 1);
                    direction = "East"
                }
            }
            _ => {}
        }
        let find_random = self.maze_set.find(random_neighbor);
        let find_current = self.maze_set.find(current_cell);
        if find_current == -1 || find_random == -1 || find_current != find_random {
            self.break_visual_wall(random_neighbor, current_cell, direction);
            self.maze_set.union(random_neighbor, current_cell);
        }
    }

    pub fn generate(&mut self) {
        while self.maze_set.num_sets() >= 1{
            self.random_break_walls()
        }
    }

    pub fn get_cell_index(&self, x:usize, y:usize) -> usize {
        x * self.columns + y
    }

    pub fn get_coordinates(&self, cell:usize) -> (usize, usize){
        let row = cell / self.columns;
        let column = cell % self.columns;

        (row, column)
    }
    pub fn get_cell(&mut self, row: usize, column: usize) -> isize {
        self.maze_set.find(self.get_cell_index(row, column)) as isize
    }
    pub fn print_maze(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                print!("{} ", self.get_cell(row, column));
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell() {
        let maze = Maze::new(3, 3);
        assert_eq!(maze.get_cell_index(0, 0), 0);
        assert_eq!(maze.get_cell_index(0, 1), 1);
        assert_eq!(maze.get_cell_index(0, 2), 2);
        assert_eq!(maze.get_cell_index(1, 0), 3);
        assert_eq!(maze.get_cell_index(1, 1), 4);
        assert_eq!(maze.get_cell_index(1, 2), 5);
        assert_eq!(maze.get_cell_index(2, 0), 6);
        assert_eq!(maze.get_cell_index(2, 1), 7);
        assert_eq!(maze.get_cell_index(2, 2), 8);
    }

    #[test]
    fn test_get_coordinates() {
        let maze = Maze::new(3, 3);
        assert_eq!(maze.get_coordinates(0), (0,0));
        assert_eq!(maze.get_coordinates(1), (0,1));
        assert_eq!(maze.get_coordinates(2), (0,2));
        assert_eq!(maze.get_coordinates(3), (1,0));
        assert_eq!(maze.get_coordinates(4), (1,1));
        assert_eq!(maze.get_coordinates(5), (1,2));
        assert_eq!(maze.get_coordinates(6), (2,0));
        assert_eq!(maze.get_coordinates(7), (2,1));
        assert_eq!(maze.get_coordinates(8), (2,2));
    }

    #[test]
    fn test_generate() {
        let x = 5;
        let y = 50;
        let mut maze = Maze::new(x, y);
        assert_eq!(maze.maze_set.num_sets(), x*y);
        //maze.print_maze();
        maze.generate();
        //maze.print_maze();
        maze.render_maze();
        println!("maze generated");
        assert_eq!(maze.maze_set.num_sets(), 0);
    }

}