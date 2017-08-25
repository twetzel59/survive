extern crate sfml;
extern crate survive;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::{Clock, Vector2i};
use survive::*;

fn main() {
    let mut win = GameWindow::new();
    win.rwin.clear(&Color::white());
    win.rwin.display();

    let res = Resources::new();

    let wg = Worldgen::new();
    //let test = Sprite::with_texture(&wg.textures[9]);

    let tilemgr = TileManager::new(wg.textures());

    let mut entitymgr = EntityManager::new();
    //entitymgr.add(entities::deciduous_tree::DeciduousTree::new(&res));
    //entitymgr.add(Box::new(entities::deciduous_tree::DeciduousTree::new(&res)));
    plants::generate_plants(&res, wg.world(), &mut entitymgr);

    let mut player = Player::new(&res);

    let mut stat = stats::Stats::new();

    let mut ui = UiManager::new(&res);

    let mut clock = Clock::start();
    'mainl: loop {
        let delta = clock.restart().as_seconds();

        win.rwin.clear(&Color::rgb(180, 215, 255));
        for i in &tilemgr {
            win.rwin.draw(i);
        }
        //win.rwin.draw(&test);
        entitymgr.draw_all(&mut win.rwin);
        win.rwin.draw(&player);
        ui.draw_all(&mut win.rwin);
        win.rwin.display();

        let dead = stat.dead();

        while let Some(e) = win.rwin.poll_event() {
            match e {
                Event::KeyPressed { code: Key::Escape, .. }
                        => break 'mainl,
                Event::MouseButtonPressed { button, x, y }
                    if !dead && button == mouse::Button::Left =>
                        entitymgr.mouse_click(&win.rwin, x, y),
                Event::Closed => break 'mainl,
                Event::Resized { width, height } => {
                    win.on_resize(width, height);
                    ui.on_resize(width, height);
                },
                _ => {},
            }
        }

        if dead {
            ui.set_display_death(true);
        } else {
            match player.update(delta, &win) {
                Some(s) => win.scroll(&s),
                None => {},
            };

            let Vector2i { x: mx, y: my } = win.rwin.mouse_position();
            player.mouse_pos(&win.rwin, mx, my);

            if player.is_in_water(&wg.world()) {
                stat.event(delta, &stats::StatEvent::InWater);
            }
            stat.update(delta);
        }

        ui.update(delta, &stat);

        //println!("{:?}", stat);
        //println!("dead: {}", stat.dead());
    }
}
