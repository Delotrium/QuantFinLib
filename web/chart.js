let initialized = false;

// Fetch surface JSON from Rust backend with query params
async function fetchSurface(url, params) {
    const query = new URLSearchParams(params).toString();
    const r = await fetch(`${url}?${query}`);
    return await r.json();
}

// Read slider values
function getParams() {
    const s = parseFloat(document.getElementById("sValue").value);
    const k = parseFloat(document.getElementById("kValue").value);
    const r = parseFloat(document.getElementById("rValue").value);
    const q = parseFloat(document.getElementById("qValue").value);
    return { s, k, r, q };
}

// Build base layout once
function makeLayout() {
    return {
        title: {
            text: "Black–Scholes Call & Put Surfaces",
            font: { color: "#e5e7eb" },
        },
        paper_bgcolor: "#020617",
        plot_bgcolor: "#020617",
        width: window.innerWidth,
        height: window.innerHeight - 60,
        font: { color: "#e5e7eb" },

        scene: {
            domain: { x: [0.0, 0.48], y: [0, 1] },
            xaxis: {
                title: "T (years)",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
            yaxis: {
                title: "σ",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
            zaxis: {
                title: "Call Price",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
        },

        scene2: {
            domain: { x: [0.52, 1.0], y: [0, 1] },
            xaxis: {
                title: "T (years)",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
            yaxis: {
                title: "σ",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
            zaxis: {
                title: "Put Price",
                backgroundcolor: "#020617",
                gridcolor: "#1f2933",
                color: "#e5e7eb",
            },
        },
    };
}

async function drawSurfaces() {
    const params = getParams();

    const [callData, putData] = await Promise.all([
        fetchSurface("/api/call_surface", params),
        fetchSurface("/api/put_surface", params),
    ]);

    // Shared colour range for both surfaces
    const allZ = callData.z.flat().concat(putData.z.flat());
    const zMin = Math.min(...allZ);
    const zMax = Math.max(...allZ);

    if (!initialized) {
        // First time: create full plot with layout
        const traceCall = {
            type: "surface",
            x: callData.t,
            y: callData.sigma,
            z: callData.z,
            name: "Call Price",
            scene: "scene",
            showscale: true,
            colorbar: {
                title: "Price",
                x: 0.46,
                len: 0.8,
                thickness: 10,
                tickfont: { color: "#e5e7eb" },
                outlinewidth: 0,
                tickformat: ".2f",
                nticks: 7,
            },
            cmin: zMin,
            cmax: zMax,
        };

        const tracePut = {
            type: "surface",
            x: putData.t,
            y: putData.sigma,
            z: putData.z,
            name: "Put Price",
            scene: "scene2",
            showscale: true,
            colorbar: {
                title: "Price",
                x: 1.02,
                len: 0.8,
                thickness: 10,
                tickfont: { color: "#e5e7eb" },
                outlinewidth: 0,
                tickformat: ".2f",
                nticks: 7,
            },
            cmin: zMin,
            cmax: zMax,
        };

        const layout = makeLayout();

        await Plotly.newPlot("plots", [traceCall, tracePut], layout);
        initialized = true;
    } else {
        // Update only data + colour scale, keep layout & camera
        // trace 0 = call, trace 1 = put
        const updateData = {
            z: [callData.z, putData.z],
            cmin: [zMin, zMin],
            cmax: [zMax, zMax],
        };

        await Plotly.update("plots", updateData, {}, [0, 1]);
        // axes & camera untouched => no jump
    }
}

// Link sliders to redraw
function linkRangeAndNumber(rangeId, numberId) {
    const range = document.getElementById(rangeId);
    const number = document.getElementById(numberId);

    range.addEventListener("input", () => {
        number.value = range.value;
        drawSurfaces();
    });

    number.addEventListener("change", () => {
        range.value = number.value;
        drawSurfaces();
    });
}

linkRangeAndNumber("sRange", "sValue");
linkRangeAndNumber("kRange", "kValue");
linkRangeAndNumber("rRange", "rValue");
linkRangeAndNumber("qRange", "qValue");

// Initial render
drawSurfaces();

// Optional: on window resize, just resize layout (doesn't touch camera)
window.addEventListener("resize", () => {
    const newLayout = {
        width: window.innerWidth,
        height: window.innerHeight - 60,
    };
    Plotly.relayout("plots", newLayout);
});
