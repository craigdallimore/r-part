import init from "./pkg/rust_particles.js";

await init();

//console.log(onTick(10));
/*
function frame(time) {

  window.requestAnimationFrame(nextTime => {
    const tick = nextTime - time;
    onTick(tick / 1000);
    frame(tick);
  });

}

frame(0);
*/
