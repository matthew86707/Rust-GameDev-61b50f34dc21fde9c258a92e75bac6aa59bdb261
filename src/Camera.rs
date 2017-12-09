extern crate glium;
extern crate image;
extern crate nalgebra;

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE : f32 = 1000.0;

pub struct Camera{
	
	pub rotation: nalgebra::Vector3<f32>,
	pub position: nalgebra::Vector3<f32>
}

impl Camera{
	pub fn new() -> Camera{

 let mut translation: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 0.0);

		let mut translation_matrix : nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut scale_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

		Camera{
			
			position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
			rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0)
		}
	}
	


	pub fn translate(&mut self, translation: nalgebra::Vector3<f32>) {
		self.position += translation;
	}

	pub fn rotate(&mut self, rotation: nalgebra::Vector3<f32>) {
		self.rotation += rotation;
	}
	pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
		let mut translation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        translation_matrix[(0, 3)] = self.position[0];
        translation_matrix[(1, 3)] = self.position[1];
        translation_matrix[(2, 3)] = self.position[2];

        use nalgebra::core::Matrix4;

        rotation_matrix = Matrix4::<f32>::from_euler_angles(self.rotation[0], self.rotation[1], 0.0).into();

        return (translation_matrix * rotation_matrix).into();

        //((rotation_matrix_z * rotation_matrix_y * rotation_matrix_x) * (translation_matrix)).into()
	}

pub fn create_projection_matrix(&mut self, fov: f32, screen_size: (u32, u32)) -> nalgebra::Matrix4<f32> {
    let aspect_ratio: f32 = screen_size.0 as f32 / screen_size.1 as f32;
    let y_scale = (1.0 / f32::tan(f32::to_radians(fov / 2.0))) * aspect_ratio;
    let x_scale = y_scale / aspect_ratio;
    let frustum_length = FAR_PLANE - NEAR_PLANE;

    let mut matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    matrix[(0, 0)] = x_scale;
    matrix[(1, 1)] = y_scale;
    matrix[(2, 2)] = -((FAR_PLANE + NEAR_PLANE) / frustum_length);
    matrix[(3, 2)] = -1.0;
    matrix[(2, 3)] = -((2.0 * NEAR_PLANE * FAR_PLANE) / frustum_length);
    matrix[(3, 3)] = 0.0;

    matrix
}
}
