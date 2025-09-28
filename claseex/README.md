# Diorama Raytraced Minecraft - Proyecto de Gráficas

## 🌟 Descripción del Proyecto

Este proyecto implementa un diorama interactivo inspirado en Minecraft utilizando técnicas avanzadas de raytracing en tiempo real. La aplicación está desarrollada en Rust usando la biblioteca Raylib y presenta un mundo en miniatura compuesto por diferentes tipos de bloques con propiedades físicas realistas de materiales.

El motor de raytracing personalizado simula el comportamiento real de la luz, manejando múltiples fenómenos ópticos complejos incluyendo reflexiones, refracciones y dispersión difusa. El sistema utiliza técnicas de optimización para mantener un rendimiento aceptable durante la interacción en tiempo real.

## ✨ Características Implementadas

### 🏆 Características Principales Implementadas

- **Rotación de Diorama y Control de Cámara**
  - ✅ **Click y Arrastrar**: Rota la cámara alrededor del diorama para observarlo desde diferentes ángulos
  - ✅ **Rueda del Mouse**: Controla el zoom de acercamiento y alejamiento (2.0 - 40.0 unidades)
  - ✅ **Teclas WASD**: Control manual directo del movimiento de la cámara en todas las direcciones
  - ✅ **Tecla SPACE**: Activa y desactiva el modo de rotación automática continua

- **Materiales Diferentes Implementados**
  1. **Hierro** - Material reflectivo metálico
  2. **Diamante** - Material con alta reflectividad y emisión
  3. **Césped** - Material difuso orgánico
  4. **Tierra** - Material mate completamente difuso
  5. **Agua** - Material transparente con refracción (índice 1.33)
  6. **Vidrio** - Material transparente con refracción (índice 1.52)

- **Refracción Implementada**
  - ✅ **Agua**: Refracción realista con índice 1.33
  - ✅ **Vidrio**: Refracción con índice 1.52
  - ✅ Implementación completa de Ley de Snell
  - ✅ Reflexión total interna
  - ✅ Aproximación de Schlick para reflectancia

- **Reflexión Implementada**
  - ✅ **Hierro**: Alta reflectividad metálica
  - ✅ **Diamante**: Reflectividad premium con emisión
  - ✅ Raytracing recursivo para múltiples rebotes

- **Skybox Implementado**
  - ✅ Skybox procedural con gradiente vertical
  - ✅ Sol con resplandor dinámico
  - ✅ Colores del cielo realistas
  - ✅ Integrado completamente en el raytracing

### 🚀 Características Técnicas Avanzadas

#### Motor de Raytracing Completo
- **Estructuras de rayos** con origen y dirección
- **Sistema de intersección** para cubos y esferas
- **Materiales físicamente basados** con propiedades realistas
- **Raytracing recursivo** hasta 10 niveles de profundidad
- **Anti-aliasing** con 4 samples por pixel

#### Optimización de Performance
- **Paralelización con Rayon** - Utiliza todos los cores de la CPU
- **Diorama reducido** - Optimizado para raytracing en tiempo real
- **Renderizado adaptativo** - Actualización cada 0.1 segundos
- **Resolución optimizada** - 800x600 para mejor performance

#### Sistema de Materiales Avanzado
```rust
pub struct Material {
    pub albedo: Vector3,           // Color base del material
    pub specular: f32,            // Reflectividad especular
    pub transparency: f32,        // Nivel de transparencia
    pub reflectivity: f32,        // Capacidad de reflexión
    pub refraction_index: f32,    // Índice de refracción
    pub emission: Vector3,        // Emisión de luz propia
}
```

#### Efectos Ópticos Realistas
El motor de raytracing simula múltiples fenómenos ópticos complejos:
- **Reflexiones** calculadas con vectores matemáticamente precisos
- **Refracciones** implementando la Ley de Snell con índices reales de materiales
- **Aproximación de Schlick** para calcular reflectancia precisa
- **Reflexión total interna** cuando los rayos no pueden refractarse

## 🏗️ Composición del Diorama

El diorama presenta una colección cuidadosamente diseñada de bloques que demuestran las capacidades del motor:
- **Bloques de hierro** con reflexiones metálicas brillantes
- **Diamantes** con efectos de refracción y brillo intenso
- **Bloques de césped** con texturas realistas multicara
- **Bloques de tierra** con textura natural mate
- **Agua** que demuestra transparencia y refracción realista
- **Vidrio** con alta transparencia y efectos ópticos sutiles

## 🚀 Optimizaciones de Rendimiento

El proyecto implementa múltiples estrategias de optimización para mantener la interactividad:
- **Renderizado adaptativo** que ajusta la frecuencia según la actividad del usuario
- **Calidad variable** que permite balance entre fidelidad visual y rendimiento
- **Detección de interacción** para aumentar frecuencia durante uso activo
- **Intervalos inteligentes** que reducen carga durante períodos de inactividad

## 🏆 Logros Técnicos

Este proyecto representa una implementación completa de un motor de raytracing funcional que demuestra comprensión profunda de los principios ópticos y matemáticos. La integración exitosa de múltiples sistemas complejos incluyendo renderizado, materiales, texturas y controles de usuario resulta en una experiencia interactiva fluida y visualmente impresionante que simula con precisión el comportamiento de la luz en un entorno tridimensional.

## 🏗️ Arquitectura del Proyecto

