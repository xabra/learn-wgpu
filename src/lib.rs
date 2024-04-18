// use tracing::info;
// #[cfg(target_arch = "wasm32")]
// use tracing_subscriber::fmt::format::Pretty;
// #[cfg(target_arch = "wasm32")]
// use tracing_subscriber::fmt::time::UtcTime;
// #[cfg(target_arch = "wasm32")]
// use tracing_subscriber::prelude::*;
// #[cfg(target_arch = "wasm32")]
// use tracing_web::{performance_layer, MakeConsoleWriter};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

             } else {
                // tracing_subscriber::fmt::init()
        }
    }
    log::warn!("here");
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body()?; //.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            _ => {}
        },
        _ => (),
    });
}
