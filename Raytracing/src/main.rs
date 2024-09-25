
mod framebuffer; 
mod ray_intersect;
mod sphere;
mod color;
mod camera;
mod light;
mod material;
mod cube;  // New cube module
mod texture; // New texture module

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec3, normalize};
use std::time::Duration;
use std::f32::consts::PI;

use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::cube::Cube;  // New cube import
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::texture::Texture;  // New texture import

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        match self {
            Object::Sphere(sphere) => sphere.ray_intersect(ray_origin, ray_direction),
            Object::Cube(cube) => cube.ray_intersect(ray_origin, ray_direction),
        }
    }
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Object], light: &Light) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut hit_object: Option<&Object> = None; // Referencia al objeto intersectado

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
            hit_object = Some(object);  // Guarda referencia al objeto que fue intersectado
        }
    }

    if !intersect.is_intersecting {
        return Color::new(4, 12, 36); // Color de fondo
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let mut diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light.intensity;

    if let Some(object) = hit_object {
        match object {
            Object::Cube(cube) => {
                if let Some(texture) = intersect.material.texture {
                    let (u, v) = cube.get_uv(&intersect.point);
                    let tex_color = texture.get_color(u, v);
                    diffuse = tex_color * intersect.material.albedo[0] * diffuse_intensity * light.intensity;
                }
            },
            Object::Sphere(_) => {
                // Manejamos esferas aquí si fuera necesario
            },
        }
    }

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

    diffuse + specular
}


pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 400;  // Reducir la resolución
    let framebuffer_height = 300;  // Reducir la resolución
    
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Mini Minecraft",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let rubber = Material::new(
        Color::new(80, 0, 0),
        1.0,
        [0.9, 0.1],
    );

    let ivory = Material::new(
        Color::new(100, 100, 80),
        50.0,
        [0.6, 0.3],
    );

    let grass_texture = Texture::load_from_file("grass.png");
    let stone_texture = Texture::load_from_file("stone.png");

    let objects = [
        Object::Cube(Cube {
            min: Vec3::new(-2.0, -1.0, -4.0),
            max: Vec3::new(-1.0, 1.0, -3.0),
            material: Material::with_texture(Color::new(255, 255, 255), 1.0, [0.7, 0.3], stone_texture),
        }),
        Object::Cube(Cube {
            min: Vec3::new(0.0, -1.0, -4.0),
            max: Vec3::new(1.0, 1.0, -3.0),
            material: Material::with_texture(Color::new(255, 255, 255), 1.0, [0.7, 0.3], grass_texture),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(1.5, 0.0, -5.0),
            radius: 0.5,
            material: rubber,
        }),
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(
        Vec3::new(100.0, 100.0, 10.0),
        Color::new(255, 255, 255),
        1.0
    );

    let rotation_speed = PI / 10.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0); 
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        render(&mut framebuffer, &objects, &camera, &light);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