### Estructura de Archivos
```
src/
├── main.rs           # Loop principal y lógica de renderizado
├── ray.rs            # Estructuras de rayos e intersección
├── raytracer.rs      # Motor principal de raytracing
├── materials.rs      # Sistema de materiales físicos
├── cubes.rs          # Definición del mundo y objetos
├── camera.rs         # Sistema de cámara orbital
├── skybox.rs         # Skybox procedural
└── math_utils.rs     # Utilidades matemáticas
```

### Dependencias Clave
```toml
raylib = "5.5.1"    # Motor gráfico base
rayon = "1.7"       # Paralelización de CPU
fastrand = "2.0"    # Generación de números aleatorios
nalgebra = "0.29"   # Matemáticas vectoriales
```

## 🎯 Estructura del Diorama

### Composición del Mundo
- **Terraza Natural Escalonada** - Base de tierra en círculos concéntricos (3 niveles)
- **Castillo de Hierro** - Fortaleza completa con torres y murallas reflectivas
- **Templo de Diamante** - Estructura en cruz con pirámide brillante y efectos de emisión
- **Lago y Cascada** - Sistema acuático en forma de L con cascada vertical
- **Observatorio de Vidrio** - Invernadero transparente con cúpula y torre de observación
- **Puentes y Caminos** - Conexiones entre estructuras con materiales diversos
- **Elementos Flotantes** - Cristales suspendidos y efectos atmosféricos

### Propiedades Físicas por Material

| Material | Albedo | Transparencia | Reflectividad | Índice Refracción | Emisión |
|----------|--------|---------------|---------------|-------------------|---------|
| **Hierro** | Gris metálico | 0.0 | 0.9 | 1.0 | No |
| **Diamante** | Azul brillante | 0.1 | 0.95 | 2.42 | Sí (azul) |
| **Césped** | Verde natural | 0.0 | 0.0 | 1.0 | No |
| **Tierra** | Marrón mate | 0.0 | 0.0 | 1.0 | No |
| **Agua** | Azul transparente | 0.8 | 0.2 | 1.33 | No |
| **Vidrio** | Casi transparente | 0.95 | 0.04 | 1.52 | No |

## 🔬 Implementación Técnica

### Algoritmo de Raytracing
1. **Generación de Rayos**: Desde la cámara hacia cada pixel
2. **Intersección**: Cálculo con todos los objetos en la escena
3. **Sombreado**: Basado en propiedades del material
4. **Recursión**: Para reflexiones y refracciones
5. **Combinación**: De colores con atenuación física

### Efectos Físicos Implementados

#### Reflexiones
- Cálculo del vector reflejado: `reflected = incident - 2 * dot(incident, normal) * normal`
- Atenuación basada en reflectividad del material
- Múltiples rebotes hasta profundidad máxima

#### Refracciones
- **Ley de Snell**: `n1 * sin(θ1) = n2 * sin(θ2)`
- **Reflexión total interna** cuando no hay refracción posible
- **Aproximación de Schlick** para reflectancia dependiente del ángulo

#### Anti-aliasing
- 4 samples por pixel con desplazamiento aleatorio
- Promediado de colores para suavizar bordes
- Corrección gamma para visualización correcta

## 🚀 Compilación y Ejecución

### Prerrequisitos
```bash
# Rust y Cargo instalados
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencias del sistema para Raylib (macOS)
brew install cmake
```

### Compilación
```bash
cd claseex
cargo build --release
```

### Ejecución
```bash
cargo run --release
```

## 📊 Performance y Optimización

### Métricas de Rendimiento
- **Resolución**: 800x600 (optimizada para raytracing)
- **Samples por pixel**: 4 (balance calidad/velocidad)
- **Profundidad máxima**: 10 rebotes
- **Actualización**: 10 FPS de raytracing, 30 FPS de UI
- **Paralelización**: Utiliza todos los cores disponibles

### Optimizaciones Implementadas
1. **Diorama artísticamente balanceado**: Diseño creativo que muestra todos los materiales de forma espectacular
2. **Paralelización por chunks**: Distribución inteligente del trabajo entre cores
3. **Renderizado adaptativo**: Ajusta la frecuencia según la interacción del usuario
4. **Estructuras eficientes**: Minimización de allocaciones y uso óptimo de memoria
5. **Composición visual estratégica**: Elementos posicionados para máximo impacto visual del raytracing

## 🔮 Características Adicionales

- **Motor de raytracing desde cero** - Implementación completa propia
- **Interfaz informativa** - UI con controles y estadísticas en pantalla
- **Código bien documentado** - Comentarios extensivos en español
- **Arquitectura modular** - Fácil extensión y mantenimiento
- **Composición visual** - Cada ángulo de cámara ofrece una vista espectacular

## � Aspectos Educativos Cubiertos

### Conceptos de Raytracing
- Generación y propagación de rayos
- Algoritmos de intersección geométrica
- Modelos de iluminación física
- Reflexión y refracción de la luz
- Anti-aliasing y muestreo

### Programación Gráfica Avanzada
- Paralelización de algoritmos gráficos
- Optimización de performance en tiempo real
- Estructuras de datos para gráficos
- Integración de múltiples efectos visuales



---

**Desarrollado para el curso de Gráficas por Computadora**  
*Implementación completa de raytracing con efectos físicamente correctos*
