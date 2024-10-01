
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

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Object], lights: &[Light]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(4, 12, 36);  // Fondo
    }

    let mut final_color = Color::new(0, 0, 0);  // Color inicial

    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal);

        // Diffuse y Specular
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light.intensity;
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

        // Fresnel para reflectividad
        let cos_theta = intersect.normal.dot(&view_dir).abs();
        let fresnel_factor = fresnel_schlick(cos_theta, intersect.material.reflectivity);

        // Transparencia y reflectividad ajustados con Fresnel
        let reflectivity = intersect.material.reflectivity * fresnel_factor;
        let transparency = intersect.material.transparency * (1.0 - fresnel_factor);

        final_color = final_color + diffuse * (1.0 - reflectivity) + specular * reflectivity;

    }

    final_color
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 2.0;  // Cambia el FOV para ampliar la vista
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights);

            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 200;  // Reducir la resolución
    let framebuffer_height = 150;  // Reducir la resolución

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
        0.9,
        0.5,
    );

    // Carga las texturas
    let grass_texture = Texture::load_from_file("src/grass.png");
    let stone_texture = Texture::load_from_file("src/stone.png");
    let wood_texture = Texture::load_from_file("src/wood.png"); // Carga la textura de la madera
    let skybox_texture = Texture::load_from_file("src/skybox.png");  // Carga la textura del cielo

    // Skybox
    let skybox = Object::Cube(Cube {
        min: Vec3::new(-50.0, -50.0, -50.0),
        max: Vec3::new(50.0, 50.0, 50.0),
        material: Material::with_texture(Color::new(135, 206, 235), 1.0, [0.0, 0.0], &skybox_texture, 0.0, 0.0),
        is_skybox: true,  // Marcamos este cubo como el skybox
    });

    // Crea los árboles
    let mut objects: Vec<Object> = Vec::new();
    objects.extend(create_tree(-1.5, -4.0, 3.0, 1.0, &wood_texture, &grass_texture));  // Árbol 1
    objects.extend(create_tree(1.5, -5.0, 4.0, 1.5, &wood_texture, &grass_texture));  // Árbol 2
    objects.extend(create_tree(0.0, -6.0, 2.0, 1.0, &wood_texture, &grass_texture));  // Árbol 3
    objects.extend(create_tree(3.0, -6.0, 3.0, 1.2, &wood_texture, &grass_texture)); // Árbol 4 (más a la derecha)
    objects.extend(create_tree(-3.0, -3.0, 3.5, 1.0, &wood_texture, &grass_texture)); // Árbol 5 (más hacia adelante y a la izquierda)
    objects.extend(create_tree(2.0, -8.0, 2.8, 1.2, &wood_texture, &grass_texture)); // Árbol 6 (un poco más al fondo)

    // Otros objetos en la escena
    objects.push(Object::Cube(Cube {
        min: Vec3::new(-2.0, -1.0, -4.0),
        max: Vec3::new(-1.0, 1.0, -3.0),
        material: Material::with_texture(Color::new(255, 255, 255), 1.0, [0.7, 0.3], &stone_texture, 0.3, 0.0),
        is_skybox: false,
    }));

        // Piedra 2 (más adelante)
    objects.push(Object::Cube(Cube {
        min: Vec3::new(-1.5, -1.0, -3.5),
        max: Vec3::new(-0.5, 0.5, -2.5),
        material: Material::with_texture(Color::new(200, 200, 200), 1.0, [0.7, 0.3], &stone_texture, 0.3, 0.0),  // Pasar referencia
        is_skybox: false,
    }));

    // Piedra 3 (más pequeña, a la derecha)
    objects.push(Object::Cube(Cube {
        min: Vec3::new(2.5, -0.5, -4.5),
        max: Vec3::new(3.0, 0.0, -4.0),
        material: Material::with_texture(Color::new(180, 180, 180), 1.0, [0.7, 0.3], &stone_texture, 0.3, 0.0),  // Pasar referencia
        is_skybox: false,
    }));



    objects.push(Object::Sphere(Sphere {
        center: Vec3::new(1.5, 4.0, -6.0), // Elevamos la esfera para simular el sol
        radius: 0.5,
        material: Material::new(Color::new(255, 255, 0), 1.0, [1.0, 1.0], 0.0, 0.0),  // Esfera brillante
    }));

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),  // Posición de la cámara
        Vec3::new(0.0, 0.0, 0.0),  // Hacia dónde mira la cámara
        Vec3::new(0.0, 1.0, 0.0),  // Arriba
    );

    let mut lights = vec![
        Light::new(Vec3::new(100.0, 100.0, 10.0), Color::new(255, 255, 255), 3.0), // Luz principal
        Light::new(Vec3::new(-50.0, 50.0, 20.0), Color::new(255, 100, 100), 2.0),  // Luz roja adicional
    ];

    let sun = Object::Sphere(Sphere {
        center: lights[0].position,  // El Sol en la misma posición que la luz principal
        radius: 5.0,  // Ajusta el tamaño del sol
        material: Material::new(Color::new(255, 255, 0), 100.0, [1.0, 0.0], 0.0, 0.0),  // Color amarillo brillante
    });
    

    // Agregar el sol a la lista de objetos
    objects.push(sun);


    let mut time_of_day = 0;  // 0 = Día, 1 = Tarde, 2 = Noche
    let rotation_speed = PI / 10.0;

    let zoom_speed = 0.5;  // Velocidad de zoom

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

    // Control de zoom
    if window.is_key_down(Key::Z) {  // Zoom in (acercar)
        camera.zoom_in(zoom_speed);
    }
    if window.is_key_down(Key::X) {  // Zoom out (alejar)
        camera.zoom_out(zoom_speed);
    }

    // Ciclo de día y noche controlado por teclas
    if window.is_key_down(Key::D) {
        time_of_day = (time_of_day + 1) % 3;  // Cambia entre Día, Tarde y Noche
    }

    // Cambios en la iluminación según la hora del día
    match time_of_day {
        0 => {
            lights[0].color = Color::new(255, 255, 255);  // Luz blanca para el día
            lights[0].intensity = 3.0;
        },
        1 => {
            lights[0].color = Color::new(255, 165, 0);  // Luz anaranjada para la tarde
            lights[0].intensity = 2.0;
        },
        2 => {
            lights[0].color = Color::new(0, 0, 139);  // Luz azul para la noche
            lights[0].intensity = 1.0;
        },
        _ => {}
    }

    render(&mut framebuffer, &objects, &camera, &lights);

    window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();

    std::thread::sleep(frame_delay);
}
}

