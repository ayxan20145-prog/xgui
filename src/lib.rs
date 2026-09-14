use x11rb::{
    COPY_DEPTH_FROM_PARENT, connect,
    connection::Connection,
    protocol::xproto::{ConnectionExt, CreateWindowAux, EventMask, WindowClass},
};

pub fn window(x: i16, y: i16, width: u16, height: u16) {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let window = conn.generate_id().unwrap();

    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        window,
        screen.root,
        x,
        y,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::EXPOSURE | EventMask::KEY_PRESS | EventMask::BUTTON_PRESS),
    )
    .unwrap();

    conn.map_window(window).unwrap();

    println!("Window ID: {}", window);

    conn.flush().unwrap();

    loop {
        let event = conn.wait_for_event().unwrap();
        println!("{:?}", event);
    }
}
