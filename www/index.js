import init from '../pkg/voronoi.js';

init().then(() => console.log("Initialized"))

// import { redraw } from '../pkg/voronoi.js';
// init().then(run)
// class Seed {
//     constructor(x, y) {
//         this.x = x;
//         this.y = y;
//         this.c = Math.floor((Math.random() * 0xFFFFFF));
//         return this;
//     }
//     toString() {
//         return `{ x: ${this.x}, y: ${this.y}, c: ${this.c} }`
//     }
// }
// function run() {
//     const canvas = document.createElement('canvas');
//     const ctx = canvas.getContext('webgl');
//     if (!ctx) {
//         console.log("Failed to get a WebGL context for the canvas!");
//         return;
//     }
//     document.body.append(canvas);
//     canvas.width = window.innerWidth;
//     canvas.height = window.innerHeight;

//     const seeds = [new Seed(10, 26), new Seed(100, 67)];

//     const json_seeds = JSON.stringify(seeds);

//     window.addEventListener('resize', () => {
//         canvas.width = window.innerWidth;
//         canvas.height = window.innerHeight;
//         const json_seeds = JSON.stringify(seeds);
//         draw(ctx, canvas.width, canvas.height, json_seeds);
//     })

//     document.onmousedown = () => {
//         seeds.push(new Seed(window.event.clientX, window.event.clientY));
//         const json_seeds = JSON.stringify(seeds);
//         draw(ctx, canvas.width, canvas.height, json_seeds);
//     };

//     setInterval(() => {
//         for (let item of seeds) {
//             item.x += (randn_bm() * 3).floor();
//             item.y += (randn_bm() * 3).floor();
//             item.x = item.x.clamp(0, canvas.width);
//             item.y = item.y.clamp(0, canvas.height);
//         }
//         const json_seeds = JSON.stringify(seeds);
//         draw(ctx, canvas.width, canvas.height, json_seeds);
//     }, 100);

//     draw(ctx, canvas.width, canvas.height, json_seeds);
// }

function randn_bm() {
    let u = 1 - Math.random(); //Converting [0,1) to (0,1)
    let v = Math.random();
    return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
}

Number.prototype.floor = function () {
    return Math.floor(this);
}

Number.prototype.clamp = function (a, b) {
    return Math.min(Math.max(this, a), b);
}
