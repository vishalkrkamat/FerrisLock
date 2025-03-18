use wayland_client::{
    protocol::{wl_compositor, wl_registry, wl_surface},
    Connection, Dispatch, QueueHandle,
};

// This struct represents the state of our Wayland client.
struct AppData {
    compositor: Option<wl_compositor::WlCompositor>,
}

// Implement `Dispatch<WlRegistry, ()>` for handling registry events
impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        regis: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(), // ✅ Corrected type
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[{}] {} (v{})", name, interface, version);
            if interface == "wl_compositor" {
                state.compositor = Some(regis.bind::<wl_compositor::WlCompositor, _, _>(
                    name,
                    version.min(4),
                    qh,
                    (),
                ));
            }
        }
    }
}

// Implement `Dispatch<WlCompositor, ()>` to handle `wl_compositor` events.
impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // No events to handle for wl_compositor
    }
}

// Implement Dispatch for WlSurface
impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // Handle surface events (if needed)
    }
}

fn main() {
    // Step 1: Connect to Wayland
    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland server");

    // Step 2: Retrieve the display
    let display = conn.display();

    // Step 3: Create an event queue and get its handle
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    // Step 4: Request the global registry
    let registry = display.get_registry(&qh, ());
    println!("{registry:?}");

    // Step 5: Wait for the Wayland server to respond and process registry events
    println!("Advertised globals:");
    let mut state = AppData { compositor: None };
    event_queue.roundtrip(&mut state).unwrap();

    if let Some(compositor) = state.compositor {
        let surface = compositor.create_surface(&qh, ());
        println!("Created a Wayland surface!{surface:?}");
    } else {
        println!("Failed to bind wl_compositor");
    }
}
