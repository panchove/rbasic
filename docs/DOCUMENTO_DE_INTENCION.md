# RBASIC

## Documento de Intención del Proyecto

### Versión 0.1

## 1. Introducción

RBASIC es una iniciativa para diseñar y desarrollar un lenguaje de programación moderno inspirado en la simplicidad histórica de BASIC, incorporando principios contemporáneos de seguridad, rendimiento, mantenibilidad y compilación nativa.

El proyecto busca recuperar la facilidad de lectura y aprendizaje que caracterizó a BASIC, eliminando las limitaciones que históricamente impidieron su adopción en sistemas de software modernos.

RBASIC no pretende ser una recreación de BASIC clásico, sino una reinterpretación moderna orientada a la construcción de software seguro, mantenible y eficiente.

---

## 2. Visión

Crear un lenguaje de programación que combine:

* La legibilidad de BASIC.
* La seguridad de Rust.
* La simplicidad operacional de Go.
* La cercanía al hardware de C.
* La productividad de los lenguajes modernos.

RBASIC deberá ser capaz de evolucionar hasta convertirse en un lenguaje autocontenido capaz de compilar su propio compilador.

---

## 3. Objetivos Estratégicos

### Objetivo Principal

Desarrollar un ecosistema completo compuesto por:

* Lenguaje RBASIC.
* Compilador RBASIC.
* Herramientas de desarrollo.
* Sistema de paquetes.
* Biblioteca estándar.
* Documentación oficial.

### Objetivos Técnicos

* Compilación nativa.
* Tipado estático.
* Seguridad de memoria.
* Manejo explícito de errores.
* Portabilidad multiplataforma.
* Capacidad de interoperar con código C.
* Soporte futuro para WebAssembly.
* Capacidad futura de self-hosting.

---

## 4. Principios Fundamentales

### 4.1 Seguridad por Defecto

Todo programa RBASIC debe ser seguro por defecto.

El programador deberá optar explícitamente por operaciones inseguras mediante mecanismos controlados.

### 4.2 Legibilidad Primero

El código debe ser comprensible antes que ingenioso.

Las construcciones del lenguaje favorecerán claridad sobre complejidad.

### 4.3 Simplicidad Evolutiva

El núcleo del lenguaje deberá mantenerse pequeño y estable.

Las características avanzadas deberán construirse sobre una base mínima y consistente.

### 4.4 Cero Coste Cuando Sea Posible

Las abstracciones del lenguaje no deben introducir penalizaciones innecesarias en tiempo de ejecución.

---

## 5. Características Iniciales del Lenguaje

### Sistema de Tipos

Tipos primitivos:

* bool
* i8
* i16
* i32
* i64
* u8
* u16
* u32
* u64
* f32
* f64
* string

### Variables

```basic
LET name = "RBASIC"
LET MUT counter = 0
```

### Funciones

```basic
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION
```

### Control de Flujo

```basic
IF condition THEN
END IF

WHILE condition
END WHILE

FOR item IN items
END FOR
```

### Colecciones

```basic
Array<T>
```

### Referencias Seguras

```basic
Ref<T>
MutRef<T>
```

### Valores Opcionales

```basic
Optional<T>
```

### Resultados y Errores

```basic
Result<T, E>
```

---

## 6. Exclusiones Iniciales

Para mantener el alcance controlado, la primera versión no incluirá:

* Herencia clásica.
* Clases complejas.
* Recolección de basura.
* Macros avanzadas.
* Reflexión.
* Metaprogramación.
* Concurrencia avanzada.
* Generics complejos.

Estas características serán evaluadas posteriormente.

---

## 7. Arquitectura del Compilador

### Fase Inicial

El primer compilador será desarrollado en Rust.

Arquitectura:

```text
Lexer
 ↓
Parser
 ↓
AST
 ↓
Análisis Semántico
 ↓
AST Tipado
 ↓
Generación de Código
```

### Backend Inicial

RBASIC → Rust → rustc (LLVM)

Esta estrategia permitirá:

* Aprovechar el infraestructura de optimización de LLVM vía rustc.
* Mantener el compilador principal en un único lenguaje (Rust).
* Facilitar el bootstrapping futuro al reescribir el codegen en RBASIC.
* Portabilidad a todas las plataformas que soporta Rust.

---

## 8. Estrategia de Evolución

### Etapa 1

RBASIC mínimo funcional.

### Etapa 2

Biblioteca estándar básica.

### Etapa 3

Compilador capaz de procesar proyectos reales.

### Etapa 4

Reescritura progresiva del compilador en RBASIC.

### Etapa 5

Autocompilación (Self-Hosting).

### Etapa 6

Backend nativo basado en LLVM o Cranelift.

---

## 9. Meta de Largo Plazo

Convertir RBASIC en un lenguaje moderno, seguro y sostenible que pueda utilizarse para:

* Aplicaciones de consola.
* Herramientas de automatización.
* Sistemas embebidos.
* Servicios backend.
* Aplicaciones multiplataforma.
* Compiladores y herramientas de desarrollo.
* Automatización de suites ofimáticas vía **RBA (RBasic for Applications)**, como reemplazo moderno de VBA en LibreOffice, FreeOffice y OnlyOffice.

### RBA — RBasic for Applications

RBA será una variante embebida de RBASIC diseñada para actuar como motor de scripting en suites ofimáticas de código abierto:

* **LibreOffice** — Integración vía UNO API.
* **FreeOffice** — Integración vía su API nativa.
* **OnlyOffice** — Integración vía sus mecanismos de plugin/scripting.

RBA compartirá el mismo núcleo lingüístico que RBASIC, pero incluirá una biblioteca estándar orientada a la manipulación de documentos, hojas de cálculo, presentaciones y automatización de tareas ofimáticas. El objetivo es ofrecer una alternativa moderna, segura y multiplataforma a VBA (Visual Basic for Applications), eliminando la dependencia de entornos Windows y Microsoft Office.

---

## 10. Declaración Final

RBASIC nace con la intención de demostrar que un lenguaje puede ser simultáneamente simple de leer, seguro de ejecutar y suficientemente potente para construir sistemas modernos.

La simplicidad será una característica permanente del proyecto, no una limitación temporal.

