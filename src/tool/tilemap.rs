use bevy::{math::bool, prelude::*};

#[derive(Debug, Clone, Copy)]
pub enum GroundLevelType {}
#[derive(Debug, Clone, Copy)]
pub enum ParticleLevelType {}
#[derive(Debug, Clone, Copy)]
pub enum ObjectLevelType {}

#[derive(Debug, Clone, Copy)]
pub enum TilesType {
    GroundLevel(GroundLevelType),
    ParticlesLevel(ParticleLevelType),
    ObjectLevel(ObjectLevelType),
}

#[derive(Debug, Clone, Copy)]
pub struct Tiles {
    pub pos: Vec2,
    pub tiles_type: TilesType,
}

impl Tiles {
    pub fn new(x: f32, y: f32, tiles_type: TilesType) -> Self {
        Self {
            pos: Vec2::new(x, y),
            tiles_type,
        }
    }
}

#[derive(Component)]
pub struct TilemapQuadTree {
    pub storage: QuadTreeStorage,
    sprite: Sprite,
    layout: TextureAtlasLayout,
}

#[derive(Debug, Clone)]
pub struct QuadTreeStorage {
    boundaries: Rect,
    tiles: Vec<Tiles>,
    divided: bool,
    childnode: Option<[Box<QuadTreeStorage>; 4]>,
}

impl QuadTreeStorage {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            boundaries: Rect::new(x0, y0, x1, y1),
            tiles: Vec::new(),
            divided: false,
            childnode: None,
        }
    }
    pub fn insert(&mut self, tile: Tiles) -> bool {
        if self.boundaries.contains(tile.pos) == false {
            return false;
        }
        if self.tiles.len() <= 4 {
            self.tiles.push(tile);
            return true; // ini untuk memastikan fungsi berhenti di sini jika masih bisa dimasukkan
        } else {
            if self.divided == false {
                self.subdivide()
            }
            return self.distribute(tile);
        }
    }
    fn subdivide(&mut self) {
        self.divided = true;
        let center = self.boundaries.center();
        let (centerx, centery) = (center.x, center.y);
        let quadra: [Box<QuadTreeStorage>; 4] = [
            // Top Left
            Box::new(QuadTreeStorage::new(
                centerx,
                centery,
                self.boundaries.min.x,
                self.boundaries.max.y,
            )),
            // Bottomleft
            Box::new(QuadTreeStorage::new(
                centerx,
                centery,
                self.boundaries.min.x,
                self.boundaries.min.y,
            )),
            // TopRight
            Box::new(QuadTreeStorage::new(
                centerx,
                centery,
                self.boundaries.max.x,
                self.boundaries.max.y,
            )),
            // Bottomright
            Box::new(QuadTreeStorage::new(
                centerx,
                centery,
                self.boundaries.max.x,
                self.boundaries.min.y,
            )),
        ];
        self.childnode = Some(quadra)
    }
    fn distribute(&mut self, tile: Tiles) -> bool {
        let child_node = self.childnode.as_mut().unwrap();
        if child_node[0].insert(tile) {
            return true;
        } else if child_node[1].insert(tile) {
            return true;
        } else if child_node[2].insert(tile) {
            return true;
        } else if child_node[3].insert(tile) {
            return true;
        } else {
            return false;
        }
    }
}
