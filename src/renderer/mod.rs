use crate::window;
use beryllium;

mod camera;

struct Renderer(window::Window);

impl Renderer {
    pub fn new(
        start_func: fn(&window::Window) -> (),
        mainloop: fn(&window::Window) -> (),
    ) -> Option<Self> {
        Some(Self(
            window::Window::create_window(
                "title",
                beryllium::WindowPosition::Centered,
                720,
                480,
                start_func,
                mainloop,
            )
            .unwrap(),
        ))
    }
}
