use bevy::prelude::*;

// Quadtree itu sendiri
#[derive(Resource, Debug, Clone)]
pub struct TkQuadTree {
    boundaries: Rect,
    tiles_unit: Vec<Vec2>, // apakah kita akan menyimpan data dalam poin atau rect
    divided: bool,
    childnode: Option<[Box<TkQuadTree>; 4]>,
}

impl TkQuadTree {
    // Membuat Tilemap Quadtree baru, Pastikan
    pub fn new(border_x0: f32, border_y0: f32, border_x1: f32, border_y1: f32) -> Self {
        Self {
            boundaries: Rect::new(border_x0, border_y0, border_x1, border_y1),
            tiles_unit: Vec::new(),
            divided: false,
            childnode: None,
        }
    }
    pub fn insert(&mut self, tile: Vec2) -> bool {
        if self.boundaries.contains(tile) == false {
            return false;
        }
        if self.tiles_unit.len() <= 4 {
            self.tiles_unit.push(tile);
            return true; // ini untuk memastikan fungsi berhenti di sini jika masih bisa dimasukkan
        } else {
            if self.divided == false {
                self.subdivide()
            }
            return self.distribute(tile);
        }
    }
    // # Fungsi untuk membangun anakan
    fn subdivide(&mut self) {
        self.divided = true;
        let center = self.boundaries.center();
        let (centerx, centery) = (center.x, center.y);
        let quadra: [Box<TkQuadTree>; 4] = [
            // Top Left
            Box::new(TkQuadTree::new(
                self.boundaries.min.x,
                centery,
                centerx,
                self.boundaries.max.y,
            )),
            // Bottomleft
            Box::new(TkQuadTree::new(
                self.boundaries.min.x,
                self.boundaries.min.y,
                centerx,
                centery,
            )),
            // TopRight
            Box::new(TkQuadTree::new(
                centerx,
                centery,
                self.boundaries.max.x,
                self.boundaries.max.y,
            )),
            // Bottomright
            Box::new(TkQuadTree::new(
                centerx,
                self.boundaries.min.y,
                self.boundaries.max.x,
                centery,
            )),
        ];
        self.childnode = Some(quadra)
    }
    // # Fungsi Untuk Memindahkan Nilai Setelah Subdivide
    fn distribute(&mut self, tile: Vec2) -> bool {
        // fungsi distribute adalah fungsi yang akan dijalankan setelah quadtree ini melakukan
        // subdivide. karena pada dasarnya nilainya masih ada di quadtree, maka nilai itu akan di
        // lakukan perpindahan ke quadtree yang ada di bawahnya
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
