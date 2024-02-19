use std::env;
use std::path;
use std::path::PathBuf;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use colored::Colorize;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Image, Text};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use ggez::mint::{Point2, Vector2};
use rand::Rng;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use std::thread;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;


use holy_crab_best_path::MinerRobot;
const SCREEN_SIZE: f32 = 1600.;



fn main()->GameResult{


    run_my_game()
}

fn run_bot(){


    // match run {
    //     Ok(mut running) => {
    //         loop{
    //             let _ = running.game_tick();
    //             let game_over_ref = game_over.borrow();
    //             // if the game_over value is true then the game ends
    //             if *game_over_ref {
    //                 break;
    //             }
    //             sleep(Duration::from_millis(2000))
    //         };
    //     }
    //     Err(e) => {
    //         println!("Error in runnable - main");
    //         println!("{:?}", e);
    //     }
    // }
    // println!("{}", "THE GAME ENDED!".green());
}

fn run_my_game()->GameResult{
    // Make a Context.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE, SCREEN_SIZE))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
    frames: usize,
    map: Vec<Vec<(Tile, usize)>>,
    grass_image: Image,
    water_image: Image,
    lava_image: Image,
    snow_image: Image,
    deep_water_image: Image,
    sand_image: Image,
    sand_water_image: Image,
    hill_image1: Image,
    hill_image2: Image,
    mountain_image: Image,
    wall_image: Image,
    street_image: Image,
    down_hill: Image,
    size: f32,
    zoom: f32,
    offset: (usize, usize),
    coordinates: Arc<Mutex<(usize, usize)>>
}

