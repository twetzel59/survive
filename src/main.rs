extern crate sfml;
extern crate survive;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::{Clock, Vector2i, Vector2u};
use survive::*;

fn main() {
    let mut win = GameWindow::new();
    win.rwin.clear(&Color::white());
    win.rwin.display();

    let res = Resources::new();

    'outer: loop {
        let Vector2u { x: width, y: height } = win.rwin.size();

        let wg = Worldgen::new();
        //let test = Sprite::with_texture(&wg.textures[9]);

        let tilemgr = TileManager::new(wg.textures());

        let mut day = DayNight::new();
        day.on_resize(width, height);

        let mut entitymgr = EntityManager::new();
        //entitymgr.add(entities::deciduous_tree::DeciduousTree::new(&res));
        //entitymgr.add(Box::new(entities::deciduous_tree::DeciduousTree::new(&res)));
        entitymgr.add(entities::bonfire::Bonfire::with_position2f(&res, 128., 128.));
        plants::generate_plants(&res, wg.world(), &mut entitymgr);

        let mut player = Player::new(&res);

        let mut stat = stats::Stats::new();

        let mut inv = Inventory::new();

        let mut ui = UiManager::new(&res);
        ui.on_resize(width, height);

        let mut dead = stat.dead();
        let mut restart_clock = Clock::start();

        let mut clock = Clock::start();
        'mainl: loop {
            let delta = clock.restart().as_seconds();

            win.rwin.clear(&Color::rgb(180, 215, 255));
            for i in &tilemgr {
                win.rwin.draw(i);
            }
            //win.rwin.draw(&test);
            entitymgr.draw_all(&mut win.rwin, true);

            win.rwin.draw(&day);

            entitymgr.draw_all(&mut win.rwin, false);

            win.rwin.draw(&player);

            ui.draw_all(&mut win.rwin);

            win.rwin.display();

            if !dead && stat.dead() {
                dead = true;
                restart_clock.restart();
            }

            while let Some(e) = win.rwin.poll_event() {
                match e {
                    Event::KeyPressed { code: Key::Escape, .. }
                            => break 'outer,
                    Event::MouseButtonPressed { button, x, y }
                        if !dead && button == mouse::Button::Left =>
                            entitymgr.click(&player.position(), &mut inv, &win.rwin, x, y),
                    Event::Closed => break 'outer,
                    Event::Resized { width, height } => {
                        win.on_resize(width, height);
                        day.on_resize(width, height);
                        ui.on_resize(width, height);
                    },
                    _ => {},
                }
            }

            if dead {
                ui.set_display_death(true);

                //println!("restart_clock: {}", restart_clock.elapsed_time().as_seconds());
                if restart_clock.elapsed_time().as_milliseconds() > 2000 {
                    let mut view = win.rwin.view().to_owned();
                    view.set_center2f(0., 0.);
                    win.rwin.set_view(&view);

                    continue 'outer;
                }
            } else {
                day.update(delta);

                match player.update(delta, &win) {
                    Some(s) => win.scroll(&s),
                    None => {},
                };

                let Vector2i { x: mx, y: my } = win.rwin.mouse_position();
                player.mouse_pos(&win.rwin, mx, my);

                entitymgr.update(delta);

                if player.is_in_water(&wg.world()) {
                    stat.event(&stats::StatEvent::InWater { delta });
                }

                if entitymgr.near_bonfire(&player.position()) {
                    stat.event(&stats::StatEvent::NearBonfire { delta });
                }

                stat.update(delta, day.daylight());
            }

            ui.update(delta, day.time(), &stat, &inv);

            //println!("daylight: {}", day.daylight());
            //println!("{:?}", stat);
            //println!("dead: {}", stat.dead());
            //println!("Wood count: {}", inv.items()[0]);
            //println!("current day: {:?}", day);
        }
    }
}
