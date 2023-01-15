use std::collections::{HashMap, HashSet};

use super::lib_utils::sat;
pub struct SpatialHashGrid {
    _bounds: Bounds,
    _dimensions: Dimensions,
    _cells: HashMap<u32, HashSet<String>>,
}

pub struct Dimensions {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct TwoDimensionPos {
    pub x: f32,
    pub y: f32,
}
pub struct Client {
    pub position: TwoDimensionPos,
    pub indexes: [[u32; 2]; 2],
    pub id: String,
}

pub struct Bounds {
    pub x: [f32; 2],
    pub y: [f32; 2],
}
impl SpatialHashGrid {
    pub fn new(bounds: Bounds, dimensions: Dimensions) -> SpatialHashGrid {
        SpatialHashGrid {
            _bounds: bounds,
            _dimensions: dimensions,
            _cells: HashMap::new(),
        }
    }
}
impl SpatialHashGrid {
    pub fn createClient(&mut self, position: Vec<f32>, id: String) -> Client {
        let mut client = Client {
            position: TwoDimensionPos {
                x: position.get(0).unwrap().clone(),
                y: position.get(1).unwrap().clone(),
            },
            indexes: [[0, 0], [0, 0]],
            id,
        };
        self._insert(&mut client);
        return client;
    }

    fn _key(&self, i1: u32, i2: u32) -> u32 {
        let mut s = String::new();
        s.push_str(&i1.to_string());
        s.push_str(".");
        s.push_str(&i2.to_string());
        let checksum = super::crc::crc32_legacy(s.as_bytes(), 0);
        return checksum;
    }

    fn _get_cell_index(&self, position: &[f32; 2]) -> [u32; 2] {
        let x = sat((position[0] - self._bounds.x[0]) / (self._bounds.y[0] - self._bounds.x[0]));
        let y = sat((position[1] - self._bounds.x[1]) / (self._bounds.y[1] - self._bounds.x[1]));
        let x_index = f32::floor(x * (self._dimensions.x - 1.0));
        let y_index = f32::floor(y * (self._dimensions.y - 1.0));

        [x_index as u32, y_index as u32]
    }

    pub fn update(&mut self, client: &mut Client) -> () {
        self.remove(&client);
        self._insert(client);
    }

    pub fn remove(&mut self, client: &Client) -> () {
        for x in client.indexes[0] {
            for y in client.indexes[1] {
                let k = self._key(x, y);
                self._cells.get_mut(&k).unwrap().remove(&client.id);
            }
        }
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
                let k = self._key(x, y);
                println!("key registred {} {}", x, y);
                if !self._cells.contains_key(&k) {
                    self._cells.insert(k.clone(), HashSet::new());
                }
                self._cells.get_mut(&k).unwrap().insert(client.id.clone());
            }
        }
    }

    pub fn find_nearby(&mut self, position: TwoDimensionPos, radius: f32) -> HashSet<&String> {
        let i1_calc_x = position.x - radius;
        let i1_calc_y = position.y - radius;
        let i1 = self._get_cell_index(&[i1_calc_x, i1_calc_y]);
        let i2_calc_x = position.x + radius;
        let i2_calc_y = position.y + radius;
        let i2 = self._get_cell_index(&[i2_calc_x, i2_calc_y]);
        println!("indices {:?} {:?}", i1, i2);
        let mut clients = HashSet::new();
        for x in i1[0]..i2[0] + 1 {
            for y in i1[1]..i2[1] + 1 {
                let k = self._key(x, y);
                if self._cells.contains_key(&k) {
                    for id in self._cells.get(&k).unwrap() {
                        clients.insert(id);
                    }
                }
            }
        }
        return clients;
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn create_spatial_hash_grid() {
        let dimensions = super::Dimensions { x: 10.0, y: 10.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        super::SpatialHashGrid::new(bounds, dimensions);
    }
    #[test]
    fn get_cell_index() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let cell_index = sgrid._get_cell_index(&[69.5, 55.4]);
        assert_eq!(cell_index, [52, 52])
    }
    #[test]
    fn create_client() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let final_pos = super::TwoDimensionPos {
            x: position.get(0).unwrap().clone(),
            y: position.get(1).unwrap().clone(),
        };
        let client = sgrid.createClient(position, "a".to_owned());
        assert_eq!(client.position.x, final_pos.x);
        assert_eq!(client.position.y, final_pos.y);
    }
    #[test]
    fn get_client_indices() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let client = sgrid.createClient(position, "b".to_owned());
        assert_eq!(client.indexes, [[49, 49], [49, 49]]);
    }
    #[test]
    fn insert() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let id = "hello".to_owned();
        let client = sgrid.createClient(position, id.clone());
        let key = sgrid._key(client.indexes[0][0], client.indexes[0][1]);
        assert_eq!(sgrid._cells.contains_key(&key), true);
        assert_eq!(sgrid._cells.get(&key).unwrap().contains(&id), true);
    }
    #[test]
    fn remove_client() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [1.0, 2.0, 3.0].to_vec();
        let id = "hello".to_owned();
        let client = sgrid.createClient(position, id.clone());
        let key = sgrid._key(client.indexes[0][0], client.indexes[0][1]);
        sgrid.remove(&client);
        assert_eq!(sgrid._cells.get(&key).unwrap().contains(&id), false);
    }
    #[test]
    fn update_client() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [10.0, 20.0, 3.0].to_vec();
        let id = "hello".to_owned();
        let mut client = sgrid.createClient(position, id.clone());
        let key = sgrid._key(client.indexes[0][0], client.indexes[0][1]);
        let new_pos = super::TwoDimensionPos { x: 1.0, y: 2.0 };
        client.position = new_pos.clone();
        sgrid.update(&mut client);
        let key2 = sgrid._key(client.indexes[0][0], client.indexes[0][1]);
        assert_eq!(client.indexes, [[49, 49], [49, 49]]);
        assert_eq!(sgrid._cells.get(&key).unwrap().contains(&id), false);
        assert_eq!(sgrid._cells.get(&key2).unwrap().contains(&id), true);
    }
    #[test]
    fn nearby_clients() {
        let dimensions = super::Dimensions { x: 100.0, y: 100.0 };
        let bounds = super::Bounds {
            x: [-1000.0, -1000.0],
            y: [1000.0, 1000.0],
        };
        let mut sgrid = super::SpatialHashGrid::new(bounds, dimensions);
        let position = [10.0, 20.0, 3.0].to_vec();
        let id = "hello".to_owned();
        let id2 = "hellow".to_owned();
        let mut client = sgrid.createClient(position.clone(), id.clone());
        sgrid.createClient(position, id2.clone());
        println!("{:?}", sgrid._cells.clone());
        let new_pos = super::TwoDimensionPos { x: 350.0, y: 20.0 };
        client.position = new_pos.clone();
        let nearby = sgrid.find_nearby(client.position, 300.0);
        assert_eq!(nearby.len(), 2);
        let mut nearby_set: super::HashSet<&String> = super::HashSet::new();
        nearby_set.insert(&id);
        nearby_set.insert(&id2);
        assert_eq!(nearby, nearby_set);
    }
}