fn create_tree(base_x: f32, base_z: f32, trunk_height: f32, leaves_size: f32, wood_texture: &Texture, grass_texture: &Texture) -> Vec<Object> {
    let mut tree_objects = Vec::new();
    
    // Tronco (cubos verticales)
    for i in 0..(trunk_height as i32) {
        tree_objects.push(Object::Cube(Cube {
            min: Vec3::new(base_x - 0.25, i as f32 - 1.0, base_z - 0.25),
            max: Vec3::new(base_x + 0.25, (i as f32) + 0.25, base_z + 0.25),
            material: Material::with_texture(Color::new(139, 69, 19), 1.0, [0.7, 0.3], &wood_texture.clone(), 0.1, 0.0),  // Clonamos la textura
            is_skybox: false,
        }));
    }

    // Hojas (cubos grandes encima del tronco)
    let leaves_base_y = trunk_height - 0.5;  // Altura donde empiezan las hojas
    tree_objects.push(Object::Cube(Cube {
        min: Vec3::new(base_x - leaves_size, leaves_base_y, base_z - leaves_size),
        max: Vec3::new(base_x + leaves_size, leaves_base_y + leaves_size, base_z + leaves_size),
        material: Material::with_texture(Color::new(34, 139, 34), 1.0, [0.7, 0.3], &grass_texture.clone(), 0.1, 0.0), // Clonamos la textura
        is_skybox: false,
    }));

    tree_objects
}

fn fresnel_schlick(cos_theta: f32, reflectivity: f32) -> f32 {
    let r0 = (1.0 - reflectivity) / (1.0 + reflectivity);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
}
