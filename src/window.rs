use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;

pub struct Window {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    // Autres champs nécessaires pour wgpu, etc.
}


impl Window {
    pub fn new() -> Self {
        // Initialisation de l'événement loop et de la fenêtre avec winit
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Initialisation de wgpu, etc.
        // Vous pouvez ajouter ici la logique d'initialisation de wgpu

        Self {
            event_loop,
            window,
            // Initialisation des autres champs nécessaires
        }
    }

    pub fn run(&self) {
        let window = self.window.clone();
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                },
                // Gérer d'autres événements de fenêtre si nécessaire
                _ => (),
            }
        });
    }

    // Méthodes supplémentaires pour gérer la fenêtre, par exemple obtenir la taille, etc.
}
