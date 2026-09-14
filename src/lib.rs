use x11rb::{connect, connection::Connection};

fn get_root_window() -> u32 {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    screen.root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_window() {
        let root = get_root_window();

        assert_ne!(root, 0);
    }
}
