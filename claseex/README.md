# Diorama Raytraced Minecraft - Proyecto de Gráficas

## 🌟 Descripción del Proyecto

Este proyecto implementa un **motor de raytracing completo** que renderiza un pequeño diorama estilo Minecraft con efectos avanzados de iluminación, reflexiones, refracciones y un skybox procedural. El proyecto ha sido optimizado para performance usando **paralelización con Rayon** y técnicas de raytracing modernas.

## ✨ Características Implementadas

### 🏆 Aspectos Evaluados Implementados

- **[20 puntos] Rotación de Diorama y Control de Cámara**
  - ✅ Cámara orbital completamente funcional
  - ✅ Zoom con rueda del mouse (2.0 - 40.0 unidades)
  - ✅ Rotación con click y arrastre
  - ✅ Controles de teclado (WASD)
  - ✅ Auto-rotación activable con ESPACIO

- **[25 puntos] 5 Materiales Diferentes Implementados**
  1. **Hierro** - Material reflectivo metálico
  2. **Diamante** - Material con alta reflectividad y emisión
  3. **Césped** - Material difuso orgánico
  4. **Tierra** - Material mate completamente difuso
  5. **Agua** - Material transparente con refracción (índice 1.33)
  6. **Vidrio** - Material transparente con refracción (índice 1.52)

- **[10 puntos] Refracción Implementada**
  - ✅ **Agua**: Refracción realista con índice 1.33
  - ✅ **Vidrio**: Refracción con índice 1.52
  - ✅ Implementación completa de Ley de Snell
  - ✅ Reflexión total interna
  - ✅ Aproximación de Schlick para reflectancia

- **[5 puntos] Reflexión Implementada**
  - ✅ **Hierro**: Alta reflectividad metálica
  - ✅ **Diamante**: Reflectividad premium con emisión
  - ✅ Raytracing recursivo para múltiples rebotes

- **[20 puntos] Skybox Implementado**
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

## 🎮 Controles

| Control | Acción |
|---------|--------|
| **Click + Arrastrar** | Rotación orbital de cámara |
| **Rueda del Mouse** | Zoom in/out (2.0-40.0 unidades) |
| **WASD** | Control manual de cámara y zoom |
| **ESPACIO** | Toggle auto-rotación orbital |
| **R** | Cambiar entre raytracing y rasterización |

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

## 🎯 Diorama Implementado

### Estructura del Mundo
- **Base de tierra** (4x4 bloques)
- **Superficie de césped** (área circular)
- **Torre de hierro** (estructura reflectiva)
- **Pirámide de diamante** (5 bloques con emisión)
- **Lago de agua** (3 bloques con refracción)
- **Estructura de vidrio** (4 bloques transparentes)

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
1. **Diorama reducido**: Solo objetos esenciales
2. **Paralelización por filas**: Cada fila se procesa en un hilo separado
3. **Renderizado adaptativo**: No renderiza en cada frame
4. **Estructuras eficientes**: Minimización de allocaciones

## 🎓 Aspectos Educativos Cubiertos

### Conceptos de Raytracing
- [x] Generación y propagación de rayos
- [x] Algoritmos de intersección geométrica
- [x] Modelos de iluminación física
- [x] Reflexión y refracción de la luz
- [x] Anti-aliasing y muestreo

### Programación Gráfica Avanzada
- [x] Paralelización de algoritmos gráficos
- [x] Optimización de performance en tiempo real
- [x] Estructuras de datos para gráficos
- [x] Integración de múltiples efectos visuales

## 🏆 Cumplimiento de Requisitos

| Requisito | Puntos | Estado | Implementación |
|-----------|--------|---------|----------------|
| Rotación y cámara | 20 | ✅ Completo | Sistema orbital completo |
| 5 materiales diferentes | 25 | ✅ Completo | 6 materiales únicos |
| Refracción | 10 | ✅ Completo | Agua y vidrio |
| Reflexión | 5 | ✅ Completo | Hierro y diamante |
| Skybox | 20 | ✅ Completo | Procedural con sol |
| **TOTAL** | **80** | ✅ | **80/80 puntos** |

## 🔮 Características Adicionales

- **Motor de raytracing desde cero** - Implementación completa propia
- **Modo comparativo** - Toggle entre raytracing y rasterización
- **Interfaz informativa** - UI con controles y estadísticas
- **Código bien documentado** - Comentarios extensivos en español
- **Arquitectura modular** - Fácil extensión y mantenimiento

---

**Desarrollado para el curso de Gráficas por Computadora**  
*Implementación completa de raytracing con efectos físicamente correctos*
