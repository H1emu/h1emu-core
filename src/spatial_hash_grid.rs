use std::collections::HashSet;

use super::lib_utils::sat;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct SpatialHashGrid {
    _bounds: Bounds,
    _dimensions: Dimensions,
    _cells: Vec<Vec<HashSet<u64>>>,
}

pub struct Dimensions {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone)]
pub struct TwoDimensionPos {
    pub x: f32,
    pub y: f32,
}
pub struct Client {
    pub position: TwoDimensionPos,
    pub indexes: [[u32; 2]; 2],
    pub id: u64,
}

pub struct Bounds {
    pub x: [f32; 2],
    pub y: [f32; 2],
}
#[wasm_bindgen]
impl SpatialHashGrid {
    #[wasm_bindgen(constructor)]
    pub fn new(bounds: Vec<f32>, dimensions: Vec<u32>) -> SpatialHashGrid {
        let mut t: Vec<Vec<HashSet<u64>>> = vec![];
        for x in 0..dimensions[0] {
            t.push(vec![]);
            for _y in 0..dimensions[1] {
                t[x as usize].push(HashSet::new())
            }
        }
        SpatialHashGrid {
            _bounds: Bounds {
                x: [bounds[0], bounds[1]],
                y: [bounds[2], bounds[3]],
            },
            _dimensions: Dimensions {
                x: dimensions[0],
                y: dimensions[1],
            },
            _cells: t,
        }
    }
    pub fn create_client(&mut self, position: Vec<f32>, id: u64) -> Vec<u32> {
        let mut client = Client {
            position: TwoDimensionPos {
                x: position[0],
                y: position[1],
            },
            indexes: [[0, 0], [0, 0]],
            id,
        };
        self._insert(&mut client);
        let mut indexes_as_vec: Vec<u32> = vec![];
        indexes_as_vec.append(&mut client.indexes[0].to_vec());
        indexes_as_vec.append(&mut client.indexes[1].to_vec());
        return indexes_as_vec;
    }
    pub fn update(&mut self, position: Vec<f32>, indexes: Vec<u32>, id: u64) -> Vec<u32> {
        self.remove(indexes, id.clone());
        let mut client = Client {
            id,
            indexes: [[0, 0], [0, 0]],
            position: TwoDimensionPos {
                x: position[0],
                y: position[1],
            },
        };
        self._insert(&mut client);
        let mut indexes_as_vec: Vec<u32> = vec![];
        indexes_as_vec.append(&mut client.indexes[0].to_vec());
        indexes_as_vec.append(&mut client.indexes[1].to_vec());
        return indexes_as_vec;
    }

    pub fn remove(&mut self, indexes: Vec<u32>, id: u64) -> () {
        let indexes = [[indexes[0], indexes[1]], [indexes[2], indexes[3]]];
        for x in indexes[0] {
            for y in indexes[1] {
                self._cells[x as usize][y as usize].remove(&id);
            }
        }
    }
    pub fn find_nearby(&mut self, full_position: Vec<f32>, radius: f32) -> Vec<u64> {
        let position = TwoDimensionPos {
            x: full_position[0],
            y: full_position[1],
        };
        let i1_calc_x = position.x - radius;
        let i1_calc_y = position.y - radius;
        let i1 = self._get_cell_index(&[i1_calc_x, i1_calc_y]);
        let i2_calc_x = position.x + radius;
        let i2_calc_y = position.y + radius;
        let i2 = self._get_cell_index(&[i2_calc_x, i2_calc_y]);
        let mut clients: HashSet<u64> = HashSet::new();
        for x in i1[0]..i2[0] + 1 {
            for y in i1[1]..i2[1] + 1 {
                clients.extend(self._cells[x as usize][y as usize].clone());
            }
        }
        let v = clients.into_iter().collect();
        return v;
    }
}
impl SpatialHashGrid {
    fn _get_cell_index(&self, position: &[f32; 2]) -> [u32; 2] {
        let x = sat((position[0] - self._bounds.x[0]) / (self._bounds.y[0] - self._bounds.x[0]));
        let y = sat((position[1] - self._bounds.x[1]) / (self._bounds.y[1] - self._bounds.x[1]));
        let x_index = f32::floor(x * (self._dimensions.x - 1) as f32);
        let y_index = f32::floor(y * (self._dimensions.y - 1) as f32);

        [x_index as u32, y_index as u32]
    }

