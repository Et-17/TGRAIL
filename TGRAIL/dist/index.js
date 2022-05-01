var drawing = true;
var should_draw = false;

function onMouseMove(ev, ctx) {
    if (!should_draw) return;

    var x = ev.layerX,
        y = ev.layerY;

    if (!drawing) {
        drawing = true;
        ctx.moveTo(x, y);
    } else {
        ctx.lineTo(x, y);
        ctx.stroke();
    }
}

function onMouseUp(ev, ctx) {
    should_draw = false;
    ctx.endPath();
}

function onMouseDown(ev, ctx) {
    should_draw = true;
    ctx.beginPath();
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