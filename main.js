import init, { onTick } from "./pkg/rust_particles.js";

await init();

function frame(time) {

  window.requestAnimationFrame(nextTime => {
    const tick = nextTime - time;
    onTick(tick / 1000);
    frame(tick);
  });

}

frame(0);