fn choose_image<'a>(game: &'a MyGame, map: &'a Vec<Vec<(Tile,usize)>>, coordinate: (usize, usize)) -> &'a Image {
    let tile = &map[coordinate.0][coordinate.1];
    // aggiungere controlli su tile in bass per aggiungere scalino
    return match tile.0.tile_type {
        TileType::Grass => &game.grass_image,
        TileType::Lava => &game.lava_image,
        TileType::Snow => &game.snow_image,
        TileType::DeepWater => &game.deep_water_image,
        TileType::Sand => {
            if coordinate.1+1 == map[0].len(){
                return &game.sand_image;
            }
            let tile_down = &map[coordinate.0][coordinate.1+1];
            if tile_down.0.tile_type == TileType::ShallowWater {
                &game.sand_water_image
            } else {
                return if tile.1 > 8 {
                    &game.sand_image
                } else {
                    &game.sand_image
                }
            }


        },
        TileType::Hill => {

            if coordinate.1+1 == map[0].len(){
                return if tile.1 > 8 {
                    &game.hill_image1
                } else {
                    &game.hill_image2
                }
            }

            let tile_down = &map[coordinate.0][coordinate.1+1];
            if tile_down.0.tile_type == TileType::Grass {
                &game.down_hill
            } else {
                return if tile.1 > 8 {
                    &game.hill_image1
                } else {
                    &game.hill_image2
                }
            }
        },
        TileType::Mountain => &game.mountain_image,
        TileType::Wall => &game.wall_image,
        TileType::Street => &game.street_image,
        TileType::ShallowWater => &game.water_image,
        _ => &game.grass_image
    }
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let tile_grass = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0
        };

        let water_image = Image::from_path(ctx, "/tiles/Map_tile_01.png")?;
        let grass_image = Image::from_path(ctx, "/tiles/Map_tile_23.png")?;
        let lava_image = Image::from_path(ctx, "/tiles/Map_tile_110.png")?;
        let snow_image = Image::from_path(ctx, "/tiles/Map_tile_23.png")?;
        let deep_water_image = Image::from_path(ctx, "/tiles/Map_tile_37.png")?;
        let sand_image = Image::from_path(ctx, "/tiles/sand.png")?;
        let sand_water_image = Image::from_path(ctx, "/tiles/sand_water.png")?;
        let hill_image1 = Image::from_path(ctx, "/tiles/hill1.png")?;
        let hill_image2 = Image::from_path(ctx, "/tiles/hill2.png")?;
        let mountain_image = Image::from_path(ctx, "/tiles/Map_tile_20.png")?;
        let wall_image = Image::from_path(ctx, "/tiles/Map_tile_23.png")?;
        let street_image = Image::from_path(ctx, "/tiles/Map_tile_23.png")?;
        let down_hill = Image::from_path(ctx, "/tiles/down_hill.png")?;

        let gui_start = false;
        let path = PathBuf::new().join("world/bridge2.bin");
        let mut world_generator = WorldgeneratorUnwrap::init(gui_start, Some(path));
        let world = world_generator.gen();

        let map = world.0;
        let mut option_map: Vec<Vec<(Tile, usize)>> = vec![];
        let mut rng = rand::thread_rng();

        /**/

        let robot_thread =  MinerRobot::new();
        let coordinates = robot_thread.coordinates.clone();


        thread::spawn(move || {
            let game_over = robot_thread.game_over.clone();
            let robot_box = Box::new(robot_thread);

            let run = Runner::new( robot_box, &mut world_generator);
            match run {
                Ok(mut running) => {
                    loop{
                        let _ = running.game_tick();
                        let mut game_over_ref = game_over.lock().unwrap();


                        // if the game_over value is true then the game ends
                        if *game_over_ref {
                            break;
                        }
                        sleep(Duration::from_millis(2000))
                    };
                }
                Err(e) => {
                    println!("Error in runnable - main");
                    println!("{:?}", e);
                }
            }
        });

        //let data = receiver.recv().unwrap();



        /**/

        for row in map{
            let mut mrow = vec![];
            for tile in row{
                let n = rng.gen_range(0..10);
                mrow.push((tile, n as usize));
            }
            option_map.push(mrow);
        }

        let res = MyGame {
            frames: 0,
            map: option_map,
            grass_image,
            water_image,
            lava_image,
            snow_image,
            deep_water_image,
            sand_image,
            sand_water_image,
            hill_image1,
            hill_image2,
            mountain_image,
            wall_image,
            street_image,
            down_hill,
            size: 32.,
            zoom: 2.,
            offset: (0,0),
            coordinates
        };


        Ok(res)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        if ctx.keyboard.is_key_pressed(KeyCode::I) {
            self.zoom += 0.1;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::O) {
            self.zoom -= 0.1;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            if self.offset.1 < self.map.len() - (SCREEN_SIZE/self.size/self.zoom) as usize{
                self.offset.1 += 1;
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            if self.offset.1 > 0{
                self.offset.1 -= 1;
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            if self.offset.0 > 0{
                self.offset.0 -= 1;
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            if self.offset.0 < self.map.len() - (SCREEN_SIZE/self.size/self.zoom) as usize{
                self.offset.0 += 1;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // Draw code here...

        println!("************{:?}", self.coordinates.lock().unwrap());

        // TODO: separare in un file la grafica dal main
        // TODO: collegare un bot
        // TODO: visualizzare bot
        // TODO: attach detach grafica

        // eventuali
        // TODO: animazioni mare
        // TODO: migliorie grafiche
        // TODO: animazioni bot

        draw_map(self, &mut canvas);

        canvas.finish(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }
        Ok(())
    }
}

fn draw_map(mygame: &MyGame, canvas: &mut Canvas ){

    let mut x: usize = mygame.offset.0;
    let mut x_from_zero = 0;
    for row in &mygame.map {

        if x >= mygame.map.len(){
            break
        }

        if mygame.size*mygame.zoom*x_from_zero as f32 > SCREEN_SIZE
        {
            break
        }


        let mut y: usize = mygame.offset.1;
        let mut y_from_zero = 0;

        for tile in row{

            if y >= mygame.map.len() {break}
            if mygame.size*mygame.zoom*y_from_zero as f32 > SCREEN_SIZE
            {
                break
            }


            let draw_param = DrawParam::new()
                .dest(Vec2::new((x - mygame.offset.0) as f32 * &mygame.size * &mygame.zoom,
                                (y - mygame.offset.1) as f32 * &mygame.size * &mygame.zoom))
                .scale(Vec2::new(mygame.zoom, mygame.zoom));

            canvas.draw(choose_image(&mygame, &mygame.map, (x, y)), draw_param);

            y+=1;
            y_from_zero += 1;

        }
        x+=1;
        x_from_zero += 1;

    }
}