    pub fn get_cells(&self) -> Vec<Vec<HashSet<u64>>> {
        return self._cells.clone();
    }

    fn _insert(&mut self, client: &mut Client) -> () {
        let scale: f32 = 1.0;
        let i1_calc_x = client.position.x - scale / 2.0;
        let i1_calc_y = client.position.y - scale / 2.0;
        let i1 = self._get_cell_index(&[i1_calc_x, i1_calc_y]);
        let i2_calc_x = client.position.x + scale / 2.0;
        let i2_calc_y = client.position.y + scale / 2.0;
        let i2 = self._get_cell_index(&[i2_calc_x, i2_calc_y]);

        client.indexes = [i1, i2];

        for x in i1 {
            for y in i2 {
                self._cells[x as usize][y as usize].insert(client.id.clone());
            }
        }
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn create_spatial_hash_grid() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        super::SpatialHashGrid::new(bounds, dimensions);
    }
    #[test]
    fn get_cell_index() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let cell_index = sgrid._get_cell_index(&[69.5, 55.4]);
        assert_eq!(cell_index, [52, 52])
    }
    #[test]
    fn create_client() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        sgrid.create_client(position, 1);
    }
    #[test]
    fn get_client_indices() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let indexes_vec = sgrid.create_client(position, 1);
        assert_eq!(indexes_vec, [49, 49, 49, 49].to_vec());
    }
    #[test]
    fn insert() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let id = 1;
        let indexes = sgrid.create_client(position.clone(), id.clone());
        assert_eq!(
            sgrid._cells[indexes[0] as usize][indexes[1] as usize].len(),
            1
        );
        assert_eq!(
            sgrid._cells[indexes[0] as usize][indexes[1] as usize].contains(&id),
            true
        );
    }
    #[test]
    fn remove_client() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let id = 1;
        let indexes = sgrid.create_client(position.clone(), id.clone());
        sgrid.remove(indexes.clone(), id);

        assert_eq!(
            sgrid._cells[indexes[0] as usize][indexes[1] as usize].contains(&id),
            false
        );
    }
    #[test]
    fn update_client() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [10.0, 20.0, 3.0].to_vec();
        let id = 1;
        let indexes = sgrid.create_client(position.clone(), id.clone());
        let new_pos = [1.0, 2.0, 3.0].to_vec();
        let new_indexes = sgrid.update(new_pos.clone(), indexes.clone(), id);
        assert_eq!(new_indexes, [49, 49, 49, 49].to_vec());
        assert_eq!(
            sgrid._cells[indexes[0] as usize][indexes[1] as usize].contains(&id),
            false
        );
        assert_eq!(
            sgrid._cells[new_indexes[0] as usize][new_indexes[1] as usize].contains(&id),
            true
        );
    }
    #[test]
    fn nearby_clients() {
        let dimensions = [100, 100].to_vec();
        let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [10.0, 20.0, 3.0].to_vec();
        let id = 5;
        let id2 = 45;
        sgrid.create_client(position.clone(), id.clone());
        sgrid.create_client(position, id2.clone());
        let nearby = sgrid.find_nearby([0.0, 0.0, 0.0, 0.0].to_vec(), 300.0);
        assert_eq!(nearby.len(), 2);
        let mut v: Vec<u64> = vec![];
        v.push(id);
        v.push(id2);
        assert_eq!(nearby.contains(&id), true);
        assert_eq!(nearby.contains(&id2), true);
    }
}
