# Kart Racer

Ray caster simple en Rust con estilo de carreras arcade. El juego renderiza un circuito completo, permite manejar un kart, completar vueltas con checkpoints y terminar la carrera al cruzar la meta.

## Como ejecutar

```bash
cargo run
```

## Controles

- `W` / flecha arriba: acelerar
- `S` / flecha abajo: frenar o retroceder
- `A` / flecha izquierda: girar a la izquierda
- `D` / flecha derecha: girar a la derecha
- Mouse: rotacion horizontal de camara
- `Space`: derrape
- `P`: pausar
- `Enter` / `R`: continuar o reiniciar segun la pantalla
- `Backspace`: volver al menu desde la pantalla final

## Features

- Ray caster con paredes, piso y objetos del escenario.
- Circuito jugable con colisiones contra paredes y bordes.
- Checkpoints sincronizados con vueltas.
- Meta final sincronizada con la franja de meta.
- Minimap en esquina con posicion del jugador y objetivo activo.
- FPS visibles en pantalla.
- Menu de inicio funcional con seleccion de color del kart.
- Pantalla de pausa.
- Pantalla final con tiempos y opcion para jugar de nuevo.
- Boost pads con feedback visual.
- Derrape con humo y marcas de llanta.
- Decoracion de pista: gradas, senales, conos, barreras y fuente central.

## Pendiente

- Agregar musica y efectos de sonido.
- Agregar un segundo circuito seleccionable.
