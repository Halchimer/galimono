use beryllium::*;
use ogl33::*;

pub struct Window(
    SDL,
    GlWindow,
    fn(&Window) -> (),
    fn(&Window) -> (),
    bool, // When true, the window quits when the current mainloop cycle is complete.
);

impl Window {
    pub fn create_window(
        title: &str,
        pos: WindowPosition,
        width: u32,
        height: u32,
        start_func: fn(&Window) -> (),
        mainloop: fn(&Window) -> (),
    ) -> Option<Window> {
        let sdl = SDL::init(InitFlags::Everything).unwrap();

        sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
        sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();

        sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
            .unwrap();
        #[cfg(target_os = "macos")]
        {
            sdl.gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
                .unwrap();
        }

        let win_res = sdl.create_gl_window(title, pos, width, height, WindowFlags::OpenGL);

        match win_res {
            Ok(_) => return Some(Window(sdl, win_res.unwrap(), start_func, mainloop, false)),
            Err(_) => return None,
        }
    }
    pub fn get_glWindow(&self) -> &GlWindow {
        &self.1
    }
    pub fn get_sdl(&self) -> &SDL {
        &self.0
    }
    pub fn init(&mut self) {
        self.2(&self);

        'mainloop: loop {
            'eventloop: for event in self.0.poll_events().iter() {
                match event {
                    Result::Err(_) => continue 'eventloop,
                    _ => (),
                }
            }

            self.3(self);

            if self.4 {
                break 'mainloop;
            }
        }
    }
    pub fn quit(&mut self) {
        self.4 = true;
    }
}
