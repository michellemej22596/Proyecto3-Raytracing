# Proyecto3-Raytracing
Michelle Mejía 22596


MINI MINECRAFT 

Este proyecto implementa un motor de raytracing en Rust que simula una escena estilo Minecraft. El motor incluye características como manejo de texturas, múltiples fuentes de luz, efectos de fresnel, y un ciclo de día y noche. A lo largo del proyecto se han implementado varios objetos y elementos para hacer la simulación lo más atractiva posible.

Requisitos del Proyecto
El proyecto está diseñado para cumplir con los siguientes requisitos:

Complejidad de la escena - Se han implementado varios árboles de bloques con troncos y hojas, además de cubos y esferas que simulan materiales como piedra, grama y madera.

Atractivo visual - Se utilizan texturas y efectos de iluminación avanzados, como el fresnel y materiales emisivos, para mejorar el realismo y la estética de la escena.

Performance del raytracer - El proyecto fue optimizado para correr con buena performance, incluyendo un framebuffer de menor resolución para mejorar la velocidad de procesamiento.

Materiales con texturas - Se han añadido diferentes materiales, cada uno con su propia textura, albedo, especularidad, transparencia y reflectividad. Esto incluye materiales como:

Piedra
Grama
Madera
Skybox 

Ciclo de día y noche - Se ha implementado un ciclo de iluminación que permite cambiar la hora del día (día, tarde, noche) presionando teclas específicas. Este ciclo cambia tanto la posición como el color de las luces.

Materiales emisivos - Algunos objetos como la esfera pueden representar una fuente de luz emisiva, afectando la iluminación de la escena.

Estructura del Código
El proyecto está dividido en los siguientes módulos:

camera.rs - Controla la cámara y su movimiento en la escena.
color.rs - Define las operaciones y manipulaciones de colores.
framebuffer.rs - Controla el framebuffer, donde se renderiza la imagen final.
light.rs - Define las fuentes de luz en la escena, incluyendo intensidad y color.
material.rs - Gestiona los materiales de los objetos, incluyendo texturas, transparencia y reflectividad.
ray_intersect.rs - Implementa las intersecciones de rayos con los objetos en la escena.
sphere.rs - Maneja las esferas en la escena, incluyendo su geometría y comportamiento con la luz.
cube.rs - Maneja los cubos en la escena, que se utilizan para construir los árboles y otros elementos.
texture.rs - Gestiona las texturas de los objetos.
skybox.rs - (En desarrollo) Simula el entorno de cielo alrededor de la escena.

Instalación y Uso
Para correr el proyecto:

Asegúrate de tener Rust instalado en tu máquina. Puedes instalarlo desde rustup.rs.
Clona este repositorio en tu computadora.
Navega hasta el directorio del proyecto.
Ejecuta el comando cargo run --release para compilar y ejecutar el proyecto en modo release.

cargo run --release

Controles
Teclas de dirección - Orbitan la cámara alrededor de la escena.
Tecla D - Cambia la iluminación a modo "Día".
Tecla T - Cambia la iluminación a modo "Tarde".
Tecla N - Cambia la iluminación a modo "Noche".

Futuras Mejoras
Finalizar la implementación del skybox.
Mejorar las texturas de los bloques y optimizar el renderizado.
Implementar soporte para animación de texturas como agua o fuego.
Añadir más objetos y detalles a la escena.

Licencia
Este proyecto está bajo la licencia MIT. Puedes modificar y distribuir el código libremente, siempre y cuando des crédito a los autores originales.
