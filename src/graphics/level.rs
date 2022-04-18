use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use std::collections::HashMap;

use crate::constant::*;
use crate::game::SPRITE_SCALE;
use crate::graphics::tile;
use crate::physics::collides_with;
use crate::player::{Bat, Enemy, EnemyRenderable, Player};
use sdl2::video::WindowContext;
use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator};
use std::path::Path;
use tiled::{parse_file, Frame, PropertyValue};

use super::{graphics, AnimatedTile, Door, Graphics, Rectangle, Renderable, Vector2};

/// 맵의 가로 타일 수
pub const MAP_WIDTH: i32 = 20;

/// 맵의 세로 타일 수
pub const MAP_HEIGHT: i32 = 16;

/// 기울기용 기조체
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Slope {
    pub from: Vector2,
    pub to: Vector2,
}

impl Slope {
    pub fn get_slope(&self) -> f32 {
        (self.to.top().abs() - self.from.top().abs())
            / (self.to.left().abs() - self.from.left().abs())
    }

    pub fn collides_with(&self, rect: Rectangle) -> bool {
        rect.right > self.to.left()
            && rect.left < self.from.left()
            && rect.top < self.to.top()
            && rect.bottom > self.from.top()
            || rect.right > self.from.left()
                && rect.left < self.to.left()
                && rect.top < self.from.top()
                && rect.bottom > self.to.top()
            || rect.left < self.from.left()
                && rect.right > self.to.left()
                && rect.top < self.from.top()
                && rect.bottom > self.to.top()
            || rect.left < self.to.left()
                && rect.right > self.from.left()
                && rect.top < self.to.top()
                && rect.bottom > self.from.top()
    }
}

/// 지도용 구조체
/// 지도에는 map용 파일과
/// 각 map 블럭에 대한 정보를 넣는다.
pub struct Level<'a> {
    pub x: i32,     //  x
    pub y: i32,     //  y
    pub cam_x: i32, // camera_x
    pub cam_y: i32, // camera_y
    pub tile_atlases: HashMap<usize, tile::TileAtlas>,
    pub width: u32,  // total number of tile in horizontal in this map
    pub height: u32, // total numbr of tile in vertical in this map
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_widths: HashMap<usize, u32>, // width of a tile in pixels
    pub tile_heights: HashMap<usize, u32>, // height of a tile in pixels
    pub layers: Vec<tiled::Layer>,
    pub textures: HashMap<usize, Texture<'a>>,
    pub blocks: Vec<Rect>,
    pub slopes: Vec<Slope>,
    pub gids: HashMap<u32, usize>,
    pub animations: HashMap<u32, AnimatedTile>,
    pub start_pos: Vector2,
    pub doors: Vec<Door>,
    pub enemies: Vec<Box<dyn EnemyRenderable>>,
}

impl<'a> Level<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, path: String) -> Level<'a> {
        // read tmx file
        let map: tiled::Map = parse_file(Path::new(&(ASSET_DIR.to_owned() + &path))).unwrap();

        let layers: Vec<tiled::Layer> = map.layers;
        let object_group: Vec<tiled::ObjectGroup> = map.object_groups;
        let tile_sets: Vec<tiled::Tileset> = map.tilesets;

        let mut slopes = vec![];
        let mut textures = HashMap::new();
        let mut tile_atlases = HashMap::new();
        let mut tile_widths = HashMap::new();
        let mut tile_heights = HashMap::new();

        let mut gids = HashMap::new();
        gids.insert(0, 0);

        let mut animations = HashMap::new();

        let mut start_pos: Vector2 = Vector2(0., 0.);
        let mut doors: Vec<Door> = vec![];
        let mut enemies: Vec<Box<dyn EnemyRenderable>> = vec![];

        for (i, tileset) in tile_sets.iter().enumerate() {
            let tile_width = tileset.tile_width;
            let tile_height = tileset.tile_height;

            let texture = texture_creator
                .load_texture(Path::new(&(ASSET_DIR.to_owned() + &tileset.images[0].source)))
                .unwrap();

            // tile_atlas는 현재 tileset의 Texture를 tile 한 개의 폭, 높이로 잘라
            // first_gid 부터 다음 tileset의 first_gid 까지를 전체 크기로 하는 타일 정보를 만든다.
            let tile_atlas =
                tile::TileAtlas::new(&texture, tileset.first_gid, tile_width, tile_height);
            textures.insert(i, texture);

            // tile_atlas를 만들고, gid가 주어졌을 때 어떤 tile_atlas에서 부터 찾아야할지
            // 참조할 수 있는 역참조 테이블을 만든다.
            for (j, _) in tile_atlas.atlas.iter().enumerate() {
                gids.insert(j as u32 + tileset.first_gid, i);
            }

            tile_atlases.insert(i, tile_atlas);
            tile_widths.insert(i, tileset.tile_width);
            tile_heights.insert(i, tileset.tile_height);

            // tileset에 <tile /> 태그가 붙는 경우 <frame /> 태그를 통한 애니메이션이 설정된다.
            // 각각의 Animation에 대한 (gid로 대표) Frame정보는 animaions에 넣는다.

            if !tileset.tiles.is_empty() {
                for tile in &tileset.tiles {
                    if let Some(animation) = &tile.animation {
                        let animation: Vec<Frame> = animation
                            .iter()
                            .map(|frame| Frame {
                                tile_id: tileset.first_gid + frame.tile_id,
                                duration: frame.duration,
                            })
                            .collect();
                        animations.insert(
                            tile.id + tileset.first_gid,
                            AnimatedTile::new(animation, 0, 0),
                        );
                    }
                }
            }
        }

