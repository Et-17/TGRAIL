const invoke = window.__TAURI__.invoke;

var drawing = true;
var should_draw = false;
var pointsx = [],
    pointsy = [];
var ctx;

function onMouseMove(ev) {
    if (!should_draw) return;

    var x = ev.layerX,
        y = ev.layerY,
        d_x = pointsx[pointsx.length - 1] - x,
        d_y = pointsy[pointsy.length - 1] - y;

    if (Math.sqrt((d_x * d_x) + (d_y * d_y)) <= 5) return;

    if (!drawing) {
        drawing = true;
        ctx.moveTo(x, y);
        ctx.beginPath();
    } else {
        ctx.lineTo(x, y);
        ctx.stroke();
    }

    pointsx.push(x);
    pointsy.push(y);
}

function onMouseUp(ev) {
    should_draw = false;
    if (pointsx.length <= 5) return;
    invoke("log_points", {
        xPts: pointsx,
        yPts: pointsy
    }).then(x => {
        console.log(x);
        for (var i = 0; i < x.length; i++) {
            ctx.rect(x[i].x, x[i].y, 10, 10);
        }
        ctx.stroke();
    });
}

function onMouseDown(ev) {
    should_draw = true;
    drawing = false;
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    pointsx = [];
    pointsy = []
}

// Initialize
if (window.addEventListener) {
    window.addEventListener('load', () => {
        // Connect to canvas element
        var canvas = document.getElementById("c");
        ctx = canvas.getContext("2d");

        canvas.addEventListener('mousedown', e =>
            onMouseDown(e), false);
        canvas.addEventListener('mouseup', e =>
            onMouseUp(e), false);
        canvas.addEventListener('mousemove', e =>
            onMouseMove(e), false);
    });
} else {
    alert("error while hooking to onload");
}