use engine::frame::Frame;
use engine::scene::{CameraProjection, Object, Scene};
use engine::types::{Quat, RotationOrder, Vector2, Vector3};

use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

fn main() {
	let mut scene = Scene::new();

	scene.camera.cframe = Vector3::new(0., 0., 5.).into();
	scene.camera.projection = CameraProjection::Orthographic;

	let pyramid = Object::pyramid();
	scene.objects.push(pyramid);

	let event_loop = EventLoop::new().unwrap();

	let context = Context::new(event_loop.owned_display_handle()).unwrap();
	let mut app = App {
		context,
		state: AppState::Initital,
		scene,
	};

	event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
	event_loop.run_app(&mut app).unwrap();
}

struct App {
	context: Context<OwnedDisplayHandle>,
	state: AppState,
	scene: Scene,
}

enum AppState {
	Initital,
	Suspended {
		window: Rc<Window>,
	},
	Running {
		surface: Surface<OwnedDisplayHandle, Rc<Window>>,
	},
}

impl ApplicationHandler for App {
	fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
		if let StartCause::Init = cause {
			let window_attrs = Window::default_attributes();
			let window = event_loop
				.create_window(window_attrs)
				.expect("Failed to create window");
			self.state = AppState::Suspended {
				window: Rc::new(window),
			}
		}
	}

	fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
		let AppState::Suspended { window } = &mut self.state else {
			unreachable!("Got resumed event while not suspended");
		};
		let mut surface =
			Surface::new(&self.context, window.clone()).expect("Failed to create surface");

		let size = window.inner_size();
		if let (Some(width), Some(height)) =
			(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
		{
			surface.resize(width, height).unwrap();
		}

		self.state = AppState::Running { surface };
	}

	fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
		let AppState::Running { surface } = &mut self.state else {
			unreachable!("Got resumed event while not running");
		};
		let window = surface.window().clone();
		self.state = AppState::Suspended { window };
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		let AppState::Running { surface } = &mut self.state else {
			unreachable!("Got window event while suspended");
		};

		if surface.window().id() != window_id {
			return;
		}

		match event {
			WindowEvent::Resized(size) => {
				if let (Some(width), Some(height)) =
					(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
				{
					surface.resize(width, height).unwrap();
				}
			}
			WindowEvent::RedrawRequested => {
				let time = std::time::SystemTime::now()
					.duration_since(std::time::UNIX_EPOCH)
					.unwrap()
					.as_secs_f64();
				let size = surface.window().inner_size();

				self.scene.camera.screen_dimensions =
					Vector2::new(size.width as f64, size.height as f64);

				let mut frame = Frame::new(size.width as usize, size.height as usize);

				self.scene.objects[0].cframe.rotation =
					Quat::from_euler_angles(-0.3, time, 0., RotationOrder::YXZ);
				self.scene.objects[0].cframe.position = Vector3::y_axis() * f64::sin(time) * 0.1;

				self.scene.render_into(&mut frame);

				let mut buffer = surface.buffer_mut().unwrap();
				let frame_data = frame.get_raw_data();
				for i in 0..frame_data.len() - 1 {
					buffer[i] = frame_data[i].get_compact_rgb();
				}

				buffer.present().unwrap();
				surface.window().request_redraw();
			}
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			_ => (),
		}
	}
}
