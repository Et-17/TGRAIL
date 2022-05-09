const invoke = window.__TAURI__.invoke;

var drawing = true;
var should_draw = false;
var pointsx = [],
    pointsy = [];
// The minimum distance required for points to be logged
var min_dist = 20;

function onMouseMove(ev, ctx) {
    if (!should_draw) return;

    var x = ev.layerX,
        y = ev.layerY,
        d_x = pointsx[pointsx.length - 1] - x,
        d_y = pointsy[pointsy.length - 1] - y;

    if (!drawing) {
        drawing = true;
        ctx.moveTo(x, y);
    } else {
        ctx.lineTo(x, y);
        ctx.stroke();
    }

    if (Math.sqrt((d_x * d_x) + (d_y * d_y)) <= min_dist) return;
    pointsx.push(x);
    pointsy.push(y);
}

function onMouseUp(ev, ctx) {
    should_draw = false;
    ctx.closePath();
    if (pointsx.length <= 5) return;
    invoke("log_points", {
        xPts: pointsx,
        yPts: pointsy
    });
}

function onMouseDown(ev, ctx) {
    should_draw = true;
    ctx.beginPath();
    pointsx = [];
    pointsy = []
}

// Initialize
if (window.addEventListener) {
    window.addEventListener('load', () => {
        // Connect to canvas element
        var canvas = document.getElementById("c");
        var ctx = canvas.getContext("2d");

        canvas.addEventListener('mousedown', e =>
            onMouseDown(e, ctx), false);
        canvas.addEventListener('mouseup', e =>
            onMouseUp(e, ctx), false);
        canvas.addEventListener('mousemove', e =>
            onMouseMove(e, ctx), false);
    });
} else {
    alert("error while hooking to onload");
}