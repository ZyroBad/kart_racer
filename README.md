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
- `P`: pausar o continuar
- `Enter` / `R`: aceptar, continuar o reiniciar segun la pantalla
- `Backspace`: volver al menu desde la pantalla final
- En pausa: `W/S` elige opcion, `Enter` acepta, `Backspace` vuelve al menu
- En seleccion de pista: `W/S` cambia entre pista y vehiculo, `A/D` cambia la opcion

## Features

- Ray caster con paredes, piso y objetos del escenario.
- Circuito jugable con colisiones contra paredes y bordes.
- Checkpoints sincronizados con vueltas.
- Meta final sincronizada con la franja de meta.
- Minimap en esquina con posicion del jugador y objetivo activo.
- FPS visibles en pantalla.
- Menu de inicio funcional con imagen de fondo, controles y salida.
- Menu de pausa con opcion para activar/desactivar musica.
- Pantalla separada para seleccionar pista y vehiculo antes de iniciar la carrera.
- Pantalla de pausa.
- Menu de pausa con opcion para continuar o volver al menu principal.
- Pantalla final con tiempos y opcion para jugar de nuevo.
- Boost pads con feedback visual.
- Derrape con humo y marcas de llanta.
- Dos circuitos: Jardin Rust y Gran Premio Metro nocturno.
- Vehiculos seleccionables: kart y moto.
- Decoracion de pista: gradas, edificios con ventanas, luces urbanas, semaforos, rotulos, senales, conos, barreras y fuente central.
- Guia de direccion hacia el siguiente checkpoint para evitar perderse en pistas grandes.
- Soporte de audio opcional para musica y efectos desde `assets/audio`.

## Criterios cubiertos

- FPS visibles durante la carrera.
- Camara con movimiento hacia delante, atras, giro y rotacion horizontal con mouse.
- Minimap en esquina con posicion del jugador y checkpoint activo.
- Musica de fondo con opcion para activar/desactivar.
- Efectos de sonido para motor, boost y checkpoints.
- Pantalla de bienvenida/menu principal.
- Seleccion de multiples niveles.
- Seleccion de vehiculo entre kart y moto.
- Pantalla de pausa con opcion para continuar, volver al menu y controlar musica/efectos.
- Pantalla de exito al completar la carrera.
- Paredes, suelos y objetos con colores distintos segun el tipo de tile.

## Checklist de entrega

- Probar una carrera completa en `Jardin Rust`.
- Probar una carrera completa en `Gran Premio Metro`.
- Confirmar que el jugador no atraviesa paredes ni objetos solidos.
- Confirmar que `P` pausa y que se puede volver al menu.
- Confirmar que `Musica ON/OFF` reinicia la cancion al volver a encenderla.
- Confirmar que los efectos de motor, boost y checkpoint suenan durante carrera.
- Grabar video mostrando menu, seleccion de pista, seleccion de vehiculo, carrera, minimap, FPS, pausa, audio y pantalla final.

## Pendiente opcional

- Convertir `besame_mucho.wav` a `.ogg` para reducir el peso del repositorio.
- Ajustar volumenes finales segun se escuche en el video de entrega.

## Audio

El juego busca musica en `assets/audio/music/besame_mucho.wav`, `besame_mucho.ogg` y `vuelve.ogg`, y efectos en `assets/audio/sfx/engine.wav`, `boost.wav` y `checkpoint.wav`.

No se incluyen grabaciones comerciales en el repositorio. Si se usan canciones como versiones de Luis Miguel, deben agregarse archivos con permiso o licencia valida.