        // layer의 이름이 collision인 일반 Tile의 경우에는 해당하는 값의 좌표를 blocks에 넣는다.
        let mut blocks = vec![];

        for (_, layer) in layers.iter().enumerate() {
            if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                if layer.name == "collision" {
                    for y in 0..map.height {
                        for x in 0..map.width {
                            let gid = tiles[y as usize][x as usize].gid;
                            if gid != 0 {
                                blocks.push(Rect::new(
                                    (x * map.tile_width) as i32,
                                    (y * map.tile_height) as i32,
                                    map.tile_width,
                                    map.tile_height,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // object 그룹은 slope에 해당하는 점들을 등록한다.
        for (_, object_group) in object_group.iter().enumerate() {
            if object_group.name == "slope" {
                let objects = &object_group.objects;
                for object in objects {
                    if let tiled::ObjectShape::Polyline { points } = &object.shape {
                        let mut from = Vector2(
                            (object.x + points[0].0).ceil(),
                            (object.y + points[0].1).ceil(),
                        );

                        points.iter().skip(1).for_each(|&point| {
                            let to =
                                Vector2((object.x + point.0).ceil(), (object.y + point.1).ceil());
                            slopes.push(Slope { from, to });
                            from = to;
                        });
                    }
                }
            } else if object_group.name == "start" {
                let objects = &object_group.objects;
                start_pos = Vector2(objects[0].x, objects[0].y);
            } else if object_group.name == "doors" {
                let objects = &object_group.objects;
                for object in objects {
                    if let Some(PropertyValue::StringValue(s)) =
                        object.properties.get("destination")
                    {
                        doors.push(Door::new(
                            (*s).clone(),
                            Rectangle {
                                left: object.x,
                                right: object.x + object.width,
                                top: object.y,
                                bottom: object.y + object.height,
                                width: object.width,
                                height: object.height,
                            },
                        ));
                    }
                }
            } else if object_group.name == "enemies" {
                let objects = &object_group.objects;
                for object in objects {
                    if object.name == "bat" {
                        let mut bat = Bat::new(object.x as i32, object.y as i32);
                        bat.add_animation(
                            "fly_left".into(),
                            Rect::new(32, 32, 16, 16),
                            150,
                            false,
                            3,
                            1,
                        );
                        bat.add_animation(
                            "fly_right".into(),
                            Rect::new(32, 48, 16, 16),
                            150,
                            false,
                            3,
                            1,
                        );
                        bat.set_animation("fly_left".into());
                        enemies.push(Box::new(bat));
                    }
                }
            }
        }

        Level {
            x: 0,
            y: 0,
            cam_x: 0,
            cam_y: 0,
            tile_atlases,
            width: map.width,
            height: map.height,
            tile_width: map.tile_width,
            tile_height: map.tile_height,
            tile_widths,
            tile_heights,
            layers,
            textures,
            blocks,
            gids,
            slopes,
            animations,
            start_pos,
            doors,
            enemies,
        }
    }

    /// translate position (left, top) to tile
    /// map is display rom x, y
    pub fn point_to_tile(&self, tile_index: usize, left: i32, top: i32) -> (i32, i32) {
        let o_x = self.x.max(left);
        let o_y = self.y.max(top);

        let tile_width = *self.tile_widths.get(&tile_index).unwrap();
        let tile_height = *self.tile_heights.get(&tile_index).unwrap();
        let clamp_x = o_x.min(left + (self.width * tile_width) as i32 - 1);
        let clamp_y = o_y.min(top + (self.height * tile_height) as i32 - 1);

        let tile_x = (clamp_x - self.x) / tile_width as i32;
        let tile_y = (clamp_y - self.y) / tile_height as i32;

        (tile_x, tile_y)
    }

    /// translate (tile_x, tile_y)
    /// to coordinates
    pub fn get_tile_xy(&self, tile_x: u32, tile_y: u32) -> (f64, f64) {
        let tile_x: u32 = tile_x.min(self.width);
        let tile_y: u32 = tile_y.min(self.height);

        ((tile_x * self.tile_width) as f64, (tile_y * self.tile_height) as f64)
    }

    pub fn update(&mut self, dt: u32, player: &Player) {
        self.animations.values_mut().for_each(|v| {
            v.update(dt);
        });

        self.enemies.iter_mut().for_each(|e| {
            e.to_enemy_mut().unwrap().update(dt, player);
        });
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera_rect: &Rect) {
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.name != "collision" {
                if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                    let (tile_left, tile_top) = self.point_to_tile(i, camera_rect.x, camera_rect.y);
                    let (tile_right, tile_bottom) = self.point_to_tile(
                        i,
                        camera_rect.x + camera_rect.w,
                        camera_rect.y + camera_rect.h,
                    );

                    // 카메라의 좌측/위가 타일에 정확히 일치한다면 tile_start_x와 tile_start_y는 0이 되겠지만
                    // 그렇지 않은 경우는 좌측/위 타일에서 떨어진 좌표값만큼을 반환하게된다.
                    // 이 값은 대상 texture의 영역을 어디에 노출시킬까 정할 때, 대상의 타일을 tile_start_x, tile_start_y만큼
                    // 좌상단으로 올림으로써 부드러운 스크롤을 가능하게한다.
                    let tile_width = *self.tile_widths.get(&i).unwrap();
                    let tile_height = *self.tile_heights.get(&i).unwrap();

                    let tile_start_x = camera_rect.x - tile_left * tile_width as i32;
                    let tile_start_y = camera_rect.y - tile_top * tile_height as i32;

                    for y in tile_top..tile_bottom {
                        for x in tile_left..tile_right {
                            let gid = tiles[y as usize][x as usize].gid;

                            if gid != 0 {
                                // gid 로 부터 tile_atlases의 index를 구함
                                // tile_atlases의 모든 first_gid 중 gid 값보다 큰 것 중에 가장 작은 인덱스를 구할 것
                                // 해당 인덱스가 tile_atlases의 인덱스이다.
                                // TODO : 이와 같은 방식은 비 경제적이다.
                                // tile_atlas를 생성할 때, 어떤 texutre index인지, 그리고 해당 texture의 어떤 위치인지를
                                // 등록하는 편이 좋다.
                                // 즉 말하자면 Vector이면 되지, 굳이 HashMap일 필요가 없다.
                                // Vec<(texture_idx: usize, x, y, w, h)> 이면 됨..
                                let idx_gid = self.gids.get(&gid).unwrap();

                                let tile_atlas = self.tile_atlases.get(idx_gid).unwrap();

                                let rect = if layer.name == "animation" {
                                    let animation = self
                                        .animations
                                        .get(&tiles[y as usize][x as usize].gid)
                                        .unwrap();
                                    tile_atlas.get_tile_rect(animation.get_current_frame())
                                } else {
                                    tile_atlas.get_tile_rect(gid)
                                };

                                let dest = Rect::new(
                                    (((x - tile_left) as i32 * tile_width as i32 - tile_start_x)
                                        as f32
                                        * SPRITE_SCALE) as i32,
                                    (((y - tile_top) as i32 * tile_height as i32 - tile_start_y)
                                        as f32
                                        * SPRITE_SCALE) as i32,
                                    (tile_width as f32 * SPRITE_SCALE) as u32,
                                    (tile_height as f32 * SPRITE_SCALE) as u32,
                                );

                                canvas
                                    .copy_ex(
                                        &self.textures[idx_gid],
                                        Some(rect),
                                        Some(dest),
                                        0.0,
                                        None,
                                        false,
                                        false,
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn render_enemies(&self, graphics: &Graphics, canvas: &mut WindowCanvas) {
        // render enemies
        for enemy in &self.enemies {
            let renderable = enemy.to_renderable().unwrap();
            graphics.render_sprite(canvas, renderable);
        }
    }

    pub fn collided_blocks(&self, other: &Rect) -> Vec<Rect> {
        self.blocks.iter().filter(|block| collides_with(*block, other)).copied().collect()
    }

    pub fn collided_slopes(&self, other: &Rect) -> Vec<Slope> {
        self.slopes
            .iter()
            .filter(|slope| (*slope).collides_with((*other).into()))
            .copied()
            .collect()
    }

    pub fn collided_doors(&self, other: &Rect) -> Vec<Door> {
        self.doors.iter().filter(|door| collides_with(&door.position, other)).cloned().collect()
    }
}
