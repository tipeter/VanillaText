mod win;
mod page;

use std::env::Args;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::{ GtkApplicationExt };

use gio::{
    ApplicationExt, ApplicationExtManual,
    ApplicationFlags, ActionMapExt, SimpleActionExt
};

use self::win::{
    Window, WindowExtend, Windows, WindowsExtend
};

fn init_actions(app: &gtk::Application, wins: &Windows) {
    let new_window_action = gio::SimpleAction::new("new_window", None);
    {
        let app = app.clone();
        let wins = wins.clone();
        new_window_action.connect_activate(move |_, _| {
            let w = Window::create(&app, wins.clone());
            w.init();
        });
    }

    let quit_action = gio::SimpleAction::new("quit", None);
    {
        let app = app.clone();
        quit_action.connect_activate(move |_, _| {
            app.quit();
        });
    }

    app.add_action(&new_window_action);
    app.add_action(&quit_action);
}

fn init_accels(app: &gtk::Application) {
    //app.add_accelerator("<Ctrl>q", "app.quit", None);
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);

    //app.add_accelerator("<Ctrl>n", "app.new_window", None);
    app.set_accels_for_action("app.new_window", &["<Ctrl>n"]);

    //app.add_accelerator("<Ctrl>o", "win.open", None);
    app.set_accels_for_action("win.open", &["<Ctrl>o"]);

    //app.add_accelerator("<Ctrl>s", "win.save", None);
    app.set_accels_for_action("win.save", &["<Ctrl>s"]);

    //app.add_accelerator("<Shift><Ctrl>s", "win.saveas", None);
    app.set_accels_for_action("win.saveas", &["<Shift><Ctrl>s"]);

    //app.add_accelerator("<Ctrl>w", "win.close_tab", None);
    app.set_accels_for_action("win.close_tab", &["<Ctrl>w"]);

    //app.add_accelerator("<Ctrl>t", "win.new_tab", None);
    app.set_accels_for_action("win.new_tab", &["<Ctrl>t"]);

    //app.add_accelerator("<Ctrl>a", "win.selectall", None);
    app.set_accels_for_action("win.selectall", &["<Ctrl>a"]);

    //app.add_accelerator("<Ctrl>c", "win.copy", None);
    app.set_accels_for_action("win.copy", &["<Ctrl>c"]);

    //app.add_accelerator("<Ctrl>v", "win.paste", None);
    app.set_accels_for_action("win.paste", &["<Ctrl>v"]);

    //app.add_accelerator("<Ctrl>x", "win.cut", None);
    app.set_accels_for_action("win.cut", &["<Ctrl>x"]);
}

fn run(args: Args) {
    match gtk::Application::new("com.github.koji-m.vanilla_text", ApplicationFlags::HANDLES_OPEN) {
        Ok(app) => {
            let wins = Rc::new(RefCell::new(Vec::<Window>::new()));

            {
                let wins = wins.clone();
                app.connect_startup(move |app| {
                    init_actions(app, &wins);
                    init_accels(app);
                    let builder = gtk::Builder::new_from_file(Path::new("ui/menu.ui"));

                    let app_menu: gio::Menu = builder.get_object("app_menu").unwrap();
                    app.set_app_menu(&app_menu);

                    let menu_bar: gio::Menu = builder.get_object("menu_bar").unwrap();
                    app.set_menubar(&menu_bar);
                });
            }

            {
                let wins = wins.clone();
                app.connect_activate(move |app| {
                    let w = Window::create(app, wins.clone());
                    w.init();
                });
            }

            {
                let wins = wins.clone();
                app.connect_open(move |app, files, _| {
                    let w = Window::create(app, wins.clone());
                    for file in files {
                        if let Some(p) = wins.get_page(&file) {
                            w.present(p);
                        } else {
                            w.open(&file, wins.clone(), false);
                        }
                    }
                    w.init();
                });
            }


            let args: Vec<String> = args.collect();
            //let argv: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

            app.run(&args);
        },

        Err(_) => {
            println!("Application run error");
        }
    };
}


fn main() {
    run(std::env::args());
}
