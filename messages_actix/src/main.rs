use messages_actix::MessageApp;

fn main() {
    //setup & start logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //setup & start app
    let app = MessageApp::new(8080);
    app.run();
}
